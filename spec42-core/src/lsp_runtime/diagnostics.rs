use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};
use tower_lsp::Client;
use tracing::info;

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::workspace::ServerState;

async fn perf_logging_enabled(state: &Arc<RwLock<ServerState>>) -> bool {
    let locked = state.read().await;
    locked.perf_logging_enabled
}

pub(crate) async fn publish_document_diagnostics(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    uri: Url,
    text: &str,
) {
    let started_at = Instant::now();
    let diagnostics = collect_diagnostics_for_document(state, config, &uri, text).await;
    if perf_logging_enabled(state).await {
        info!(
            event = "diagnostics:document",
            uri = %uri,
            count = diagnostics.len(),
            elapsed_ms = started_at.elapsed().as_millis() as u64
        );
    }
    client.publish_diagnostics(uri, diagnostics, None).await;
}

pub(crate) async fn publish_workspace_diagnostics(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    target_uris: Option<&[Url]>,
) {
    let started_at = Instant::now();
    let docs: Vec<(Url, String)> = {
        let st = state.read().await;
        if let Some(targets) = target_uris {
            targets
                .iter()
                .filter_map(|uri| {
                    st.index
                        .get(uri)
                        .map(|entry| (uri.clone(), entry.content.clone()))
                })
                .collect()
        } else {
            st.index
                .iter()
                .filter(|(uri, _)| !util::uri_under_any_library(uri, &st.library_paths))
                .map(|(uri, entry)| (uri.clone(), entry.content.clone()))
                .collect()
        }
    };

    let doc_count = docs.len();
    let mut published_count = 0usize;
    let mut diagnostic_count = 0usize;
    for (uri, text) in docs {
        let diagnostics = collect_diagnostics_for_document(state, config, &uri, &text).await;
        diagnostic_count += diagnostics.len();
        published_count += 1;
        client.publish_diagnostics(uri, diagnostics, None).await;
    }
    if perf_logging_enabled(state).await {
        info!(
            event = "diagnostics:workspace",
            target_uris = target_uris.map(|uris| uris.len()).unwrap_or(0),
            published_docs = published_count,
            discovered_docs = doc_count,
            diagnostics = diagnostic_count,
            elapsed_ms = started_at.elapsed().as_millis() as u64
        );
    }
}

async fn collect_diagnostics_for_document(
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    uri: &Url,
    text: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let result = util::parse_for_editor(text);
    for error in &result.errors {
        let range = error
            .to_lsp_range()
            .map(|(sl, sc, el, ec)| Range {
                start: Position::new(sl, sc),
                end: Position::new(el, ec),
            })
            .unwrap_or_else(|| Range {
                start: Position::new(0, 0),
                end: Position::new(0, 0),
            });
        let severity = error
            .severity
            .map(|severity| match severity {
                sysml_parser::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                sysml_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
            })
            .unwrap_or(DiagnosticSeverity::ERROR);
        diagnostics.push(Diagnostic {
            range,
            severity: Some(severity),
            code: error.code.clone().map(NumberOrString::String),
            code_description: None,
            source: Some("sysml".to_string()),
            message: error.message.clone(),
            related_information: None,
            tags: None,
            data: None,
        });
    }
    for usage in util::untyped_part_usage_diagnostics(text) {
        diagnostics.push(Diagnostic {
            range: usage.range,
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String("untyped_part_usage".to_string())),
            code_description: None,
            source: Some("sysml".to_string()),
            message: format!("Part '{}' has no declared type.", usage.name),
            related_information: None,
            tags: None,
            data: None,
        });
    }
    if result.errors.is_empty() {
        for range in util::missing_semicolon_ranges(text) {
            diagnostics.push(Diagnostic {
                range,
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("missing_semicolon".to_string())),
                code_description: None,
                source: Some("sysml".to_string()),
                message: "Missing ';' at end of statement.".to_string(),
                related_information: None,
                tags: None,
                data: None,
            });
        }
    }
    if result.errors.is_empty() {
        let uri_norm = util::normalize_file_uri(uri);
        let locked = state.read().await;
        for provider in &config.check_providers {
            diagnostics.extend(provider.compute_diagnostics(&locked.semantic_graph, &uri_norm));
        }
        let has_unresolved_type_reference = diagnostics.iter().any(|diagnostic| {
            diagnostic.source.as_deref() == Some("semantic")
                && diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_type_reference".to_string(),
                    ))
        });
        if has_unresolved_type_reference && locked.library_paths.is_empty() {
            if let Some(import_range) = util::import_statement_ranges(text).into_iter().next() {
                diagnostics.push(Diagnostic {
                    range: import_range,
                    severity: Some(DiagnosticSeverity::INFORMATION),
                    code: Some(NumberOrString::String(
                        "missing_library_context".to_string(),
                    )),
                    code_description: None,
                    source: Some("semantic".to_string()),
                    message: "This document imports external library symbols, but no SysML library paths are configured or indexed. Install or configure a library if these references should resolve.".to_string(),
                    related_information: None,
                    tags: None,
                    data: None,
                });
            }
        }
    }
    diagnostics
}
