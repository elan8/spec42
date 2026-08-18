pub(crate) mod handle;
pub(crate) mod import_graph;
pub(crate) mod library_closure;
pub(crate) mod library_search;
pub(crate) mod scan;
pub(crate) mod services;
pub(crate) mod snapshot;
pub(crate) mod state;

pub(crate) use handle::WorkspaceHandle;
pub(crate) use scan::scan_sysml_files;
// Relocated to `workspace` crate (Tier 2 unified-incremental-engine Phase 1): the disk
// caches are portable, protocol-neutral logic — see
// Incremental workspace consolidation history lives in git.
pub(crate) use services::{parse_scanned_entries, rebuild_publication_inputs_staged};
pub(crate) use state::{IndexEntry, RuntimeConfig, ServerState};
pub(crate) use workspace::parse_cache;
