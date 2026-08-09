use sysml_model::semantic::kinds::is_namespace;
use sysml_model::{
    build_graph_from_doc, build_semantic_graph_from_documents, patch_graph_for_document,
    resolve_import_target, resolve_imported_node_ids_for_simple_name, ElementKind,
    ImportTargetResolution, SysmlDocument, SysmlDocumentSourceKind,
};
use url::Url;

fn graph(source: &str) -> sysml_model::SemanticGraph {
    let uri = Url::parse("file:///import-target-resolution.sysml").expect("URI");
    let parsed = sysml_v2_parser::parse(source).expect("parser-backed source");
    build_graph_from_doc(&parsed, &uri)
}

fn document(path: &str, source: &str) -> SysmlDocument {
    SysmlDocument::from_memory_path(
        "typed-import-resolution",
        path,
        source.to_string(),
        SysmlDocumentSourceKind::Workspace,
        None,
        None,
    )
    .expect("document")
}

fn namespace<'a>(
    graph: &'a sysml_model::SemanticGraph,
    name: &str,
) -> &'a sysml_model::SemanticNode {
    graph
        .nodes_by_uri
        .values()
        .flatten()
        .filter_map(|id| graph.get_node(id))
        .find(|node| node.element_kind == ElementKind::Package && node.name == name)
        .unwrap_or_else(|| panic!("missing namespace {name}"))
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

    let filtered =
        graph("package Source { part def Item; } package Client { import Source [ 1 ]; }");
    assert!(matches!(
        resolve_import_target(&filtered, imports(&filtered)[0]),
        ImportTargetResolution::UnsupportedFiltered
    ));
}

#[test]
fn parser_backed_feature_declaration_is_not_a_namespace_import_target() {
    let graph = graph("package P { feature item; import item::*; }");
    let feature = graph
        .nodes_by_uri
        .values()
        .flatten()
        .filter_map(|id| graph.get_node(id))
        .find(|node| node.name == "item")
        .expect("parser-backed feature declaration");
    assert!(
        !is_namespace(&feature.element_kind),
        "a feature declaration must not become a namespace through an opaque KerML role"
    );
    assert!(
        !is_namespace(&ElementKind::KermlDecl),
        "the opaque KerML declaration bucket also contains feature declarations"
    );
    let import = imports(&graph)[0];
    assert!(matches!(
        resolve_import_target(&graph, import),
        ImportTargetResolution::Resolved { target } if target == feature.id
    ));
}

#[test]
fn same_and_cross_source_ambiguities_have_canonical_candidates() {
    let same_source =
        graph("package Source {} package Client { package Source {} import Source::*; }");
    let same = resolve_import_target(&same_source, imports(&same_source)[0]);
    assert!(
        matches!(same, ImportTargetResolution::Ambiguous { ref candidates } if candidates.len() == 2),
        "expected same-source ambiguity, got {same:?}"
    );

    let left = document("left.sysml", "package Source {}");
    let right = document("right.sysml", "package Source {}");
    let client = document("client.sysml", "package Client { import Source::*; }");
    let client_uri = client.uri.clone();
    let (cross_source, _) =
        build_semantic_graph_from_documents(&[left, right, client]).expect("cross-source graph");
    let import = cross_source
        .nodes_for_uri(&client_uri)
        .into_iter()
        .find(|node| node.element_kind == ElementKind::Import)
        .expect("client import");
    let ImportTargetResolution::Ambiguous { candidates } =
        resolve_import_target(&cross_source, import)
    else {
        panic!("expected cross-source ambiguity");
    };
    assert_eq!(candidates.len(), 2);
    assert!(
        candidates
            .windows(2)
            .all(|pair| (pair[0].uri.as_str(), &pair[0].qualified_name)
                <= (pair[1].uri.as_str(), &pair[1].qualified_name)),
        "candidate identities must have canonical URI/name ordering: {candidates:?}"
    );
}

#[test]
fn public_reexport_is_visible_while_private_and_protected_are_not() {
    for (visibility, is_visible) in [
        ("public ", true),
        ("private ", false),
        ("protected ", false),
    ] {
        let source = format!(
            "package Original {{ part def Item; }} package Middle {{ {visibility}import Original::*; }} package Client {{ import Middle::*; }}"
        );
        let graph = graph(&source);
        let client = namespace(&graph, "Client");
        let visible = resolve_imported_node_ids_for_simple_name(&graph, client, "Item");
        assert_eq!(
            !visible.is_empty(),
            is_visible,
            "{visibility:?} re-export visibility must use the canonical membership visibility"
        );
    }
}

#[test]
fn recursive_import_cycles_terminate_without_changing_results() {
    let graph = graph(
        "package A { part def Item; public import B::*; } package B { public import A::*; } package Client { import B::**; }",
    );
    let client = namespace(&graph, "Client");
    let first = resolve_imported_node_ids_for_simple_name(&graph, client, "Item");
    let warm = resolve_imported_node_ids_for_simple_name(&graph, client, "Item");
    assert_eq!(first, warm, "warm import cache must preserve cycle results");
    assert!(first.iter().any(|id| id.qualified_name == "A::Item"));
}

#[test]
fn full_and_incremental_import_resolution_have_cache_invalidation_parity() {
    let original = document(
        "incremental.sysml",
        "package Source { part def Old; } package Client { import Source::*; }",
    );
    let uri = original.uri.clone();
    let (mut graph, _) = build_semantic_graph_from_documents(&[original]).expect("initial graph");
    let initial_context = namespace(&graph, "Client").clone();
    let cold = resolve_imported_node_ids_for_simple_name(&graph, &initial_context, "Old");
    let warm = resolve_imported_node_ids_for_simple_name(&graph, &initial_context, "Old");
    assert_eq!(cold, warm, "cold/warm resolution parity");
    assert!(!cold.is_empty());

    let updated = "package Source { part def New; } package Client { import Source::*; }";
    let parsed = sysml_v2_parser::parse_for_editor(updated).root;
    patch_graph_for_document(&mut graph, &uri, Some(&parsed), true);
    let updated_context = namespace(&graph, "Client").clone();
    assert!(resolve_imported_node_ids_for_simple_name(&graph, &updated_context, "Old").is_empty());
    let incremental = resolve_imported_node_ids_for_simple_name(&graph, &updated_context, "New");

    let rebuilt = document("incremental.sysml", updated);
    let (full, _) = build_semantic_graph_from_documents(&[rebuilt]).expect("rebuilt graph");
    let full_context = namespace(&full, "Client");
    assert_eq!(
        incremental,
        resolve_imported_node_ids_for_simple_name(&full, full_context, "New"),
        "incremental cache invalidation must match the full graph"
    );
}

#[test]
fn expose_keeps_its_distinct_typed_origin_and_resolver_contract() {
    let graph = graph("package P { part vehicle; view v { expose P::vehicle; } }");
    let expose = imports(&graph)
        .into_iter()
        .find(|node| {
            node.declared_facts
                .membership
                .as_ref()
                .and_then(|membership| membership.import.as_ref())
                .is_some_and(|facts| facts.origin == sysml_model::ImportOrigin::Expose)
        })
        .expect("parser-backed expose membership");
    assert!(
        graph.effective_membership_visibility_for(expose).is_none(),
        "Expose must not inherit Import's private default"
    );
    assert!(matches!(
        resolve_import_target(&graph, expose),
        ImportTargetResolution::NotApplicable
    ));
}
