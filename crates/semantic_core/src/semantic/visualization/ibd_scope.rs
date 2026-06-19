//! IBD scoping and filtering for interconnection visualization.

use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::ibd::{IbdDataDto, IbdRootViewDto};
use crate::semantic::visualization::scope::IbdBuildScope;

pub fn filter_ibd_by_visible_ids(ibd: &IbdDataDto, visible_ids: &HashSet<String>) -> IbdDataDto {
    if visible_ids.is_empty() {
        return ibd.clone();
    }
    let visible_dot_ids: HashSet<String> =
        visible_ids.iter().map(|id| id.replace("::", ".")).collect();
    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| {
            visible_ids.contains(&part.id) || visible_dot_ids.contains(&part.qualified_name)
        })
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> = parts
        .iter()
        .map(|part| part.qualified_name.as_str())
        .collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            visible_ids.contains(&port.id)
                || visible_dot_ids.contains(&port.id.replace("::", "."))
                || part_ids.contains(port.parent_id.as_str())
                || part_qualified_names.contains(port.parent_id.as_str())
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let endpoint_visible = |endpoint: &str| {
        let endpoint = endpoint.replace("::", ".");
        if endpoint.is_empty() {
            return false;
        }
        port_dot_ids.contains(&endpoint)
            || part_qualified_names
                .iter()
                .any(|qn| endpoint == *qn || endpoint.starts_with(&format!("{qn}.")))
            || part_ids.iter().any(|id| {
                let id_dot = id.replace("::", ".");
                endpoint == id_dot || endpoint.starts_with(&format!("{id_dot}."))
            })
    };
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || endpoint_visible(&connector.source_id)
                || endpoint_visible(&connector.target_id)
                || connector.source_part_id.as_deref().is_some_and(|part_id| {
                    part_ids.contains(part_id) || part_qualified_names.contains(part_id)
                })
                || connector.target_part_id.as_deref().is_some_and(|part_id| {
                    part_ids.contains(part_id) || part_qualified_names.contains(part_id)
                })
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group
                .member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();
    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| {
                visible_ids.contains(&part.id) || visible_dot_ids.contains(&part.qualified_name)
            })
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                visible_ids.contains(&port.id)
                    || visible_dot_ids.contains(&port.id.replace("::", "."))
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_ids.contains(connector.source_id.as_str())
                    || filtered_part_ids.contains(connector.target_id.as_str())
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group
                    .member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }
    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

fn endpoint_matches_root_prefix(endpoint: &str, root_prefix: &str) -> bool {
    endpoint == root_prefix || endpoint.starts_with(&format!("{root_prefix}."))
}

fn is_variant_or_alternative_endpoint(endpoint: &str) -> bool {
    endpoint.contains(".Variants.") || endpoint.contains(".expansionAlternatives.")
}

fn enrich_root_prefixes_for_interconnection(
    ibd: &crate::semantic::ibd::IbdDataDto,
    root_prefixes: &HashSet<String>,
) -> HashSet<String> {
    let mut enriched: HashSet<String> = root_prefixes
        .iter()
        .map(|id| id.replace("::", "."))
        .collect();
    let mappings = crate::semantic::ibd::infer_def_instance_scope_mappings_for_ibd(ibd);
    for prefix in root_prefixes {
        let dot = prefix.replace("::", ".");
        for (def_root, instance_root) in &mappings {
            if endpoint_matches_root_prefix(&dot, def_root) {
                let remainder = dot
                    .strip_prefix(def_root)
                    .and_then(|value| value.strip_prefix('.'))
                    .unwrap_or("");
                let instance_prefix = if remainder.is_empty() {
                    instance_root.clone()
                } else {
                    format!("{instance_root}.{remainder}")
                };
                enriched.insert(instance_prefix);
            } else if endpoint_matches_root_prefix(&dot, instance_root) {
                let remainder = dot
                    .strip_prefix(instance_root)
                    .and_then(|value| value.strip_prefix('.'))
                    .unwrap_or("");
                let def_prefix = if remainder.is_empty() {
                    def_root.clone()
                } else {
                    format!("{def_root}.{remainder}")
                };
                enriched.insert(def_prefix);
            }
        }
    }
    enriched
}

fn architecture_scope_prefix(root_prefixes: &HashSet<String>) -> Option<String> {
    let mut scopes: HashSet<String> = HashSet::new();
    for prefix in root_prefixes {
        if let Some(pos) = prefix.find(".architecture.") {
            scopes.insert(prefix[..pos + ".architecture".len()].to_string());
        } else if prefix.ends_with(".architecture") {
            scopes.insert(prefix.clone());
        }
    }
    if scopes.len() == 1 {
        scopes.into_iter().next()
    } else {
        None
    }
}

pub(crate) fn filter_ibd_by_root_prefixes(ibd: &IbdDataDto, root_prefixes: &HashSet<String>) -> IbdDataDto {
    let strict_part_expose = root_prefixes.len() > 1;
    let architecture_scope = architecture_scope_prefix(root_prefixes);
    let matches_any_root = |endpoint: &str| {
        root_prefixes
            .iter()
            .any(|prefix| endpoint_matches_root_prefix(endpoint, prefix))
    };
    let endpoint_in_architecture_scope = |endpoint: &str| {
        architecture_scope
            .as_ref()
            .is_none_or(|scope| endpoint.starts_with(scope))
    };
    let connector_in_scope = |connector: &crate::semantic::ibd::IbdConnectorDto| {
        if is_variant_or_alternative_endpoint(&connector.source_id)
            || is_variant_or_alternative_endpoint(&connector.target_id)
        {
            return false;
        }
        if !endpoint_in_architecture_scope(&connector.source_id)
            || !endpoint_in_architecture_scope(&connector.target_id)
        {
            return false;
        }
        if strict_part_expose {
            return matches_any_root(&connector.source_id)
                && matches_any_root(&connector.target_id);
        }
        if architecture_scope.is_some() {
            return true;
        }
        matches_any_root(&connector.source_id) || matches_any_root(&connector.target_id)
    };

    let parts: Vec<_> = ibd
        .parts
        .iter()
        .filter(|part| {
            endpoint_in_architecture_scope(&part.qualified_name)
                && matches_any_root(&part.qualified_name)
        })
        .cloned()
        .collect();
    let part_ids: HashSet<_> = parts.iter().map(|part| part.id.as_str()).collect();
    let part_qualified_names: HashSet<_> = parts
        .iter()
        .map(|part| part.qualified_name.as_str())
        .collect();
    let ports: Vec<_> = ibd
        .ports
        .iter()
        .filter(|port| {
            endpoint_in_architecture_scope(&port.parent_id)
                && (matches_any_root(&port.parent_id)
                    || part_ids.contains(port.parent_id.as_str())
                    || part_qualified_names.contains(port.parent_id.as_str()))
        })
        .cloned()
        .collect();
    let port_ids: HashSet<_> = ports.iter().map(|port| port.id.as_str()).collect();
    let port_dot_ids: HashSet<String> = ports
        .iter()
        .map(|port| port.id.replace("::", "."))
        .collect();
    let connectors: Vec<_> = ibd
        .connectors
        .iter()
        .filter(|connector| {
            if !connector_in_scope(connector) {
                return false;
            }
            port_ids.contains(connector.source.as_str())
                || port_ids.contains(connector.target.as_str())
                || port_dot_ids.contains(&connector.source_id)
                || port_dot_ids.contains(&connector.target_id)
                || part_qualified_names.contains(connector.source_id.as_str())
                || part_qualified_names.contains(connector.target_id.as_str())
                || connector_in_scope(connector)
        })
        .cloned()
        .collect();
    let container_groups: Vec<_> = ibd
        .container_groups
        .iter()
        .filter(|group| {
            group
                .member_part_ids
                .iter()
                .any(|member_id| part_ids.contains(member_id.as_str()))
        })
        .cloned()
        .collect();

    let mut root_views = HashMap::new();
    for (name, view) in &ibd.root_views {
        let filtered_parts: Vec<_> = view
            .parts
            .iter()
            .filter(|part| matches_any_root(&part.qualified_name))
            .cloned()
            .collect();
        let filtered_part_ids: HashSet<_> =
            filtered_parts.iter().map(|part| part.id.as_str()).collect();
        let filtered_part_qualified_names: HashSet<_> = filtered_parts
            .iter()
            .map(|part| part.qualified_name.as_str())
            .collect();
        let filtered_ports: Vec<_> = view
            .ports
            .iter()
            .filter(|port| {
                matches_any_root(&port.parent_id)
                    || filtered_part_ids.contains(port.parent_id.as_str())
                    || filtered_part_qualified_names.contains(port.parent_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_port_ids: HashSet<_> =
            filtered_ports.iter().map(|port| port.id.as_str()).collect();
        let filtered_port_dot_ids: HashSet<String> = filtered_ports
            .iter()
            .map(|port| port.id.replace("::", "."))
            .collect();
        let filtered_connectors: Vec<_> = view
            .connectors
            .iter()
            .filter(|connector| {
                matches_any_root(&connector.source_id)
                    || matches_any_root(&connector.target_id)
                    || filtered_port_ids.contains(connector.source.as_str())
                    || filtered_port_ids.contains(connector.target.as_str())
                    || filtered_port_dot_ids.contains(&connector.source_id)
                    || filtered_port_dot_ids.contains(&connector.target_id)
                    || filtered_part_qualified_names.contains(connector.source_id.as_str())
                    || filtered_part_qualified_names.contains(connector.target_id.as_str())
            })
            .cloned()
            .collect();
        let filtered_container_groups: Vec<_> = view
            .container_groups
            .iter()
            .filter(|group| {
                group
                    .member_part_ids
                    .iter()
                    .any(|member_id| filtered_part_ids.contains(member_id.as_str()))
            })
            .cloned()
            .collect();
        if !filtered_parts.is_empty() || !filtered_connectors.is_empty() {
            root_views.insert(
                name.clone(),
                IbdRootViewDto {
                    parts: filtered_parts,
                    ports: filtered_ports,
                    connectors: filtered_connectors,
                    container_groups: filtered_container_groups,
                    package_container_groups: Vec::new(),
                },
            );
        }
    }

    let root_candidates: Vec<_> = ibd
        .root_candidates
        .iter()
        .filter(|candidate| root_views.contains_key(*candidate))
        .cloned()
        .collect();
    let default_root = ibd
        .default_root
        .as_ref()
        .filter(|root| root_views.contains_key(root.as_str()))
        .cloned()
        .or_else(|| root_candidates.first().cloned());
    IbdDataDto {
        parts,
        ports,
        connectors,
        container_groups,
        package_container_groups: Vec::new(),
        root_candidates,
        root_views,
        default_root,
    }
}

/// Optional trace snapshot for diagnosing empty interconnection payloads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IbdScopeTrace {
    pub full_parts: usize,
    pub full_connectors: usize,
    pub visible_parts: usize,
    pub visible_connectors: usize,
    pub root_scoped_parts: usize,
    pub root_scoped_connectors: usize,
    pub chosen: &'static str,
}

pub fn select_interconnection_ibd_scope_with_trace(
    full_ibd: &IbdDataDto,
    selected_ids: &HashSet<String>,
    selected_exposed_ids: Option<&HashSet<String>>,
) -> (IbdDataDto, IbdScopeTrace) {
    let trace =
        |chosen: &'static str, visible: &IbdDataDto, root_scoped: &IbdDataDto| -> IbdScopeTrace {
            IbdScopeTrace {
                full_parts: full_ibd.parts.len(),
                full_connectors: full_ibd.connectors.len(),
                visible_parts: visible.parts.len(),
                visible_connectors: visible.connectors.len(),
                root_scoped_parts: root_scoped.parts.len(),
                root_scoped_connectors: root_scoped.connectors.len(),
                chosen,
            }
        };

    if selected_ids.is_empty() && selected_exposed_ids.is_none_or(|ids| ids.is_empty()) {
        return (
            full_ibd.clone(),
            trace("full_unfiltered", full_ibd, full_ibd),
        );
    }

    let mut scoped_source = full_ibd.clone();
    crate::semantic::ibd::normalize_ibd_to_instance_paths(&mut scoped_source);
    crate::semantic::ibd::enrich_connector_endpoint_refs(
        &mut scoped_source.connectors,
        &scoped_source.parts,
        &scoped_source.ports,
    );
    let visible_scope_ibd = filter_ibd_by_visible_ids(&scoped_source, selected_ids);
    let root_prefixes: HashSet<String> = selected_exposed_ids
        .map(|exposed_ids| enrich_root_prefixes_for_interconnection(full_ibd, exposed_ids))
        .unwrap_or_default();
    if root_prefixes.is_empty() {
        let label = if visible_scope_ibd.parts.is_empty() && visible_scope_ibd.connectors.is_empty()
        {
            "full_visible_empty"
        } else {
            "visible_ids"
        };
        let empty = IbdDataDto {
            parts: Vec::new(),
            ports: Vec::new(),
            connectors: Vec::new(),
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: HashMap::new(),
        };
        let trace_result = trace(label, &visible_scope_ibd, &empty);
        let chosen = if label == "full_visible_empty" {
            full_ibd.clone()
        } else {
            visible_scope_ibd
        };
        return (chosen, trace_result);
    }
    let mut root_scoped_ibd = filter_ibd_by_root_prefixes(&scoped_source, &root_prefixes);
    crate::semantic::ibd::enrich_connector_endpoint_refs(
        &mut root_scoped_ibd.connectors,
        &root_scoped_ibd.parts,
        &root_scoped_ibd.ports,
    );
    let architecture_scoped = architecture_scope_prefix(&root_prefixes).is_some();
    let root_scope_has_content =
        !root_scoped_ibd.parts.is_empty() || !root_scoped_ibd.connectors.is_empty();
    let root_scope_dropped_visible_connectors =
        root_scoped_ibd.connectors.is_empty() && !visible_scope_ibd.connectors.is_empty();
    if root_scope_has_content && !root_scope_dropped_visible_connectors {
        let trace_result = trace("root_prefixes", &visible_scope_ibd, &root_scoped_ibd);
        return (root_scoped_ibd, trace_result);
    }
    if (!architecture_scoped || root_scope_dropped_visible_connectors)
        && (!visible_scope_ibd.parts.is_empty() || !visible_scope_ibd.connectors.is_empty())
    {
        let trace_result = trace("visible_ids_fallback", &visible_scope_ibd, &root_scoped_ibd);
        return (visible_scope_ibd, trace_result);
    }
    let trace_result = trace("full_scope_miss", &visible_scope_ibd, &root_scoped_ibd);
    (full_ibd.clone(), trace_result)
}

pub(crate) fn ibd_scope_trace_enabled() -> bool {
    std::env::var("SPEC42_TRACE_IBD")
        .ok()
        .is_some_and(|value| matches!(value.as_str(), "1" | "true" | "TRUE" | "yes" | "YES"))
}

pub(crate) fn log_ibd_scope_trace(trace: &IbdScopeTrace) {
    eprintln!(
        "[spec42 ibd scope] chosen={} full={{parts:{},connectors:{}}} visible={{parts:{},connectors:{}}} root_scoped={{parts:{},connectors:{}}}",
        trace.chosen,
        trace.full_parts,
        trace.full_connectors,
        trace.visible_parts,
        trace.visible_connectors,
        trace.root_scoped_parts,
        trace.root_scoped_connectors,
    );
}

pub fn select_interconnection_ibd_scope(
    full_ibd: &IbdDataDto,
    selected_ids: &HashSet<String>,
    selected_exposed_ids: Option<&HashSet<String>>,
) -> IbdDataDto {
    select_interconnection_ibd_scope_with_trace(full_ibd, selected_ids, selected_exposed_ids).0
}

#[cfg(test)]
mod interconnection_scope_tests {
    use super::*;
    use crate::semantic::ibd::{IbdConnectorDto, IbdDataDto, IbdPartDto};

    fn sample_ibd() -> IbdDataDto {
        IbdDataDto {
            parts: vec![IbdPartDto {
                id: "Pkg::droneInstance".to_string(),
                node_id: "Pkg.droneInstance".to_string(),
                name: "droneInstance".to_string(),
                qualified_name: "Pkg.droneInstance".to_string(),
                uri: None,
                container_id: None,
                element_type: "part".to_string(),
                attributes: HashMap::new(),
            }],
            ports: Vec::new(),
            connectors: vec![IbdConnectorDto {
                source: "a".to_string(),
                target: "b".to_string(),
                source_id: "Pkg.droneInstance.portA".to_string(),
                target_id: "Pkg.droneInstance.portB".to_string(),
                source_part_id: None,
                target_part_id: None,
                source_port_id: None,
                target_port_id: None,
                rel_type: "connection".to_string(),
            }],
            container_groups: Vec::new(),
            package_container_groups: Vec::new(),
            root_candidates: Vec::new(),
            default_root: None,
            root_views: HashMap::new(),
        }
    }

    #[test]
    fn interconnection_scope_keeps_full_ibd_when_graph_ids_do_not_match_parts() {
        let full = sample_ibd();
        let graph_ids: HashSet<String> = HashSet::from(["unrelated-graph-node-id".to_string()]);
        let exposed: HashSet<String> = HashSet::from(["Wrong::expose::path".to_string()]);

        let (scoped, trace) =
            select_interconnection_ibd_scope_with_trace(&full, &graph_ids, Some(&exposed));

        assert_eq!(scoped.parts.len(), full.parts.len());
        assert_eq!(scoped.connectors.len(), full.connectors.len());
        assert_eq!(trace.chosen, "full_scope_miss");
    }

    #[test]
    fn interconnection_scope_returns_full_ibd_when_filters_are_empty() {
        let full = sample_ibd();
        let (scoped, trace) =
            select_interconnection_ibd_scope_with_trace(&full, &HashSet::new(), None);
        assert_eq!(scoped.parts.len(), full.parts.len());
        assert_eq!(trace.chosen, "full_unfiltered");
    }
}
