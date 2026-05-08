use tower_lsp::lsp_types::{Diagnostic, DiagnosticSeverity as LspSeverity, NumberOrString, Url};

use crate::semantic::graph::SemanticGraph;

use super::engine_impl::compute_semantic_diagnostics;
use super::types::{DiagnosticSeverity, DiagnosticsOptions, SemanticDiagnostic};

/// Collects semantic diagnostics from an already-built semantic graph.
///
/// This API is graph-first and host-agnostic: callers provide the graph and URI,
/// and receive neutral diagnostics that can be mapped to transport-specific types.
pub fn collect_diagnostics_from_graph(
    graph: &SemanticGraph,
    uri: &Url,
    _options: DiagnosticsOptions,
) -> Vec<SemanticDiagnostic> {
    compute_semantic_diagnostics(graph, uri)
        .into_iter()
        .map(|diagnostic| to_semantic(uri, diagnostic))
        .collect()
}

fn to_semantic(uri: &Url, diagnostic: Diagnostic) -> SemanticDiagnostic {
    let severity = match diagnostic.severity.unwrap_or(LspSeverity::WARNING) {
        LspSeverity::ERROR => DiagnosticSeverity::Error,
        LspSeverity::WARNING => DiagnosticSeverity::Warning,
        LspSeverity::INFORMATION => DiagnosticSeverity::Information,
        LspSeverity::HINT => DiagnosticSeverity::Hint,
        _ => DiagnosticSeverity::Warning,
    };
    let code = match diagnostic.code {
        Some(NumberOrString::String(code)) => code,
        Some(NumberOrString::Number(code)) => code.to_string(),
        None => "semantic_diagnostic".to_string(),
    };
    SemanticDiagnostic {
        uri: uri.clone(),
        range: diagnostic.range,
        severity,
        source: diagnostic.source.unwrap_or_else(|| "semantic".to_string()),
        code,
        message: diagnostic.message,
        related_information: Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::build_graph_from_doc;

    #[test]
    fn collect_diagnostics_from_graph_emits_unresolved_import_target() {
        let input = r#"
            package P {
                import Q::*;
            }
        "#;
        let parsed = sysml_v2_parser::parse(input).expect("parse");
        let uri = Url::parse("file:///test.sysml").expect("uri");
        let graph = build_graph_from_doc(&parsed, &uri);
        let diagnostics = collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(diagnostics
            .iter()
            .any(|diagnostic| diagnostic.code == "unresolved_import_target"));
    }
}
