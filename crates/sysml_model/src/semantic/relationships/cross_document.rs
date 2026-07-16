use super::*;

/// Finds a PartDef in the root by qualified name by walking PackageBodyElements.
pub fn find_part_def_in_root<'a>(
    root: &'a RootNamespace,
    qualified: &str,
) -> Option<(&'a sysml_v2_parser::Node<sysml_v2_parser::PartDef>, String)> {
    let mut prefix = String::new();
    for node in &root.elements {
        let elements = match root_element_body(&node.value) {
            Some((elements, _, _, _)) => elements,
            None => continue,
        };
        if let Some(found) = find_part_def_in_elements(elements, &mut prefix, qualified) {
            return Some(found);
        }
    }
    None
}

pub fn find_part_def_in_elements<'a>(
    elements: &'a [sysml_v2_parser::Node<PackageBodyElement>],
    prefix: &mut String,
    target: &str,
) -> Option<(&'a sysml_v2_parser::Node<sysml_v2_parser::PartDef>, String)> {
    for node in elements {
        match &node.value {
            PackageBodyElement::Package(pkg) => {
                let name = identification_name(&pkg.identification);
                let prev = std::mem::take(prefix);
                *prefix = if prev.is_empty() {
                    name.clone()
                } else {
                    format!("{}::{}", prev, name)
                };
                if let PackageBody::Brace { elements: inner } = &pkg.body {
                    if let Some(found) = find_part_def_in_elements(inner, prefix, target) {
                        return Some(found);
                    }
                }
                *prefix = prev;
            }
            PackageBodyElement::PartDef(pd) => {
                let name = identification_name(&pd.identification);
                let q = if prefix.is_empty() {
                    name.clone()
                } else {
                    format!("{}::{}", prefix, name)
                };
                if q == target {
                    return Some((pd, q));
                }
            }
            _ => {}
        }
    }
    None
}

/// URI-scoped resolver for workspace relationship linking. Adds typing/specializes/subject
/// edges from nodes in the given URI, and maintains `cross_document_edges_by_source_uri` (see
/// `graph.rs`) so it can cleanly remove and re-add its own edges on a later call.
///
/// Self-cleaning and idempotent: any cross-document edges this function previously added for
/// `uri` are removed first (via `remove_recorded_cross_document_edges_for_uri`), so calling it
/// again for the same URI — e.g. as part of a relationship frontier where `uri`'s own nodes
/// were *not* removed and re-added — produces a correct, non-duplicated result rather than
/// accumulating stale edges alongside fresh ones.
///
/// Does NOT touch `document_dependency_targets`/`document_dependents` — those are maintained
/// separately by `update_static_dependency_targets_for_uri`, called only when `uri`'s own
/// content actually changed (this function also runs for frontier URIs whose content didn't
/// change, just their outgoing edges being defensively re-checked).
///
/// Used by the Spec42 kernel incremental update path and `refresh_relationship_frontier`.
pub fn add_cross_document_edges_for_uri(g: &mut SemanticGraph, uri: &Url) {
    g.remove_recorded_cross_document_edges_for_uri(uri);
    let edges = resolve_cross_document_edges_for_uri(g, uri);
    let mut recorded = Vec::with_capacity(edges.len());
    for (src_id, tgt_id, kind) in edges {
        if let (Some(&src_idx), Some(&tgt_idx)) = (
            g.node_index_by_id.get(&src_id),
            g.node_index_by_id.get(&tgt_id),
        ) {
            g.graph
                .add_edge(src_idx, tgt_idx, SemanticEdge::plain(kind.clone()));
            recorded.push((src_id, tgt_id, kind));
        }
    }
    if !recorded.is_empty() {
        g.cross_document_edges_by_source_uri
            .insert(uri.clone(), recorded);
    }
}

/// Re-resolves cross-document Typing/Specializes/Subject edges for `changed_uri` and every
/// other URI recorded in `g.document_dependents[changed_uri]` — i.e. every document whose own
/// parsed content (imports or qualified references) statically depends on `changed_uri` —
/// instead of the whole-graph `link_workspace_relationships`.
///
/// `resolve_cross_document_edges_for_uri` (used by `add_cross_document_edges_for_uri`) only
/// ever produces `Typing`/`Specializes`/`Subject` edges — including the `VerifiedRequirement`
/// case, which also resolves to `RelationshipKind::Subject` — so re-running it per frontier
/// URI fully covers what the whole-graph `link_case_subject_relationships` pass would refresh
/// too; no separate scoped call to that function is needed here.
///
/// Derivation-connection rewiring is deliberately **not** scoped: `resolve_cross_document_edges_for_uri`
/// doesn't cover derivation connections at all (they're wired by a separate mechanism keyed on
/// the connection's own declaring URI, independent of what it references), so there is no
/// reverse-dependency data to scope by. That pass stays whole-graph — see the Track B Phase 1
/// plan for why forcing it into scope here would risk a silent correctness gap.
pub fn refresh_relationship_frontier(g: &mut SemanticGraph, changed_uri: &Url) {
    let mut frontier: std::collections::HashSet<Url> = g
        .document_dependents
        .get(changed_uri)
        .cloned()
        .unwrap_or_default();
    frontier.insert(changed_uri.clone());
    for uri in &frontier {
        add_cross_document_edges_for_uri(g, uri);
    }

    // Derivation connections stay a whole-graph pass — see doc comment above.
    let connection_ids: Vec<NodeId> = g
        .node_index_by_id
        .keys()
        .filter(|node_id| {
            g.get_node(node_id)
                .map(|node| node.element_kind == ElementKind::DerivationConnection)
                .unwrap_or(false)
        })
        .cloned()
        .collect();
    for connection_id in connection_ids {
        try_wire_derivation_connection(g, &connection_id.uri, &connection_id);
    }
}

/// Computes the set of URIs that `uri`'s own nodes statically depend on: the declaring URI of
/// every import target (explicit or wildcard, via each `ElementKind::Import` child's
/// `importTarget` attribute) plus every `::`-qualified prefix appearing directly in a
/// Typing/Specializes/Subject/VerifiedRequirement-bearing attribute anywhere in `uri`.
///
/// Purely a function of `uri`'s own nodes in `g` — never inspects any other URI's content.
/// Package-name prefixes are turned into URIs via `node_ids_for_qualified_name`, which only
/// requires the target namespace to currently exist, not that any specific reference resolves
/// against it — this is what makes the resulting set safe to use as a stable frontier source
/// (see `update_static_dependency_targets_for_uri`'s doc comment for why that distinction
/// matters).
///
/// Deliberately over-approximates for wildcard imports: a bare unqualified name used after
/// `import A::*` could textually come from any wildcard-imported namespace in scope, so every
/// such namespace's URI is included as a candidate dependency, even if only one of them
/// actually supplies any given name. Over-inclusion just means the frontier occasionally does
/// slightly more work than strictly necessary; it is never incorrect.
pub fn compute_static_dependency_targets(
    g: &SemanticGraphData,
    uri: &Url,
) -> std::collections::HashSet<Url> {
    let mut prefixes: std::collections::HashSet<String> = std::collections::HashSet::new();
    let Some(node_ids) = g.nodes_by_uri.get(uri) else {
        return std::collections::HashSet::new();
    };
    for node_id in node_ids {
        let Some(node) = g.get_node(node_id) else {
            continue;
        };
        if node.element_kind == ElementKind::Import {
            if let Some(target) = import_target(node) {
                let normalized = if is_import_all(node) {
                    normalized_namespace_target(target)
                } else {
                    normalized_membership_target(target)
                };
                let prefix = match normalized.rsplit_once("::") {
                    Some((prefix, _member)) => prefix.to_string(),
                    None => normalized,
                };
                if !prefix.is_empty() {
                    prefixes.insert(prefix);
                }
            }
            continue;
        }
        for key in TYPE_REFERENCE_ATTR_KEYS.iter().copied().chain([
            "specializes",
            "subjectType",
            "verifiedRequirement",
        ]) {
            let Some(raw) = node.attributes.get(key).and_then(|value| value.as_str()) else {
                continue;
            };
            if let Some((prefix, _member)) = raw.rsplit_once("::") {
                if !prefix.is_empty() {
                    prefixes.insert(prefix.to_string());
                }
            }
        }
    }

    let mut targets = std::collections::HashSet::new();
    for prefix in prefixes {
        let key = normalize_for_lookup(&prefix);
        if let Some(target_ids) = g.node_ids_for_qualified_name(&key) {
            for target_id in target_ids {
                if &target_id.uri != uri {
                    targets.insert(target_id.uri.clone());
                }
            }
        }
    }
    targets
}

/// Recomputes `document_dependency_targets[uri]` from `uri`'s current content and updates
/// `document_dependents` accordingly (diffing against the prior set: removing `uri` from
/// targets it no longer depends on, adding it to newly-depended-on targets).
///
/// **Call this only when `uri`'s own content changed** — from `patch_graph_for_document`/
/// `patch_graph_for_document_scoped` right after merging the fresh subgraph, or once per URI
/// during a full build. Do NOT call this for a URI whose content is untouched, even if its
/// outgoing edges are being defensively re-checked as part of a relationship frontier
/// (`add_cross_document_edges_for_uri` on a frontier URI) — that URI's static dependencies
/// haven't changed, so recomputing here would be wasted work, and conflating "refresh this
/// URI's edges" with "this URI's content changed" was the exact bug this design replaced (see
/// the Track B Phase 1 plan's design-gap note): a resolved-edge cache could silently shrink
/// when a frontier URI's re-resolution failed, even though nothing about that URI's own
/// content changed. Static dependency targets don't have this problem *as long as they're only
/// ever recomputed in response to an actual content change* — recomputing them from a
/// defensive re-check would reintroduce the same failure mode by a different path.
pub fn update_static_dependency_targets_for_uri(g: &mut SemanticGraphData, uri: &Url) {
    let new_targets = compute_static_dependency_targets(g, uri);
    let old_targets = g
        .document_dependency_targets
        .get(uri)
        .cloned()
        .unwrap_or_default();

    for old_target in old_targets.difference(&new_targets) {
        let mut remove_entry = false;
        if let Some(dependents) = g.document_dependents.get_mut(old_target) {
            dependents.remove(uri);
            remove_entry = dependents.is_empty();
        }
        if remove_entry {
            g.document_dependents.remove(old_target);
        }
    }
    for new_target in new_targets.difference(&old_targets) {
        g.document_dependents
            .entry(new_target.clone())
            .or_default()
            .insert(uri.clone());
    }

    if new_targets.is_empty() {
        g.document_dependency_targets.remove(uri);
    } else {
        g.document_dependency_targets
            .insert(uri.clone(), new_targets);
    }
}

/// Recomputes `document_dependency_targets`/`document_dependents` from scratch for every URI
/// in the graph. O(all nodes' attributes), the same order of cost as the whole-graph relink it
/// supports. Call after a full build, or after deserialization (via `rebuild_derived_indexes`).
pub fn rebuild_static_dependency_index(g: &mut SemanticGraphData) {
    g.document_dependency_targets.clear();
    g.document_dependents.clear();
    let uris: Vec<Url> = g.nodes_by_uri.keys().cloned().collect();
    for uri in uris {
        let targets = compute_static_dependency_targets(g, &uri);
        if !targets.is_empty() {
            for target in &targets {
                g.document_dependents
                    .entry(target.clone())
                    .or_default()
                    .insert(uri.clone());
            }
            g.document_dependency_targets.insert(uri, targets);
        }
    }
}

/// Legacy URI-scoped resolver for workspace relationship linking.
/// Returns resolved cross-document edges for one URI.
///
/// Used by the Spec42 kernel incremental update path (`add_cross_document_edges_for_uri`).
/// Full workspace builds should use [`crate::semantic::pipeline::build_and_link_graph`] instead.
pub fn resolve_cross_document_edges_for_uri(
    g: &SemanticGraph,
    uri: &Url,
) -> Vec<(NodeId, NodeId, RelationshipKind)> {
    const CASE_KINDS: &[ElementKind] = &[
        ElementKind::AnalysisDef,
        ElementKind::Analysis,
        ElementKind::VerificationDef,
        ElementKind::Verification,
        ElementKind::UseCaseDef,
        ElementKind::UseCase,
        ElementKind::ConcernDef,
        ElementKind::Concern,
        ElementKind::RequirementDef,
        ElementKind::Requirement,
    ];
    let node_ids: Vec<NodeId> = g.nodes_by_uri.get(uri).cloned().unwrap_or_default();
    let mut resolved_edges = Vec::new();
    let mut seen_edges = std::collections::HashSet::new();

    for node_id in &node_ids {
        let Some(node) = g.get_node(node_id) else {
            continue;
        };
        let prefix: Option<String> = node
            .parent_id
            .as_ref()
            .and_then(|pid| g.get_node(pid))
            .map(|p| p.id.qualified_name.clone());

        // Typing relationships
        for key in TYPE_REFERENCE_ATTR_KEYS {
            if let Some(type_ref) = node.attributes.get(*key).and_then(|v| v.as_str()) {
                if let Some(target_id) = resolve_typing_edge_cross_document_inner(
                    g,
                    node,
                    type_ref,
                    prefix.as_deref(),
                    RelationshipKind::Typing,
                ) {
                    let dedupe_key = (node_id.clone(), target_id.clone(), "typing");
                    if seen_edges.insert(dedupe_key) {
                        resolved_edges.push((node_id.clone(), target_id, RelationshipKind::Typing));
                    }
                }
            }
        }

        // Specializes relationships
        let specializes_refs = node
            .attributes
            .get("specializes")
            .map(specializes_refs_from_value)
            .unwrap_or_default();
        for specializes_ref in specializes_refs {
            if let Some(target_id) = resolve_typing_edge_cross_document_inner(
                g,
                node,
                &specializes_ref,
                prefix.as_deref(),
                RelationshipKind::Specializes,
            ) {
                let dedupe_key = (node_id.clone(), target_id.clone(), "specializes");
                if seen_edges.insert(dedupe_key) {
                    resolved_edges.push((
                        node_id.clone(),
                        target_id,
                        RelationshipKind::Specializes,
                    ));
                }
            }
        }

        // Subject relationships (case/requirement declarations and usages).
        if CASE_KINDS.contains(&node.element_kind) {
            for subject in g
                .children_of(node)
                .into_iter()
                .filter(|child| child.element_kind == ElementKind::Subject)
            {
                let target_id = g
                    .outgoing_targets_by_kind(subject, RelationshipKind::Typing)
                    .into_iter()
                    .find(|target| {
                        element_kind_allowed(&target.element_kind, SUBJECT_TYPE_TARGET_KINDS)
                    })
                    .map(|target| target.id.clone())
                    .or_else(|| {
                        subject
                            .attributes
                            .get("subjectType")
                            .and_then(|value| value.as_str())
                            .and_then(|type_ref| {
                                resolve_type_reference_targets(
                                    g,
                                    subject,
                                    type_ref,
                                    SUBJECT_TYPE_TARGET_KINDS,
                                )
                                .into_iter()
                                .next()
                            })
                    });
                if let Some(target_id) = target_id {
                    let dedupe_key = (node_id.clone(), target_id.clone(), "subject");
                    if seen_edges.insert(dedupe_key) {
                        resolved_edges.push((
                            node_id.clone(),
                            target_id,
                            RelationshipKind::Subject,
                        ));
                    }
                }
            }

            for verified_requirement in g
                .children_of(node)
                .into_iter()
                .filter(|child| child.element_kind == ElementKind::VerifiedRequirement)
            {
                let Some(requirement_ref) = verified_requirement
                    .attributes
                    .get("verifiedRequirement")
                    .and_then(|value| value.as_str())
                else {
                    continue;
                };
                let Some(target_id) = resolve_type_reference_targets(
                    g,
                    verified_requirement,
                    requirement_ref,
                    VERIFIED_REQUIREMENT_TARGET_KINDS,
                )
                .into_iter()
                .next() else {
                    continue;
                };
                let dedupe_key = (node_id.clone(), target_id.clone(), "subject");
                if seen_edges.insert(dedupe_key) {
                    resolved_edges.push((node_id.clone(), target_id, RelationshipKind::Subject));
                }
            }
        }
    }
    resolved_edges
}

fn resolve_typing_edge_cross_document_inner(
    g: &SemanticGraph,
    src_node: &SemanticNode,
    type_ref: &str,
    container_prefix: Option<&str>,
    kind: RelationshipKind,
) -> Option<NodeId> {
    let normalized_type_ref = normalize_declared_type_ref(type_ref);
    if normalized_type_ref.is_empty() {
        return None;
    }
    let target_element_kinds: &[ElementKind] = match kind {
        RelationshipKind::Typing => TYPING_TARGET_KINDS,
        RelationshipKind::Specializes => SPECIALIZES_TARGET_KINDS,
        _ => return None,
    };

    let mut targets =
        resolve_type_reference_targets(g, src_node, &normalized_type_ref, target_element_kinds);
    if let Some(prefix) = container_prefix {
        for suffix_kind in kinds::DISAMBIGUATION_SUFFIXES {
            for candidate in
                type_ref_candidates_with_kind(Some(prefix), &normalized_type_ref, suffix_kind)
            {
                let tgt_qualified = normalize_for_lookup(&candidate);
                if let Some(target_ids) = g.node_ids_for_qualified_name(&tgt_qualified) {
                    for target_id in target_ids {
                        if let Some(target) = g.get_node(target_id) {
                            if element_kind_allowed(&target.element_kind, target_element_kinds) {
                                targets.push(target_id.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    targets.into_iter().next()
}
