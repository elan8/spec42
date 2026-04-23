use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url};
use tower_lsp::Client;
use tracing::info;

use crate::common::util;
use crate::host::config::Spec42Config;
use crate::workspace::ServerState;

const TRANSIENT_STARTUP_SEMANTIC_DIAGNOSTIC_CODES: &[&str] = &[
    "unresolved_type_reference",
    "unresolved_import_target",
    "unresolved_specializes_reference",
    "missing_library_context",
];

async fn perf_logging_enabled(state: &Arc<RwLock<ServerState>>) -> bool {
    let locked = state.read().await;
    locked.perf_logging_enabled
}

fn semantic_diagnostic_code(diagnostic: &Diagnostic) -> Option<&str> {
    if diagnostic.source.as_deref() != Some("semantic") {
        return None;
    }

    match diagnostic.code.as_ref() {
        Some(NumberOrString::String(code)) => Some(code.as_str()),
        _ => None,
    }
}

fn filter_transient_startup_semantic_diagnostics(
    diagnostics: Vec<Diagnostic>,
    should_suppress_transient_diagnostics: bool,
) -> Vec<Diagnostic> {
    if !should_suppress_transient_diagnostics {
        return diagnostics;
    }

    diagnostics
        .into_iter()
        .filter(|diagnostic| {
            let Some(code) = semantic_diagnostic_code(diagnostic) else {
                return true;
            };
            !TRANSIENT_STARTUP_SEMANTIC_DIAGNOSTIC_CODES.contains(&code)
        })
        .collect()
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

    let mut join_set = tokio::task::JoinSet::new();
    for (uri, text) in docs {
        let state = Arc::clone(state);
        let config = Arc::clone(config);
        let client = client.clone();
        join_set.spawn(async move {
            let diagnostics = collect_diagnostics_for_document(&state, &config, &uri, &text).await;
            let count = diagnostics.len();
            client.publish_diagnostics(uri, diagnostics, None).await;
            count
        });
    }

    while let Some(res) = join_set.join_next().await {
        if let Ok(count) = res {
            diagnostic_count += count;
            published_count += 1;
        }
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
    let has_parse_error = result.errors.iter().any(|error| {
        error
            .severity
            .unwrap_or(sysml_v2_parser::DiagnosticSeverity::Error)
            == sysml_v2_parser::DiagnosticSeverity::Error
    });
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
                sysml_v2_parser::DiagnosticSeverity::Error => DiagnosticSeverity::ERROR,
                sysml_v2_parser::DiagnosticSeverity::Warning => DiagnosticSeverity::WARNING,
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
    if !has_parse_error {
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
        let has_unresolved_import_target = diagnostics.iter().any(|diagnostic| {
            diagnostic.source.as_deref() == Some("semantic")
                && diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_import_target".to_string(),
                    ))
        });
        let has_unresolved_specializes_reference = diagnostics.iter().any(|diagnostic| {
            diagnostic.source.as_deref() == Some("semantic")
                && diagnostic.code.as_ref()
                    == Some(&NumberOrString::String(
                        "unresolved_specializes_reference".to_string(),
                    ))
        });
        if (has_unresolved_type_reference
            || has_unresolved_import_target
            || has_unresolved_specializes_reference)
            && locked.library_paths.is_empty()
        {
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
        diagnostics = filter_transient_startup_semantic_diagnostics(
            diagnostics,
            locked
                .semantic_lifecycle
                .suppresses_transient_semantic_diagnostics(),
        );
    }
    diagnostics
}

#[cfg(test)]
mod tests {
    use super::*;

    fn diag(source: &str, code: &str) -> Diagnostic {
        Diagnostic {
            range: Range {
                start: Position::new(0, 0),
                end: Position::new(0, 1),
            },
            severity: Some(DiagnosticSeverity::WARNING),
            code: Some(NumberOrString::String(code.to_string())),
            code_description: None,
            source: Some(source.to_string()),
            message: format!("{source}:{code}"),
            related_information: None,
            tags: None,
            data: None,
        }
    }

    #[test]
    fn startup_filter_removes_only_transient_semantic_diagnostics() {
        let diagnostics = vec![
            diag("semantic", "unresolved_type_reference"),
            diag("semantic", "unresolved_import_target"),
            diag("semantic", "unresolved_specializes_reference"),
            diag("semantic", "missing_library_context"),
            diag("semantic", "unconnected_port"),
            diag("sysml", "parse_error"),
        ];

        let filtered = filter_transient_startup_semantic_diagnostics(diagnostics, true);
        let remaining_codes: Vec<_> = filtered
            .iter()
            .filter_map(semantic_diagnostic_code)
            .map(str::to_string)
            .collect();

        assert_eq!(remaining_codes, vec!["unconnected_port".to_string()]);
        assert!(filtered
            .iter()
            .any(|diagnostic| diagnostic.source.as_deref() == Some("sysml")));
    }

    #[test]
    fn startup_filter_keeps_all_diagnostics_after_semantic_index_is_ready() {
        let diagnostics = vec![
            diag("semantic", "unresolved_type_reference"),
            diag("semantic", "unconnected_port"),
            diag("sysml", "parse_error"),
        ];

        let filtered = filter_transient_startup_semantic_diagnostics(diagnostics.clone(), false);

        assert_eq!(filtered.len(), diagnostics.len());
    }
}
