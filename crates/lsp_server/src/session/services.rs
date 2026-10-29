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
) -> Result<(Vec<SourceDocument>, ScanSummary), sysml_query::source::SourceError> {
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
    let report = source.load(&provider)?;
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
    Ok((report.documents, summary))
}

#[derive(Debug)]
pub(crate) struct ParsedScanEntry {
    pub(crate) uri: Url,
    pub(crate) document: SourceDocument,
    pub(crate) parsed: ParsedSource,
    pub(crate) parse_errors: Vec<String>,
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
    let editor_revision = state
        .index()
        .get(uri_norm)
        .and_then(|entry| entry.editor_revision.clone());
    state.index_mut().insert(
        uri_norm.clone(),
        IndexEntry {
            document,
            parsed: Some(parsed),
            admitted_to_publication: true,
            editor_revision,
        },
    );
    warning_from_parse_errors(uri_norm, parse_errors, diagnostic_count, context)
}

/// Admit desired source immediately. The bounded analysis worker owns parsing and publication.
pub(crate) fn store_document_text_fast(
    state: &mut impl DocumentStore,
    uri_norm: &Url,
    text: String,
) -> Option<String> {
    let kind = state
        .index()
        .get(uri_norm)
        .map_or(SourceKind::Workspace, |entry| entry.document.kind());
    let document = state
        .services()
        .source
        .admit_url(uri_norm.clone(), &text, kind);
    let previous = state.index().get(uri_norm);
    let parsed = previous
        .filter(|entry| {
            entry.document.digest() == document.digest()
                && entry.document.language() == document.language()
        })
        .and_then(|entry| entry.parsed.clone());
    let editor_revision = previous.and_then(|entry| entry.editor_revision.clone());
    state.index_mut().insert(
        uri_norm.clone(),
        IndexEntry {
            document,
            parsed,
            admitted_to_publication: true,
            editor_revision,
        },
    );
    None
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
        let editor_revision = state
            .index()
            .get(&uri_norm)
            .and_then(|entry| entry.editor_revision.clone());
        state.index_mut().insert(
            uri_norm.clone(),
            IndexEntry {
                document: entry.document,
                parsed: Some(entry.parsed),
                admitted_to_publication: true,
                editor_revision,
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
pub(crate) use edits::{apply_content_changes, remove_document};
pub(crate) use rebuild::{clear_documents_under_roots, index_library_search_entries};
