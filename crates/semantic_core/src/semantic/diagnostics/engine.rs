use url::Url;

use crate::semantic::graph::SemanticGraph;

use super::engine_impl::compute_semantic_diagnostics;
use super::types::{DiagnosticsOptions, SemanticDiagnostic};

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
