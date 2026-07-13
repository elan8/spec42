use super::*;

/// Qualifies a connector endpoint against a single, known enclosing container prefix.
/// Used wherever the caller has a real container in hand (`connect.container_prefix`,
/// `pending.container_prefix`). See [`qualify_occurrence_endpoint`] for the sibling function used
/// when no single prefix is known — the two are deliberately not merged; see that function's doc
/// comment for why.
pub(crate) fn qualify_pending_connection_endpoint(container_prefix: Option<&str>, endpoint: &str) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    // `endpoint` may be a relative feature chain (e.g. `leftMotor::phaseIn`, rendered with
    // `::` regardless of the `.` used in source text) rather than an already-absolute
    // qualified name, so a bare `::` cannot be used to decide it's already qualified.
    let trimmed_dot = trimmed.replace("::", ".");
    let Some(prefix) = container_prefix
        .map(str::trim)
        .filter(|prefix| !prefix.is_empty())
    else {
        return trimmed_dot;
    };
    let prefix_dot = prefix.replace("::", ".");
    if trimmed_dot == prefix_dot || trimmed_dot.starts_with(&format!("{prefix_dot}.")) {
        trimmed_dot
    } else {
        format!("{prefix_dot}.{trimmed_dot}")
    }
}

/// Qualifies a connector endpoint for the ambiguous fallback case: no single enclosing container
/// is known, only a guess-list of every part-def in the file (`def_container_prefixes`). See
/// [`qualify_pending_connection_endpoint`] for the primary path used whenever a real container
/// prefix is available.
///
/// **Do not consolidate these two functions.** They deliberately diverge on how they treat
/// `::`-containing endpoints (see the comment below), and collapsing that distinction previously
/// caused a real regression (`drone_connections_scoped_ibd_matches_full_workspace_filter`, 24 vs 21
/// connectors on the bundled `examples/drone` fixture) — confirmed correct-as-is by a dedicated
/// research pass, not just an oversight left unfixed.
pub(crate) fn qualify_occurrence_endpoint(endpoint: &str, def_container_prefixes: &[String]) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() {
        return String::new();
    }
    // Unlike `qualify_pending_connection_endpoint`, this path has no single known enclosing
    // definition to qualify against — only a guess-list of every part-def in the file — so an
    // endpoint already containing `::` is left as-is rather than risking a wrong prefix match
    // against an unrelated definition.
    if trimmed.contains("::") {
        return trimmed.replace("::", ".");
    }
    if def_container_prefixes.is_empty() {
        return trimmed.to_string();
    }
    let mut qualified: Vec<String> = def_container_prefixes
        .iter()
        .map(|prefix| qualify_pending_connection_endpoint(Some(prefix.as_str()), trimmed))
        .filter(|candidate| !candidate.is_empty())
        .collect();
    qualified.sort_by_key(|candidate| std::cmp::Reverse(candidate.len()));
    qualified
        .into_iter()
        .next()
        .unwrap_or_else(|| trimmed.to_string())
}

pub(crate) fn expand_relative_endpoint_to_part_path(
    endpoint: &str,
    parts: &[IbdPartDto],
    ports: &[IbdPortDto],
) -> String {
    expand_relative_endpoint_with_port_index(endpoint, parts, &build_port_names_by_parent(ports))
}

pub(crate) fn build_port_names_by_parent(ports: &[IbdPortDto]) -> HashMap<String, HashSet<String>> {
    let mut by_parent: HashMap<String, HashSet<String>> = HashMap::new();
    for port in ports {
        by_parent
            .entry(port.parent_id.clone())
            .or_default()
            .insert(port.name.clone());
    }
    by_parent
}

pub(crate) fn expand_relative_endpoint_with_port_index(
    endpoint: &str,
    parts: &[IbdPartDto],
    port_names_by_parent: &HashMap<String, HashSet<String>>,
) -> String {
    let trimmed = endpoint.trim();
    if trimmed.is_empty() || trimmed.contains("::") {
        return trimmed.replace("::", ".");
    }
    let segments: Vec<&str> = trimmed.split('.').collect();
    if segments.len() < 2 {
        return trimmed.to_string();
    }
    let port_name = segments[segments.len() - 1];
    let path_without_port = segments[..segments.len() - 1].join(".");

    let mut best_match: Option<(String, bool, usize)> = None;
    for part in parts {
        let Some(port_names) = port_names_by_parent.get(&part.qualified_name) else {
            continue;
        };
        if !port_names.contains(port_name) {
            continue;
        }
        if path_without_port == part.qualified_name
            || path_without_port.ends_with(&format!(".{}", part.qualified_name))
            || part
                .qualified_name
                .ends_with(&format!(".{path_without_port}"))
        {
            let candidate = format!("{}.{}", part.qualified_name, port_name);
            let is_instance = is_part_instance_kind(&part.element_type);
            let is_better = best_match
                .as_ref()
                .is_none_or(|(_, best_instance, best_len)| {
                    if is_instance && !*best_instance {
                        true
                    } else if is_instance == *best_instance {
                        if is_instance {
                            candidate.len() < *best_len
                        } else {
                            candidate.len() > *best_len
                        }
                    } else {
                        false
                    }
                });
            if is_better {
                let candidate_len = candidate.len();
                best_match = Some((candidate, is_instance, candidate_len));
            }
        }
    }
    best_match
        .map(|(path, _, _)| path)
        .unwrap_or_else(|| trimmed.to_string())
}
