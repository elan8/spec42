//! Regression coverage proving `TransitionEndpointFacts` (the typed replacement for the removed
//! `source`/`target` JSON attributes on `Transition` nodes) drives
//! `sysml_diagnostics::behavior_conformance` for both a resolved and an unresolved transition
//! endpoint. See `ROUNDTRIP_SEMGRAPH_PREREQS.md` section B9.

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticSeverity, DiagnosticsOptions};
use sysml_model::{
    build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind, TextRange,
};
use tempfile::tempdir;
use url::Url;

fn file_doc(filename: &str, content: &str) -> SysmlDocument {
    let dir = tempdir().expect("tempdir");
    let path = dir.path().join(filename);
    std::fs::write(&path, content).expect("write sysml");
    let uri = Url::from_file_path(&path).expect("file uri");
    std::mem::forget(dir);
    SysmlDocument::from_uri(
        uri.as_str(),
        content.to_string(),
        Some(filename.to_string()),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document")
}

fn diagnostics_for(content: &str) -> Vec<sysml_diagnostics::SemanticDiagnostic> {
    let doc = file_doc("transition-endpoints.sysml", content);
    let uri = doc.uri.clone();
    let (graph, _parsed) = build_semantic_graph_from_documents(&[doc]).expect("graph");
    collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
}

fn diagnostic_for(content: &str, code: &str) -> sysml_diagnostics::SemanticDiagnostic {
    diagnostics_for(content)
        .into_iter()
        .find(|diagnostic| diagnostic.code == code)
        .unwrap_or_else(|| panic!("missing diagnostic {code} for content:\n{content}"))
}

/// A transition endpoint that resolves to a non-state element: `TransitionEndpointFacts`'s
/// `target_expression` must reach `behavior_conformance::resolve_qualified_endpoint` and produce
/// the stable `transition_endpoint_invalid_state` diagnostic at the transition's own range.
#[test]
fn resolved_transition_endpoint_of_wrong_kind_flags_invalid_state() {
    let content = "package Demo {\n\
        \x20   state def Light {\n\
        \x20       state on;\n\
        \x20       requirement req1;\n\
        \x20       transition t1\n\
        \x20           first on\n\
        \x20           then req1;\n\
        \x20   }\n\
        }\n";
    let diagnostic = diagnostic_for(content, "transition_endpoint_invalid_state");
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Warning);
    assert_eq!(
        diagnostic.range,
        TextRange {
            start: sysml_model::TextPosition {
                line: 4,
                character: 8
            },
            end: sysml_model::TextPosition {
                line: 4,
                character: 65
            },
        }
    );
    assert!(diagnostic.message.contains("must resolve to state usages"));
}

/// A transition endpoint that names no element at all: the typed `target_expression` still
/// reaches edge construction, which reports the unresolved reference through the pending
/// relationship diagnostic rather than silently dropping it or masquerading as success.
#[test]
fn unresolved_transition_endpoint_flags_pending_relationship() {
    let content = "package Demo {\n\
        \x20   state def Light {\n\
        \x20       state on;\n\
        \x20       transition t1\n\
        \x20           first on\n\
        \x20           then Missing;\n\
        \x20   }\n\
        }\n";
    let diagnostic = diagnostic_for(content, "unresolved_pending_relationship");
    assert_eq!(diagnostic.severity, DiagnosticSeverity::Error);
    assert_eq!(
        diagnostic.range,
        TextRange {
            start: sysml_model::TextPosition {
                line: 0,
                character: 0
            },
            end: sysml_model::TextPosition {
                line: 0,
                character: 0
            },
        }
    );
    // The wrong-kind check must not also fire: the endpoint never resolved to any node.
    assert!(!diagnostics_for(content)
        .iter()
        .any(|d| d.code == "transition_endpoint_invalid_state"));
}
