use super::*;

pub(crate) fn add_pending_expression_relationship(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    source_expression: &str,
    target_expression: &str,
    kind: RelationshipKind,
    source_range: crate::semantic::text_span::TextRange,
) {
    let duplicate = g.pending_expression_relationships.iter().any(|pending| {
        pending.uri == *uri
            && pending.kind == kind
            && pending.source_expression == source_expression
            && pending.target_expression == target_expression
            && pending.container_prefix.as_deref() == container_prefix
    });
    if duplicate {
        return;
    }
    g.pending_expression_relationships
        .push(PendingExpressionRelationship {
            uri: uri.clone(),
            source_expression: source_expression.to_string(),
            target_expression: target_expression.to_string(),
            kind,
            container_prefix: container_prefix.map(ToString::to_string),
            source_range,
        });
}

/// Re-resolve pending relationship queues for every document in the merged graph.
///
/// Must run after [`link_workspace_relationships`] so typing edges needed for member-chain
/// connection endpoints are available.
pub fn resolve_workspace_pending_relationships(g: &mut SemanticGraph) {
    const MAX_PASSES: usize = 8;
    for _ in 0..MAX_PASSES {
        let pending_before =
            g.pending_relationships.len() + g.pending_expression_relationships.len();
        if pending_before == 0 {
            break;
        }
        let uris: Vec<Url> = g.nodes_by_uri.keys().cloned().collect();
        for uri in uris {
            resolve_pending_relationships_for_uri(g, &uri);
        }
        let pending_after =
            g.pending_relationships.len() + g.pending_expression_relationships.len();
        if pending_after == pending_before {
            break;
        }
    }
}

pub fn resolve_pending_relationships_for_uri(g: &mut SemanticGraph, uri: &Url) {
    resolve_pending_expression_relationships_for_uri(g, uri);

    let pending = std::mem::take(&mut g.pending_relationships);
    for pending_edge in pending {
        if &pending_edge.uri != uri {
            g.pending_relationships.push(pending_edge);
            continue;
        }
        let source_id = NodeId::new(uri, &pending_edge.source_qualified);
        let mut target_id = NodeId::new(uri, &pending_edge.target_qualified);
        if !g.node_index_by_id.contains_key(&target_id) {
            if let Some(ids) = g.node_ids_for_qualified_name(&pending_edge.target_qualified) {
                if ids.len() == 1 {
                    target_id = ids[0].clone();
                }
            }
        }
        if !g.node_index_by_id.contains_key(&target_id) {
            if let Some(source_node) = g.get_node(&source_id) {
                let resolved = if let Some(ref target_kinds) = pending_edge.target_kinds {
                    if target_kinds.is_empty() {
                        Vec::new()
                    } else {
                        resolve_pending_target(
                            g,
                            source_node,
                            &pending_edge.target_qualified,
                            target_kinds,
                        )
                    }
                } else {
                    match pending_edge.kind {
                        RelationshipKind::Typing => resolve_pending_target(
                            g,
                            source_node,
                            &pending_edge.target_qualified,
                            TYPING_TARGET_KINDS,
                        ),
                        RelationshipKind::Specializes => resolve_pending_target(
                            g,
                            source_node,
                            &pending_edge.target_qualified,
                            SPECIALIZES_TARGET_KINDS,
                        ),
                        RelationshipKind::Subject => resolve_pending_target(
                            g,
                            source_node,
                            &pending_edge.target_qualified,
                            VERIFIED_REQUIREMENT_TARGET_KINDS,
                        ),
                        _ => Vec::new(),
                    }
                };
                if resolved.len() == 1 {
                    target_id = resolved[0].clone();
                }
            }
        }
        let (Some(_), Some(tgt_node), Some(_)) = (
            g.node_index_by_id.get(&source_id),
            g.get_node(&target_id),
            g.node_index_by_id.get(&target_id),
        ) else {
            g.pending_relationships.push(pending_edge);
            continue;
        };
        if let Some(ref target_kinds) = pending_edge.target_kinds {
            if !target_kinds.contains(&tgt_node.element_kind) {
                continue;
            }
        }
        add_semantic_edge_once(
            g,
            &source_id,
            &target_id,
            SemanticEdge::plain(pending_edge.kind.clone()),
        );
    }
}

fn resolve_pending_expression_endpoint(
    g: &SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    expression: &str,
) -> ResolveResult<NodeId> {
    match resolve_expression_endpoint_strict(g, uri, container_prefix, expression) {
        ResolveResult::Resolved(id) => return ResolveResult::Resolved(id),
        ResolveResult::Ambiguous => return ResolveResult::Ambiguous,
        ResolveResult::Unresolved => {}
    }

    match crate::semantic::reference_resolution::resolve_expression_endpoint_workspace(
        g, expression,
    ) {
        ResolveResult::Resolved(id) => return ResolveResult::Resolved(id),
        ResolveResult::Ambiguous => return ResolveResult::Ambiguous,
        ResolveResult::Unresolved => {}
    }

    crate::semantic::reference_resolution::resolve_workspace_member_chain(g, expression)
}

fn resolve_pending_expression_relationships_for_uri(g: &mut SemanticGraph, uri: &Url) {
    let pending = std::mem::take(&mut g.pending_expression_relationships);
    for pending_edge in pending {
        if &pending_edge.uri != uri {
            g.pending_expression_relationships.push(pending_edge);
            continue;
        }
        let source_id = match resolve_pending_expression_endpoint(
            g,
            uri,
            pending_edge.container_prefix.as_deref(),
            &pending_edge.source_expression,
        ) {
            ResolveResult::Resolved(id) => id,
            ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                g.pending_expression_relationships.push(pending_edge);
                continue;
            }
        };
        let target_id = match resolve_pending_expression_endpoint(
            g,
            uri,
            pending_edge.container_prefix.as_deref(),
            &pending_edge.target_expression,
        ) {
            ResolveResult::Resolved(id) => id,
            ResolveResult::Ambiguous | ResolveResult::Unresolved => {
                g.pending_expression_relationships.push(pending_edge);
                continue;
            }
        };
        if pending_edge.kind == RelationshipKind::Connection {
            add_semantic_edge_once(
                g,
                &source_id,
                &target_id,
                SemanticEdge::connection_with_connect(ConnectStatementDetail {
                    declaring_uri: uri.clone(),
                    range: pending_edge.source_range,
                    source_expression: pending_edge.source_expression,
                    target_expression: pending_edge.target_expression,
                    container_prefix: pending_edge.container_prefix.clone(),
                }),
            );
        } else {
            add_semantic_edge_once(
                g,
                &source_id,
                &target_id,
                SemanticEdge::plain(pending_edge.kind.clone()),
            );
        }
    }
}
