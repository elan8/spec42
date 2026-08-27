use crate::common::util;
use crate::session::state::{DocumentStore, IndexEntry, ScanSummary};
use language_service::library_search;
use rayon::prelude::*;
use sysml_query::source::{SourceDocument, SourceKind};
use sysml_query::syntax::ParsedSource;
use sysml_query::Services;
use tower_lsp::lsp_types::{MessageType, TextDocumentContentChangeEvent, Url};

/// Walks the given roots for SysML sources through the source service.
///
/// Ignore rules (`.gitignore` and friends) are honoured by the provider, so a project's own
/// `target/` or `node_modules/` is not indexed. Disk content arrives already normalised to LF,
/// which is what the editor sends on `didOpen`, so a CRLF file does not look "changed" when
/// opened. Counts are kept for the startup log.
pub(crate) fn scan_sysml_files(
    roots: Vec<Url>,
    source: &sysml_query::source::SourceService,
) -> (Vec<SourceDocument>, ScanSummary) {
    use sysml_query::source::{FilesystemProvider, SourceError, SourceKind};

    let mut summary = ScanSummary::default();
    let mut paths = Vec::new();
    for root in roots {
        match root.to_file_path() {
            Ok(path) => paths.push(path),
            Err(_) => summary.roots_skipped_non_file += 1,
        }
    }
    let provider =
        FilesystemProvider::new(paths, SourceKind::Workspace).within_project_boundary(true);
    let report = match source.load(&provider) {
        Ok(report) => report,
        Err(error) => {
            tracing::warn!(%error, "workspace scan failed");
            return (Vec::new(), summary);
        }
    };
    summary.roots_scanned = report.roots_scanned;
    summary.roots_skipped_non_file += report.roots_skipped;
    summary.candidate_files = report.candidate_files;
    summary.files_loaded = report.documents.len();
    for skipped in &report.skipped {
        match skipped.error {
            SourceError::InvalidUri { .. } => summary.uri_failures += 1,
            _ => summary.read_failures += 1,
        }
    }
    (report.documents, summary)
}

#[derive(Debug)]
pub(crate) struct ParsedScanEntry {
    pub(crate) uri: Url,
    pub(crate) document: SourceDocument,
    pub(crate) parsed: ParsedSource,
    pub(crate) parse_errors: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) struct RebuildAllDocumentLinksMetrics {
    pub(crate) uri_count: usize,
    pub(crate) parsed_doc_count: usize,
    pub(crate) remove_nodes_ms: u32,
    pub(crate) rebuild_graphs_ms: u32,
    pub(crate) cross_edge_resolution_ms: u32,
    pub(crate) workspace_relationship_linking_ms: u32,
    pub(crate) pending_relationship_resolution_ms: u32,
    pub(crate) expression_evaluation_ms: u32,
    pub(crate) cross_document_edges_ms: u32,
    pub(crate) refresh_symbols_ms: u32,
    pub(crate) total_ms: u32,
}

fn warning_from_parse_errors(
    uri_norm: &Url,
    parse_errors: &[String],
    diagnostic_count: usize,
    context: &str,
) -> Option<String> {
    if parse_errors.is_empty() {
        None
    } else {
        Some(format!(
            "sysml parse for editor produced {} diagnostic(s) for {} during {}: {}",
            diagnostic_count,
            uri_norm.as_str(),
            context,
            parse_errors.join("; ")
        ))
    }
}

/// Scan entries for documents the source authority already admitted (the library closure).
pub(crate) fn parse_scanned_documents(
    documents: Vec<SourceDocument>,
    parallel_enabled: bool,
    services: &Services,
) -> Vec<ParsedScanEntry> {
    let entry = |document: SourceDocument| {
        let parsed = services.syntax.parse(&document);
        let parse_errors = util::parse_failure_diagnostics(&parsed, 5);
        ParsedScanEntry {
            uri: document.uri().clone(),
            document,
            parsed,
            parse_errors,
        }
    };
    if !parallel_enabled || documents.len() < 2 {
        return documents.into_iter().map(entry).collect();
    }
    documents.into_par_iter().map(entry).collect()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn store_parsed_document_text(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    document: SourceDocument,
    parsed: ParsedSource,
    parse_errors: &[String],
    diagnostic_count: usize,
    context: &str,
    _evaluate: bool,
) -> Option<String> {
    state.index_mut().insert(
        uri_norm.clone(),
        IndexEntry {
            document,
            parsed,
            admitted_to_publication: true,
        },
    );
    warning_from_parse_errors(uri_norm, parse_errors, diagnostic_count, context)
}

pub(crate) fn store_document_text(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let document =
        state
            .services()
            .source
            .admit_url(uri_norm.clone(), &text, SourceKind::Workspace);
    let parsed = state.services().syntax.parse(&document);
    let parse_errors = parsed
        .diagnostics()
        .iter()
        .take(5)
        .map(|e| e.message.clone())
        .collect::<Vec<_>>();
    let diagnostic_count = parsed.diagnostics().len();
    store_parsed_document_text(
        state,
        uri_norm,
        document,
        parsed,
        &parse_errors,
        diagnostic_count,
        "store_document_text",
        true,
    )
}

/// Like `store_document_text` but skips the expensive cross-document evaluation
/// pass (`evaluate: false`). The caller is responsible for scheduling an async
/// relink to rebuild cross-document edges and expression evaluation.
pub(crate) fn store_document_text_fast(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let document =
        state
            .services()
            .source
            .admit_url(uri_norm.clone(), &text, SourceKind::Workspace);
    let parsed = state.services().syntax.parse(&document);
    let parse_errors = parsed
        .diagnostics()
        .iter()
        .take(5)
        .map(|e| e.message.clone())
        .collect::<Vec<_>>();
    let diagnostic_count = parsed.diagnostics().len();
    store_parsed_document_text(
        state,
        uri_norm,
        document,
        parsed,
        &parse_errors,
        diagnostic_count,
        "store_document_text_fast",
        false,
    )
}

pub(crate) fn refresh_document(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    content: String,
) -> Option<String> {
    store_document_text(state, uri_norm, content)
}

pub(crate) fn ingest_parsed_scan_entries(
    state: &mut impl DocumentStore,
    entries: Vec<ParsedScanEntry>,
) -> Vec<(Url, Option<String>)> {
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let uri_norm = util::normalize_file_uri(&entry.uri);
        let warning = store_parsed_document_text(
            state,
            &uri_norm,
            entry.document,
            entry.parsed,
            &entry.parse_errors,
            entry.parse_errors.len(),
            "workspace_scan",
            false,
        );
        loaded.push((uri_norm, warning));
    }
    loaded
}

/// A faster version of ingest_parsed_scan_entries that avoids per-document relinking/evaluation.
/// Intended for use during startup when a full relink is performed immediately after.
pub(crate) fn ingest_parsed_scan_entries_batch(
    state: &mut impl DocumentStore,
    entries: Vec<ParsedScanEntry>,
) -> Vec<(Url, Option<String>)> {
    let mut loaded = Vec::with_capacity(entries.len());
    for entry in entries {
        let uri_norm = util::normalize_file_uri(&entry.uri);
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                document: entry.document,
                parsed: entry.parsed,
                admitted_to_publication: true,
            },
        );
        let warning = warning_from_parse_errors(
            &uri_norm,
            &entry.parse_errors,
            entry.parse_errors.len(),
            "workspace_scan_batch",
        );
        loaded.push((uri_norm, warning));
    }
    loaded
}

/// Applies incoming text edits to the in-memory document only (no parsing, no
/// semantic publication work). Cheap and safe to run while holding the server's
/// write lock. Returns whether the content actually changed, so the caller
/// can decide whether a (potentially slow) parse is needed.
mod edits;
mod rebuild;
pub(crate) use edits::{apply_content_changes, apply_parsed_document_update, remove_document};
pub(crate) use rebuild::{clear_documents_under_roots, index_library_paths_for_search};
