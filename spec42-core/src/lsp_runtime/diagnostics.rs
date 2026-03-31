use std::sync::Arc;

use tokio::sync::RwLock;
use tower_lsp::lsp_types::{
    Diagnostic, DiagnosticSeverity, NumberOrString, Position, Range, Url,
};
use tower_lsp::Client;

use crate::config::Spec42Config;
use crate::util;
use crate::workspace::ServerState;

pub(crate) async fn publish_document_diagnostics(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    uri: Url,
    text: &str,
) {
    let diagnostics = collect_diagnostics_for_document(state, config, &uri, text).await;
    client.publish_diagnostics(uri, diagnostics, None).await;
}

pub(crate) async fn publish_workspace_diagnostics(
    client: &Client,
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    target_uris: Option<&[Url]>,
) {
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
                .filter(|(uri, _)| {
                    !util::uri_under_any_library(uri, &st.library_paths)
                        && (st.workspace_roots.is_empty()
                            || uri_under_any_root(uri, &st.workspace_roots))
                })
                .map(|(uri, entry)| (uri.clone(), entry.content.clone()))
                .collect()
        }
    };

    for (uri, text) in docs {
        let diagnostics = collect_diagnostics_for_document(state, config, &uri, &text).await;
        client.publish_diagnostics(uri, diagnostics, None).await;
    }
}

async fn collect_diagnostics_for_document(
    state: &Arc<RwLock<ServerState>>,
    config: &Arc<Spec42Config>,
    uri: &Url,
    text: &str,
) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();
    let result = sysml_parser::parse_with_diagnostics(text);
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
    }
    diagnostics
}

fn uri_under_any_root(uri: &Url, roots: &[Url]) -> bool {
    let uri_path = match uri.to_file_path() {
        Ok(path) => path,
        Err(_) => return false,
    };
    roots.iter().any(|root| {
        root.to_file_path()
            .map(|root_path| uri_path.starts_with(root_path))
            .unwrap_or(false)
    })
}
