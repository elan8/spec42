pub(crate) mod library_search;
pub(crate) mod scan;
pub(crate) mod services;
pub(crate) mod state;

pub(crate) use scan::scan_sysml_files;
pub(crate) use services::{
    apply_document_changes, clear_documents_under_roots, indexed_text_or_empty,
    ingest_parsed_scan_entries, ingest_parsed_scan_entries_batch, parse_scanned_entries,
    rebuild_all_document_links, rebuild_semantic_graph_staged, refresh_document, remove_document,
    store_document_text,
};
pub(crate) use state::{IndexEntry, SemanticLifecycle, ServerState};
