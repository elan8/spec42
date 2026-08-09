use sysml_model::{
    build_graph_from_doc, resolve_import_target, ElementKind, ImportTargetResolution,
};
use url::Url;

fn graph(source: &str) -> sysml_model::SemanticGraph {
    let uri = Url::parse("file:///import-target-resolution.sysml").expect("URI");
    let parsed = sysml_v2_parser::parse(source).expect("parser-backed source");
    build_graph_from_doc(&parsed, &uri)
}

fn imports(graph: &sysml_model::SemanticGraph) -> Vec<&sysml_model::SemanticNode> {
    graph
        .nodes_by_uri
        .values()
        .flatten()
        .filter_map(|id| graph.get_node(id))
        .filter(|node| node.element_kind == ElementKind::Import)
        .collect()
}

#[test]
fn resolves_membership_and_namespace_imports_with_canonical_identity() {
    let graph = graph("package Source { part def Item; } package Client { import Source::Item; import Source::*; }");
    let results: Vec<_> = imports(&graph)
        .into_iter()
        .map(|import| resolve_import_target(&graph, import))
        .collect();
    assert!(results.iter().any(|result| matches!(
        result,
        ImportTargetResolution::Resolved { target }
            if target.qualified_name == "Source::Item"
    )));
    assert!(results.iter().any(|result| matches!(
        result,
        ImportTargetResolution::Resolved { target }
            if target.qualified_name == "Source"
    )));
}

#[test]
fn unresolved_and_filtered_imports_remain_distinct_statuses() {
    let unresolved = graph("package Client { import Missing::*; }");
    assert!(matches!(
        resolve_import_target(&unresolved, imports(&unresolved)[0]),
        ImportTargetResolution::Unresolved
    ));

    let filtered = graph("package Source { part def Item; } package Client { import Source [ 1 ]; }");
    assert!(matches!(
        resolve_import_target(&filtered, imports(&filtered)[0]),
        ImportTargetResolution::UnsupportedFiltered
    ));
}
