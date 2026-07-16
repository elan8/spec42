//! Orchestrates semantic graph materialize → link → pending resolve.

use std::collections::HashSet;
use std::time::Instant;

use rayon::prelude::*;
use url::Url;

use crate::semantic::analysis_typing::prepare_analysis_evaluation_context;
use crate::semantic::evaluation::evaluate_expressions;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::graph_builder::build_graph_from_doc;
use crate::semantic::library_loader::declared_packages_from_parsed;
use crate::semantic::model::SemanticEdge;
use crate::semantic::relationships::{
    add_cross_document_edges_for_uri, add_semantic_edge_once, link_workspace_derivations,
    link_workspace_relationships, rebuild_static_dependency_index, refresh_relationship_frontier,
    resolve_cross_document_edges_for_uri, resolve_workspace_pending_relationships,
    update_static_dependency_targets_for_uri,
};
use crate::semantic::source::{SysmlDocument, SysmlDocumentSourceKind};
use crate::semantic::workspace_graph::WorkspaceParsedDocument;

/// A parsed document paired with the source kind (workspace/library/external) needed to
/// decide how it merges — see [`link_parsed_documents_parallel`].
type SourceTaggedDocument = (SysmlDocumentSourceKind, WorkspaceParsedDocument);

/// Build, merge, link, and resolve pending relationships for pre-loaded documents.
pub fn build_and_link_graph(
    documents: &[SysmlDocument],
) -> Result<(SemanticGraph, Vec<WorkspaceParsedDocument>), String> {
    let mut graph = SemanticGraph::new();
    let mut parsed_docs = Vec::new();

    let mut workspace_docs = Vec::new();
    let mut library_docs = Vec::new();
    for document in documents {
        match document.source_kind {
            SysmlDocumentSourceKind::Library => library_docs.push(document),
            SysmlDocumentSourceKind::Workspace | SysmlDocumentSourceKind::External => {
                workspace_docs.push(document)
            }
        }
    }

    let mut workspace_packages = HashSet::new();

    for document in workspace_docs {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        workspace_packages.extend(declared_packages_from_parsed(&parsed));
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        let doc_graph = build_graph_from_doc(&parsed, &document.uri);
        graph.merge(doc_graph);
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    for document in library_docs {
        let parse_start = Instant::now();
        let Ok(parsed) = sysml_v2_parser::parse(&document.content) else {
            continue;
        };
        let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
        let doc_graph = build_graph_from_doc(&parsed, &document.uri);
        graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        parsed_docs.push(WorkspaceParsedDocument {
            uri: document.uri.clone(),
            content: document.content.clone(),
            parsed,
            parse_time_ms,
            parse_cached: false,
        });
    }

    finalize_and_evaluate(&mut graph);

    Ok((graph, parsed_docs))
}

fn parse_document(document: &SysmlDocument) -> Option<WorkspaceParsedDocument> {
    let parse_start = Instant::now();
    let parsed = sysml_v2_parser::parse(&document.content).ok()?;
    let parse_time_ms = parse_start.elapsed().as_millis().max(1) as u32;
    Some(WorkspaceParsedDocument {
        uri: document.uri.clone(),
        content: document.content.clone(),
        parsed,
        parse_time_ms,
        parse_cached: false,
    })
}

/// Parses, builds, and links a semantic graph from many documents in parallel — the
/// full-workspace equivalent of [`patch_graph_for_document`]. Same end result as
/// [`build_and_link_graph`] (same nodes, same edges), computed differently: parsing runs in
/// parallel, then [`link_parsed_documents_parallel`] does the rest — see its doc comment for
/// the merge/link phases.
///
/// See `docs/engineering/TIER2-PHASE3B-STEP5-FULL-REBUILD-DESIGN.md` for why this exists
/// and the equivalence testing this function's own test module is expected to carry.
pub fn build_and_link_graph_parallel(
    documents: &[SysmlDocument],
) -> (SemanticGraph, Vec<WorkspaceParsedDocument>) {
    let entries: Vec<SourceTaggedDocument> = documents
        .par_iter()
        .filter_map(|document| parse_document(document).map(|entry| (document.source_kind, entry)))
        .collect();
    link_parsed_documents_parallel(entries, true)
}

/// Merges and links already-parsed documents in parallel — the merge/link half of
/// [`build_and_link_graph_parallel`], factored out so a caller that already has parsed
/// documents (e.g. served from a disk parse cache, or an editor's live in-memory index) can
/// skip the parse step `build_and_link_graph_parallel` otherwise always does internally.
///
/// Phases: workspace documents' graphs are built and merged first (parallel), since library
/// merging needs the complete set of workspace-declared package names to avoid shadowing;
/// library documents are merged second, skipping anything the workspace already declared.
/// Cross-document edges are then resolved via parallel per-URI resolution (see
/// [`link_workspace_derivations`]'s doc comment) instead of the sequential whole-graph scan
/// inside [`link_workspace_relationships`].
///
/// Starts from an empty graph. Use [`link_parsed_documents_parallel_from`] to merge onto an
/// existing graph instead (e.g. a cached library subgraph).
///
/// `evaluate: false` skips `evaluate_expressions` (structural relink only — typing/specializes/
/// subject/derivation resolution still run); pass `true` for the same behavior as before this
/// parameter existed. See [`evaluate_workspace_graph`] to run evaluation as a separate, later
/// step on a graph built with `evaluate: false` — this is what lets a caller (e.g. `lsp_server`'s
/// live-edit relink) publish structural diagnostics immediately and defer the more expensive
/// evaluation pass without blocking on it.
pub fn link_parsed_documents_parallel(
    documents: Vec<SourceTaggedDocument>,
    evaluate: bool,
) -> (SemanticGraph, Vec<WorkspaceParsedDocument>) {
    link_parsed_documents_parallel_from(SemanticGraph::new(), documents, evaluate)
}

/// [`link_parsed_documents_parallel`], but merging `documents` onto `base_graph` instead of
/// starting from an empty graph — for callers that already have part of the graph built
/// (typically a cached library subgraph) and only want to merge/link the rest. Cross-document
/// edge resolution covers `base_graph`'s existing URIs as well as `documents`', so a document
/// being merged can still resolve references into whatever was already in `base_graph`.
pub fn link_parsed_documents_parallel_from(
    base_graph: SemanticGraph,
    documents: Vec<SourceTaggedDocument>,
    evaluate: bool,
) -> (SemanticGraph, Vec<WorkspaceParsedDocument>) {
    let (workspace_entries, library_entries): (
        Vec<SourceTaggedDocument>,
        Vec<SourceTaggedDocument>,
    ) = documents
        .into_iter()
        .partition(|(kind, _)| !matches!(kind, SysmlDocumentSourceKind::Library));

    let mut uris: Vec<Url> = base_graph.all_uris();
    let mut graph = base_graph;
    let mut parsed_docs = Vec::new();

    // Phase 1: workspace documents. Must finish (and its declared-package set must be
    // complete) before phase 2 starts.
    let workspace_built: Vec<(SemanticGraph, WorkspaceParsedDocument)> = workspace_entries
        .into_par_iter()
        .map(|(_, entry)| {
            let doc_graph = build_graph_from_doc(&entry.parsed, &entry.uri);
            (doc_graph, entry)
        })
        .collect();
    let workspace_packages: HashSet<String> = workspace_built
        .iter()
        .flat_map(|(_, entry)| declared_packages_from_parsed(&entry.parsed))
        .collect();
    for (doc_graph, entry) in workspace_built {
        uris.push(entry.uri.clone());
        graph.merge(doc_graph);
        parsed_docs.push(entry);
    }

    // Phase 2: library documents, merged skipping anything the workspace already declared.
    let library_built: Vec<(SemanticGraph, WorkspaceParsedDocument)> = library_entries
        .into_par_iter()
        .map(|(_, entry)| {
            let doc_graph = build_graph_from_doc(&entry.parsed, &entry.uri);
            (doc_graph, entry)
        })
        .collect();
    for (doc_graph, entry) in library_built {
        uris.push(entry.uri.clone());
        graph.merge_skip_existing_qualified_names(doc_graph, &workspace_packages);
        parsed_docs.push(entry);
    }

    // Parallel cross-document edge resolution, replacing the sequential typing/
    // specializes/subject scan inside `link_workspace_relationships`.
    let resolved_edges: Vec<_> = uris
        .par_iter()
        .flat_map(|uri| resolve_cross_document_edges_for_uri(&graph, uri))
        .collect();
    for (src_id, tgt_id, kind) in resolved_edges {
        // `resolve_cross_document_edges_for_uri` resolves typing/specializes/subject refs
        // for every node in the URI, not just ones whose target lives in another document —
        // for a same-document reference, `build_graph_from_doc` may already have wired the
        // identical edge. Use `add_semantic_edge_once` (not a raw `add_edge`) so this phase
        // dedupes the same way `link_workspace_relationships`'s per-node loop does.
        add_semantic_edge_once(&mut graph, &src_id, &tgt_id, SemanticEdge::plain(kind));
    }
    graph.invalidate_query_indexes();

    link_workspace_derivations(&mut graph);
    prepare_analysis_evaluation_context(&mut graph);
    resolve_workspace_pending_relationships(&mut graph);
    if evaluate {
        evaluate_expressions(&mut graph);
    }
    graph.invalidate_query_indexes();
    // See the matching comment in `finalize_and_evaluate` — this path resolves cross-document
    // edges via `add_semantic_edge_once`, not `add_cross_document_edges_for_uri`, so the
    // relationship-frontier index needs an explicit rebuild here too.
    rebuild_static_dependency_index(&mut graph);

    (graph, parsed_docs)
}

/// Runs expression evaluation on `graph` in place and invalidates query indexes — the
/// deferred "Wave 2" step for a graph previously built with `evaluate: false` (see
/// [`link_parsed_documents_parallel_from`]). Exists so callers outside this crate don't need
/// to reach into `semantic::evaluation` internals directly for what is otherwise exactly
/// `finalize_and_evaluate`'s evaluation half.
pub fn evaluate_workspace_graph(graph: &mut SemanticGraph) {
    evaluate_expressions(graph);
    graph.invalidate_query_indexes();
}

/// Link, prepare analysis context, and resolve pending relationships after graph mutation.
pub fn finalize_workspace_graph(graph: &mut SemanticGraph) {
    link_workspace_relationships(graph);
    prepare_analysis_evaluation_context(graph);
    resolve_workspace_pending_relationships(graph);
    // Edge additions above go via graph.graph.add_edge() directly, bypassing
    // insert_workspace_edge. Invalidate here so the first post-finalization query
    // builds the edge index with all edges present.
    graph.invalidate_query_indexes();
}

/// [`finalize_workspace_graph`] plus expression evaluation. Use this (not
/// `finalize_workspace_graph` directly) wherever a graph needs to end up in a fully
/// up-to-date, query-ready state — i.e. after a full build or a settled incremental
/// update. Callers that want a fast, low-latency patch during rapid live edits (deferring
/// evaluation to a later catch-up pass) should call `finalize_workspace_graph` directly
/// instead, the same way `patch_graph_for_document`'s `evaluate: false` path does.
pub fn finalize_and_evaluate(graph: &mut SemanticGraph) {
    finalize_workspace_graph(graph);
    evaluate_expressions(graph);
    graph.invalidate_query_indexes();
    // Whole-graph relink doesn't go through `add_cross_document_edges_for_uri`'s incremental
    // index maintenance, so rebuild the relationship-frontier index from scratch here. Cheap
    // (O(all nodes' attributes)) relative to the relink/evaluate work above. The scoped
    // `finalize_and_evaluate_frontier` path deliberately does NOT do this — it relies on
    // incremental maintenance to stay cheap.
    rebuild_static_dependency_index(graph);
}

/// Patches `graph` in place for a single changed document: removes that document's
/// existing nodes, rebuilds and merges its subgraph (if parsed content is provided), and
/// refreshes cross-document edges touching it.
///
/// When `evaluate` is `true`, also relinks workspace relationships, resolves pending
/// relationships, and re-evaluates expressions across the graph (via
/// [`finalize_and_evaluate`]). Pass `false` to skip those steps for a fast, low-latency
/// patch (e.g. on every keystroke) and call [`finalize_and_evaluate`] later once edits
/// settle.
pub fn patch_graph_for_document(
    graph: &mut SemanticGraph,
    uri: &Url,
    parsed: Option<&sysml_v2_parser::RootNamespace>,
    evaluate: bool,
) {
    graph.remove_nodes_for_uri(uri);
    let Some(parsed) = parsed else {
        return;
    };
    let doc_graph = build_graph_from_doc(parsed, uri);
    graph.merge(doc_graph);
    update_static_dependency_targets_for_uri(graph, uri);
    add_cross_document_edges_for_uri(graph, uri);
    if evaluate {
        finalize_and_evaluate(graph);
    }
}

/// [`finalize_and_evaluate`], but scoped: relinks only `changed_uri` and the other URIs whose
/// own content statically depends on it (via [`refresh_relationship_frontier`]) instead of the
/// whole-graph [`link_workspace_relationships`]. Expression evaluation and pending-
/// relationship resolution remain whole-graph — see the Track B Phase 1 plan
/// (`docs/engineering/`) for the reasoning and the explicit non-goals this leaves for a
/// follow-up. Relies on `graph.document_dependents` being incrementally maintained rather than
/// rebuilt from scratch — do not call this on a graph that only ever went through the
/// whole-graph paths without first confirming the index is populated (it always is after any
/// full build or prior `patch_graph_for_document`/`patch_graph_for_document_scoped` call).
pub fn finalize_and_evaluate_frontier(graph: &mut SemanticGraph, changed_uri: &Url) {
    refresh_relationship_frontier(graph, changed_uri);
    prepare_analysis_evaluation_context(graph);
    resolve_workspace_pending_relationships(graph);
    graph.invalidate_query_indexes();
    evaluate_expressions(graph);
    graph.invalidate_query_indexes();
}

/// [`patch_graph_for_document`], but using [`finalize_and_evaluate_frontier`] instead of
/// [`finalize_and_evaluate`] when `evaluate` is `true` — scopes relationship relinking to the
/// affected frontier instead of the whole graph. Opt-in sibling function rather than a change
/// to `patch_graph_for_document`'s existing behavior/callers: wire real callers
/// (`IncrementalWorkspace::apply_document`, `try_incremental_update`) to this only once the
/// differential correctness tests and benchmark in the Track B Phase 1 plan confirm it's both
/// correct and actually faster — do not assume either.
pub fn patch_graph_for_document_scoped(
    graph: &mut SemanticGraph,
    uri: &Url,
    parsed: Option<&sysml_v2_parser::RootNamespace>,
    evaluate: bool,
) {
    graph.remove_nodes_for_uri(uri);
    let Some(parsed) = parsed else {
        return;
    };
    let doc_graph = build_graph_from_doc(parsed, uri);
    graph.merge(doc_graph);
    update_static_dependency_targets_for_uri(graph, uri);
    add_cross_document_edges_for_uri(graph, uri);
    if evaluate {
        finalize_and_evaluate_frontier(graph, uri);
    }
}

#[cfg(test)]
mod tests;
