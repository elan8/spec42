//! Shared exposed-id membership matching for view-kind diagram lists.
//!
//! `filter_sequence_diagrams_by_exposed_ids` (`sequence_views/mod.rs`) and
//! `filter_state_machines_by_exposed_ids` (`state_views/mod.rs`) previously reimplemented this
//! exact 3-way id-matching logic independently. Note that `filter_activity_diagrams_by_graph`
//! (`visualization/projection.rs`) is *not* a duplicate of this — it matches against an
//! already-projected `SysmlGraphDto`'s action-like nodes by `(name, top_level_package)` rather
//! than against a raw exposed-ids set, a deliberately different mechanism for action-flow-view.

use std::collections::HashSet;

/// Whether `id` (optionally paired with `package_path`/`name` for the reconstructed-qualified-name
/// fallback) is covered by `exposed_ids`: exact match, a `::`-prefixed descendant, or a
/// `package_path::name` reconstruction matching an exposed id verbatim.
fn is_exposed(id: &str, package_path: &str, name: &str, exposed_ids: &HashSet<String>) -> bool {
    exposed_ids.iter().any(|exposed_id| {
        id == exposed_id
            || id.starts_with(&format!("{exposed_id}::"))
            || format!("{package_path}::{name}").trim_matches(':') == exposed_id
    })
}

/// Filters `items` to those whose `(id, package_path, name)` — extracted via `key_of` — is
/// exposed under `exposed_ids`. An empty `exposed_ids` set means "no exposure restriction": every
/// item passes through unfiltered.
pub(crate) fn filter_by_exposed_ids<T: Clone>(
    items: &[T],
    exposed_ids: &HashSet<String>,
    key_of: impl Fn(&T) -> (&str, &str, &str),
) -> Vec<T> {
    if exposed_ids.is_empty() {
        return items.to_vec();
    }
    items
        .iter()
        .filter(|item| {
            let (id, package_path, name) = key_of(item);
            is_exposed(id, package_path, name, exposed_ids)
        })
        .cloned()
        .collect()
}
