use super::*;

pub fn evaluate_views(
    catalog: &ViewCatalog,
    semantic_graph: &crate::semantic::graph::SemanticGraph,
    graph: &crate::semantic::dto::SysmlGraphDto,
) -> Vec<EvaluatedView> {
    let node_by_id: HashMap<&str, &crate::semantic::dto::GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let parent_by_id: HashMap<&str, &str> = graph
        .nodes
        .iter()
        .filter_map(|node| {
            node.parent_id
                .as_deref()
                .map(|parent| (node.id.as_str(), parent))
        })
        .collect();
    catalog
        .usages
        .iter()
        .map(|usage| {
            let mut issues = usage.issues.clone();
            let usage_filters = usage.filters.clone();
            let mut filters = usage_filters.clone();
            let mut conforms_to = usage.conforms_to.clone();
            let effective_view_type = Some(resolve_effective_view_type(usage, catalog));
            if usage.definition_id.is_none() {
                if let Some(type_ref) = usage.definition_ref.as_deref() {
                    if crate::semantic::standard_views::is_non_standard_explicit_view_type(type_ref)
                    {
                        issues.push(format!(
                            "View type '{type_ref}' is not a SysML v2 standard view definition (§9.2.20 Table 34); use GeneralView with filters, a render clause, or a local view def."
                        ));
                    }
                }
            }
            if let Some(definition_id) = usage.definition_id.as_deref() {
                if let Some(definition) = catalog.definitions.get(definition_id) {
                    filters.extend(definition.filters.clone());
                }
            }
            if usage_filters.is_empty() {
                if let Some(view_type) = effective_view_type.as_deref() {
                    filters.extend(
                        crate::semantic::standard_view_defaults::merge_usage_default_filters(
                            view_type,
                            &[],
                            Some(semantic_graph),
                        ),
                    );
                }
            }
            for expose in &usage.exposes {
                if let Some(filter) = &expose.filter {
                    filters.push(filter.clone());
                }
            }

            let view_uri = uri_for_qualified_name(semantic_graph, &usage.id)
                .or_else(|| {
                    node_by_id
                        .get(usage.id.as_str())
                        .and_then(|node| node.uri.clone())
                })
                .and_then(|uri| url::Url::parse(&uri).ok());
            let container_prefix = usage.id.rsplit_once("::").map(|(prefix, _)| prefix);

            let mut exposed_ids = HashSet::new();
            for expose in &usage.exposes {
                match crate::semantic::reference_resolution::resolve_expose_target(
                    semantic_graph,
                    view_uri.as_ref(),
                    container_prefix,
                    &expose.target,
                ) {
                    crate::semantic::reference_resolution::ExposeTargetResolution::Resolved(
                        names,
                    ) => {
                        for node_id in names {
                            if node_matches_expose_filter(
                                node_id.as_str(),
                                &node_by_id,
                                expose.filter.as_ref(),
                            ) {
                                exposed_ids.insert(node_id);
                            }
                        }
                    }
                    crate::semantic::reference_resolution::ExposeTargetResolution::Ambiguous => {
                        issues.push(format!("Expose target '{}' is ambiguous.", expose.target));
                    }
                    crate::semantic::reference_resolution::ExposeTargetResolution::Unresolved => {
                        issues.push(format!(
                            "Expose target '{}' does not resolve to any element.",
                            expose.target
                        ));
                    }
                }
            }

            if usage.exposes.is_empty() {
                issues.push("View has no expose members.".to_string());
            }
            if let Some(view_node) = node_by_id.get(usage.id.as_str()) {
                for edge in &graph.edges {
                    if edge.rel_type != "satisfy" || edge.source != view_node.id {
                        continue;
                    }
                    let Some(target) = node_by_id.get(edge.target.as_str()) else {
                        continue;
                    };
                    let target_kind = target.element_type.as_str();
                    if target_kind == "viewpoint" || target_kind == "viewpoint def" {
                        conforms_to.push(target.id.clone());
                    }
                }
            }
            conforms_to.sort();
            conforms_to.dedup();

            let filtered_ids: HashSet<String> = exposed_ids
                .iter()
                .filter(|node_id| node_matches_all_filters(node_id, &node_by_id, &filters))
                .cloned()
                .collect();
            let closure = with_ancestors(filtered_ids, &parent_by_id);
            EvaluatedView {
                id: usage.id.clone(),
                name: usage.name.clone(),
                effective_view_type,
                exposed_ids,
                conforms_to,
                filters,
                visible_ids: closure,
                issues,
            }
        })
        .collect()
}

pub fn project_ids_for_renderer(
    evaluated: &EvaluatedView,
    graph: &crate::semantic::dto::SysmlGraphDto,
    _renderer_view: &str,
) -> HashSet<String> {
    crate::semantic::view_projection::project_ids_for_renderer(evaluated, graph)
}

pub(crate) fn uri_for_qualified_name(
    semantic_graph: &crate::semantic::graph::SemanticGraph,
    qualified_name: &str,
) -> Option<String> {
    semantic_graph
        .graph
        .node_weights()
        .find(|node| node.id.qualified_name == qualified_name)
        .map(|node| node.id.uri.as_str().to_string())
}

pub(crate) fn with_ancestors(
    mut visible_ids: HashSet<String>,
    parent_by_id: &HashMap<&str, &str>,
) -> HashSet<String> {
    let mut stack: Vec<String> = visible_ids.iter().cloned().collect();
    while let Some(current) = stack.pop() {
        if let Some(parent) = parent_by_id.get(current.as_str()) {
            let parent_string = (*parent).to_string();
            if visible_ids.insert(parent_string.clone()) {
                stack.push(parent_string);
            }
        }
    }
    visible_ids
}

