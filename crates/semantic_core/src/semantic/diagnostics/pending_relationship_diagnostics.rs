//! Diagnostics for relationship queues left unresolved after graph construction.

use url::Url;

use crate::semantic::diagnostics::helpers::diag;
use crate::semantic::diagnostics::types::DiagnosticSeverity;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::text_span::{TextPosition, TextRange};
use crate::{RelationshipKind, SemanticDiagnostic};

/// Append error diagnostics for pending relationships on `uri` that could not be resolved.
///
/// Call after [`crate::resolve_workspace_pending_relationships`] (as in
/// [`crate::build_semantic_graph_from_documents`]).
pub fn append_unresolved_pending_relationship_diagnostics(
    graph: &SemanticGraph,
    uri: &Url,
    diagnostics: &mut Vec<SemanticDiagnostic>,
) {
    for pending in &graph.pending_relationships {
        if pending.uri != *uri {
            continue;
        }
        diagnostics.push(diag(
            uri,
            TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 0)),
            DiagnosticSeverity::Error,
            "semantic",
            "unresolved_pending_relationship",
            format!(
                "unresolved {} relationship from '{}' to '{}'",
                pending.kind.as_str(),
                pending.source_qualified,
                pending.target_qualified
            ),
        ));
    }

    for pending in &graph.pending_expression_relationships {
        if pending.uri != *uri {
            continue;
        }
        if matches!(
            pending.kind,
            RelationshipKind::Satisfy | RelationshipKind::Allocate
        ) {
            continue;
        }
        diagnostics.push(diag(
            uri,
            pending.source_range,
            DiagnosticSeverity::Error,
            "semantic",
            "unresolved_pending_expression_relationship",
            format!(
                "unresolved {} relationship from '{}' to '{}'",
                pending.kind.as_str(),
                pending.source_expression,
                pending.target_expression
            ),
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::collect_diagnostics_from_graph;
    use crate::semantic::graph::{PendingExpressionRelationship, PendingRelationship};
    use crate::semantic::text_span::{TextPosition, TextRange};
    use crate::{DiagnosticsOptions, RelationshipKind, SemanticGraph};
    use url::Url;

    #[test]
    fn collect_diagnostics_from_graph_reports_unresolved_pending_relationship() {
        let uri = Url::parse("memory://workspace/Model.sysml").expect("uri");
        let mut graph = SemanticGraph::new();
        graph.pending_relationships.push(PendingRelationship {
            uri: uri.clone(),
            source_qualified: "P::A".to_string(),
            target_qualified: "P::Missing".to_string(),
            kind: RelationshipKind::Typing,
            target_kinds: None,
        });
        graph
            .pending_expression_relationships
            .push(PendingExpressionRelationship {
                uri: uri.clone(),
                source_expression: "a.out".to_string(),
                target_expression: "b.in".to_string(),
                kind: RelationshipKind::Connection,
                container_prefix: Some("P".to_string()),
                source_range: TextRange::new(TextPosition::new(2, 4), TextPosition::new(2, 20)),
            });

        let diagnostics =
            collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default());
        assert!(
            diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code == "unresolved_pending_relationship"),
            "expected unresolved pending relationship diagnostic: {diagnostics:?}"
        );
        assert!(
            diagnostics.iter().any(|diagnostic| {
                diagnostic.code == "unresolved_pending_expression_relationship"
                    && diagnostic.range.start.line == 2
            }),
            "expected unresolved pending expression diagnostic: {diagnostics:?}"
        );
    }
}
