use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;
use walkdir::WalkDir;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::relationships::add_cross_document_edges_for_uri;

#[derive(Debug)]
pub struct WorkspaceParsedDocument {
    pub uri: Url,
    pub content: String,
    pub parsed: RootNamespace,
    pub parse_time_ms: u32,
    pub parse_cached: bool,
}

fn path_to_url(path: &Path) -> Result<Url, String> {
    let absolute = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|err| format!("failed to resolve current directory: {err}"))?
            .join(path)
    };
    let canonical = std::fs::canonicalize(&absolute).unwrap_or(absolute);
    Url::from_file_path(&canonical)
        .map_err(|_| format!("failed to convert path to file URI: {}", canonical.display()))
}

fn resolve_workspace_root(target: &Path, workspace_root: Option<&Path>) -> PathBuf {
    workspace_root.map(Path::to_path_buf).unwrap_or_else(|| {
        if target.is_dir() {
            target.to_path_buf()
        } else {
            target
                .parent()
                .map(Path::to_path_buf)
                .unwrap_or_else(|| PathBuf::from("."))
        }
    })
}

/// Build a merged semantic graph from all `.sysml` files under workspace and library roots.
///
/// Returns the merged graph and parsed documents used to build it.
pub fn build_semantic_graph_for_paths(
    target: &Path,
    workspace_root: Option<&Path>,
    library_paths: &[PathBuf],
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let workspace_root = resolve_workspace_root(target, workspace_root);
    let roots = std::iter::once(workspace_root)
        .chain(library_paths.iter().cloned())
        .collect::<Vec<_>>();

    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    for root in roots {
        if !root.exists() {
            continue;
        }
        for entry in WalkDir::new(&root).into_iter().filter_map(Result::ok) {
            let path = entry.path();
            if !entry.file_type().is_file() {
                continue;
            }
            if !path
                .extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("sysml"))
            {
                continue;
            }
            let Ok(content) = fs::read_to_string(path) else {
                continue;
            };
            let parse_start = Instant::now();
            let Ok(parsed) = sysml_v2_parser::parse(&content) else {
                continue;
            };
            let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
            let uri = path_to_url(path)?;
            graph.merge(build_graph_from_doc(&parsed, &uri));
            parsed_docs.push(WorkspaceParsedDocument {
                uri,
                content,
                parsed,
                parse_time_ms,
                parse_cached: false,
            });
        }
    }

    for doc in &parsed_docs {
        add_cross_document_edges_for_uri(&mut graph, &doc.uri);
    }

    Ok((graph, parsed_docs))
}
