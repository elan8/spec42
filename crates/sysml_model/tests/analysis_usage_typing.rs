use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, SysmlDocument, SysmlDocumentSourceKind};

const LIBRARY_SYSML: &str = r#"
package GridAnalysis {
    part def PowerSystem;

    analysis def LoadFlowAnalysis {
        subject powerSystem : PowerSystem;
        return ref loadFlowComplete {
            return true;
        }
    }

    analysis def VoltageDropAnalysis :> LoadFlowAnalysis {
        return ref voltageDropComplete {
            return true;
        }
    }
}
"#;

const SAME_FILE_SYSML: &str = r#"
package LocalCases {
    analysis def QuickStudy {
        return ref done { return true; }
    }
    analysis quickRun : QuickStudy;
}
"#;

const CROSS_FILE_SYSML: &str = r#"
package AnalysisCases {
    private import GridAnalysis::*;

    analysis voltageDropCheck : VoltageDropAnalysis;
    analysis loadFlowRun : LoadFlowAnalysis;
}
"#;

fn diags_for_documents(docs: &[SysmlDocument]) -> Vec<sysml_diagnostics::SemanticDiagnostic> {
    let (graph, _) = build_semantic_graph_from_documents(docs).expect("semantic graph");
    let uri = docs
        .last()
        .map(|doc| doc.uri.clone())
        .expect("at least one document");
    collect_diagnostics_from_graph(&graph, &uri, DiagnosticsOptions::default())
}

fn has_code(diags: &[sysml_diagnostics::SemanticDiagnostic], code: &str) -> bool {
    diags.iter().any(|d| d.code == code)
}

#[test]
fn analysis_usage_typed_by_local_analysis_def_resolves_without_unresolved_type_reference() {
    let doc = SysmlDocument::from_memory_path(
        "analysis-usage-typing",
        "LocalCases.sysml",
        SAME_FILE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let diags = diags_for_documents(&[doc]);
    assert!(
        !has_code(&diags, "unresolved_type_reference"),
        "expected analysis usage typing to resolve, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "unresolved_type_reference")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );
}

#[test]
fn analysis_usage_typed_by_imported_analysis_def_resolves_without_unresolved_type_reference() {
    let library = SysmlDocument::from_memory_path(
        "analysis-usage-typing",
        "GridAnalysis.sysml",
        LIBRARY_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("library uri");
    let consumer = SysmlDocument::from_memory_path(
        "analysis-usage-typing",
        "AnalysisCases.sysml",
        CROSS_FILE_SYSML.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("consumer uri");
    let consumer_uri = consumer.uri.clone();
    let (graph, _) =
        build_semantic_graph_from_documents(&[library, consumer]).expect("semantic graph");
    let diags =
        collect_diagnostics_from_graph(&graph, &consumer_uri, DiagnosticsOptions::default());
    assert!(
        !has_code(&diags, "unresolved_type_reference"),
        "expected imported analysis def typing to resolve, got {:?}",
        diags
            .iter()
            .filter(|d| d.code == "unresolved_type_reference")
            .map(|d| &d.message)
            .collect::<Vec<_>>()
    );

    let usage_node = graph
        .node_ids_by_qualified_name
        .get("AnalysisCases::voltageDropCheck")
        .and_then(|ids| ids.first())
        .and_then(|id| graph.get_node(id))
        .expect("voltageDropCheck node");
    let typing_targets = graph.outgoing_typing_or_specializes_targets(usage_node);
    assert!(
        typing_targets
            .iter()
            .any(|target| target.id.qualified_name.ends_with("VoltageDropAnalysis")),
        "expected typing edge to VoltageDropAnalysis, got {:?}",
        typing_targets
            .iter()
            .map(|node| node.id.qualified_name.as_str())
            .collect::<Vec<_>>()
    );
}

#[test]
fn analysis_usage_typed_by_part_def_emits_incompatible_type_kind() {
    let input = r#"
        package P {
            part def Engine;
            analysis study : Engine;
        }
    "#;
    let doc = SysmlDocument::from_memory_path(
        "analysis-usage-typing",
        "Invalid.sysml",
        input.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document uri");
    let diags = diags_for_documents(&[doc]);
    assert!(
        has_code(&diags, "incompatible_type_kind"),
        "expected incompatible_type_kind for analysis usage typed by part def, got {:?}",
        diags.iter().map(|d| &d.code).collect::<Vec<_>>()
    );
}
