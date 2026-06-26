//! Host validation and projection assembly from a built semantic graph.

use std::collections::{BTreeSet, HashMap};

use sysml_model::{
    collect_diagnostics_from_graph_with_unit_registry, collect_untyped_part_usage_diagnostics,
    missing_library_context_diagnostic, DiagnosticSeverity, DiagnosticsOptions, SemanticDiagnostic,
    SemanticGraph, SysmlDocument, UnitRegistry,
};
use sysml_v2_parser::DiagnosticSeverity as ParseSeverity;
use url::Url;

use super::discovery::path_to_file_url;
use super::projection::{
    HostSemanticModelNode, HostSemanticModelRelationship, HostSemanticProjection,
};
use super::validation::{
    HostValidatedDocument, HostValidationReport, HostValidationSummary,
};

pub(crate) fn collect_host_validation_report(
    graph: &SemanticGraph,
    documents: &[SysmlDocument],
    library_urls: &[Url],
    target_files: &[std::path::PathBuf],
    workspace_root: Option<&std::path::Path>,
    library_paths_display: &[std::path::PathBuf],
    strict_diagnostics: bool,
) -> crate::error::WorkspaceResult<HostValidationReport> {
    let target_urls = target_file_urls(target_files)?;
    let unit_registry = UnitRegistry::from_graph(graph);
    let document_text: HashMap<&str, &str> = documents
        .iter()
        .map(|doc| (doc.uri.as_str(), doc.content.as_str()))
        .collect();
    let mut host_documents = Vec::new();

    for uri in &target_urls {
        let text = document_text.get(uri.as_str()).copied().unwrap_or("");
        let diagnostics = collect_host_document_diagnostics(
            graph,
            &unit_registry,
            library_urls,
            uri,
            text,
            strict_diagnostics,
        );
        host_documents.push(HostValidatedDocument {
            uri: uri.to_string(),
            diagnostics,
        });
    }

    Ok(HostValidationReport {
        workspace_root: workspace_root.map(|path| path.display().to_string()),
        resolved_library_paths: library_paths_display
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        documents: host_documents.clone(),
        summary: summarize_host_documents(&host_documents),
    })
}

pub(crate) fn project_host_semantic_model(
    graph: &SemanticGraph,
    target_files: &[std::path::PathBuf],
) -> crate::error::WorkspaceResult<HostSemanticProjection> {
    let target_urls = target_file_urls(target_files)?;
    let mut nodes = Vec::new();
    for uri in &target_urls {
        for node in graph.nodes_for_uri(uri) {
            nodes.push(HostSemanticModelNode {
                uri: node.id.uri.to_string(),
                qualified_name: node.id.qualified_name.clone(),
                name: node.name.clone(),
                element_kind: node.element_kind.clone(),
                range: node.range,
                parent: node
                    .parent_id
                    .as_ref()
                    .map(|parent| parent.qualified_name.clone()),
                attributes: node.attributes.clone(),
            });
        }
    }
    nodes.sort_by(|a, b| {
        a.uri
            .cmp(&b.uri)
            .then_with(|| a.qualified_name.cmp(&b.qualified_name))
            .then_with(|| a.element_kind.as_str().cmp(b.element_kind.as_str()))
    });

    let mut relationships = Vec::new();
    for uri in &target_urls {
        for (src_id, tgt_id, edge) in graph.edges_for_uri(uri) {
            relationships.push(HostSemanticModelRelationship {
                source: src_id.qualified_name,
                target: tgt_id.qualified_name,
                kind: edge.kind,
                connect: edge.connect,
            });
        }
    }
    relationships.sort_by(|a, b| {
        a.source
            .cmp(&b.source)
            .then_with(|| a.target.cmp(&b.target))
            .then_with(|| a.kind.as_str().cmp(b.kind.as_str()))
    });
    relationships.dedup_by(|a, b| {
        a.source == b.source && a.target == b.target && a.kind == b.kind
    });

    Ok(HostSemanticProjection {
        nodes,
        relationships,
    })
}

fn target_file_urls(target_files: &[std::path::PathBuf]) -> crate::error::WorkspaceResult<BTreeSet<Url>> {
    target_files
        .iter()
        .map(|path| path_to_file_url(path.as_path()))
        .collect::<Result<BTreeSet<_>, _>>()
}

fn collect_host_document_diagnostics(
    graph: &SemanticGraph,
    unit_registry: &UnitRegistry,
    library_urls: &[Url],
    uri: &Url,
    text: &str,
    strict_diagnostics: bool,
) -> Vec<SemanticDiagnostic> {
    let mut diagnostics = parse_diagnostics(uri, text);
    diagnostics.extend(collect_untyped_part_usage_diagnostics(uri, text));

    let has_parse_error = diagnostics.iter().any(|diagnostic| {
        diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
    });
    let allow_semantic = if strict_diagnostics {
        !has_parse_error
    } else {
        true
    };

    if allow_semantic {
        diagnostics.extend(collect_diagnostics_from_graph_with_unit_registry(
            graph,
            uri,
            DiagnosticsOptions::default(),
            unit_registry,
        ));

        let has_unresolved_type_reference = has_semantic_code(&diagnostics, "unresolved_type_reference");
        let has_unresolved_import_target = has_semantic_code(&diagnostics, "unresolved_import_target");
        let has_unresolved_specializes_reference =
            has_semantic_code(&diagnostics, "unresolved_specializes_reference");

        if let Some(diagnostic) = missing_library_context_diagnostic(
            uri,
            text,
            has_unresolved_type_reference
                || has_unresolved_import_target
                || has_unresolved_specializes_reference,
            !library_urls.is_empty(),
        ) {
            diagnostics.push(diagnostic);
        }
    }

    if strict_diagnostics && has_parse_error {
        diagnostics.retain(|diagnostic| {
            diagnostic.severity == DiagnosticSeverity::Error && diagnostic.source == "sysml"
        });
    }

    diagnostics
}

fn parse_diagnostics(uri: &Url, text: &str) -> Vec<SemanticDiagnostic> {
    let result = sysml_v2_parser::parse_with_diagnostics(text);

    result
        .errors
        .into_iter()
        .map(|error| {
            let severity = match error.severity.unwrap_or(ParseSeverity::Error) {
                ParseSeverity::Error => DiagnosticSeverity::Error,
                ParseSeverity::Warning => DiagnosticSeverity::Warning,
            };
            SemanticDiagnostic {
                uri: uri.clone(),
                range: error
                    .to_lsp_range()
                    .map(|(sl, sc, el, ec)| {
                        sysml_model::TextRange::new(
                            sysml_model::TextPosition::new(sl, sc),
                            sysml_model::TextPosition::new(el, ec),
                        )
                    })
                    .unwrap_or_else(|| {
                        sysml_model::TextRange::new(
                            sysml_model::TextPosition::new(0, 0),
                            sysml_model::TextPosition::new(0, 0),
                        )
                    }),
                severity,
                source: "sysml".to_string(),
                code: error.code.unwrap_or_else(|| "parse_error".to_string()),
                message: error.message,
                related_information: Vec::new(),
            }
        })
        .collect()
}

fn has_semantic_code(diagnostics: &[SemanticDiagnostic], code: &str) -> bool {
    diagnostics.iter().any(|diagnostic| diagnostic.code == code)
}

fn summarize_host_documents(documents: &[HostValidatedDocument]) -> HostValidationSummary {
    let mut summary = HostValidationSummary {
        document_count: documents.len(),
        ..HostValidationSummary::default()
    };
    for document in documents {
        for diagnostic in &document.diagnostics {
            match diagnostic.severity {
                DiagnosticSeverity::Error => summary.error_count += 1,
                DiagnosticSeverity::Warning => summary.warning_count += 1,
                DiagnosticSeverity::Information | DiagnosticSeverity::Hint => {
                    summary.information_count += 1
                }
            }
        }
    }
    summary
}
