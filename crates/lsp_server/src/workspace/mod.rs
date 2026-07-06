pub(crate) mod handle;
pub(crate) mod import_graph;
pub(crate) mod library_closure;
pub(crate) mod library_search;
pub(crate) mod scan;
pub(crate) mod services;
pub(crate) mod snapshot;
pub(crate) mod state;
pub(crate) mod viz_cache;

pub(crate) use handle::WorkspaceHandle;
pub(crate) use scan::scan_sysml_files;
// Relocated to `workspace` crate (Tier 2 unified-incremental-engine Phase 1): the disk
// caches are portable, protocol-neutral logic — see
// docs/engineering/TIER2-UNIFIED-INCREMENTAL-ENGINE-DESIGN.md.
pub(crate) use services::{
    indexed_text_or_empty, ingest_parsed_scan_entries, parse_scanned_entries,
    rebuild_all_document_links, rebuild_semantic_graph_staged,
};
pub(crate) use state::{IndexEntry, RuntimeConfig, ServerState};
pub(crate) use workspace::{library_graph_cache, parse_cache};
