use std::path::Path;

use sysml_diagnostics::{collect_diagnostics_from_graph, DiagnosticsOptions};
use sysml_model::{build_semantic_graph_from_documents, build_view_catalog, build_workspace_graph_dto_for_uris, evaluate_views, resolve_expose_target, resolve_library_closure, ExposeTargetResolution, LibraryClosureOptions, SysmlDocument, SysmlDocumentSourceKind, WorkspaceSource};

const WEBSHOP_FILES: &[&str] = &[
    "webshop.sysml",
    "WebShopArchitecture.sysml",
    "WebShopBehavior.sysml",
    "WebShopRequirements.sysml",
    "Views.sysml",
];

type WebshopWorkspace = (
    Vec<SysmlDocument>,
    Vec<url::Url>,
    sysml_model::SemanticGraph,
    Vec<sysml_model::WorkspaceParsedDocument>,
);

fn load_webshop_workspace() -> Option<WebshopWorkspace> {
    build_webshop_documents(false)
}

fn build_webshop_documents(include_domain_libraries: bool) -> Option<WebshopWorkspace> {
    let workspace_root = Path::new(r"C:\Git\sysml-examples\webshop");
    if !workspace_root.is_dir() {
        return None;
    }

    let mut file_contents = Vec::new();
    let mut documents = Vec::new();
    let mut uris = Vec::new();

    for name in WEBSHOP_FILES {
        let path = workspace_root.join(name);
        file_contents.push(std::fs::read_to_string(&path).expect("read webshop model"));
        let content = file_contents.last().expect("content");
        let doc = SysmlDocument::from_memory_path(
            "webshop",
            name,
            content.clone(),
            SysmlDocumentSourceKind::Workspace,
            None,
            None,
        )
        .expect("document uri");
        uris.push(doc.uri.clone());
        documents.push(doc);
    }

    let workspace_sources: Vec<WorkspaceSource<'_>> = WEBSHOP_FILES
        .iter()
        .zip(file_contents.iter())
        .map(|(name, content)| WorkspaceSource {
            path: name,
            content: content.as_str(),
        })
        .collect();

    if include_domain_libraries {
        let library_root = Path::new(r"C:\Git\sysml-domain-libraries");
        if !library_root.is_dir() {
            eprintln!("skipping domain-library webshop test: sysml-domain-libraries not found");
            return None;
        }
        let roots = vec![library_root.to_string_lossy().replace('\\', "/")];
        let loaded = resolve_library_closure(
            &workspace_sources,
            &roots,
            &LibraryClosureOptions::default(),
        )
        .expect("library closure");
        let loaded_paths: Vec<_> = loaded.iter().map(|f| f.path.as_str()).collect();
        assert!(
            !loaded_paths
                .iter()
                .any(|path| path.contains("examples/webshop") || path.contains("examples\\webshop")),
            "domain-library webshop example must not load when workspace declares packages, got {loaded_paths:?}"
        );
        for file in loaded {
            let path = Path::new(&file.root).join(&file.path);
            let doc = SysmlDocument::from_memory_path(
                "webshop-lib",
                path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("library.sysml"),
                file.content,
                SysmlDocumentSourceKind::Library,
                None,
                None,
            )
            .expect("library document uri");
            documents.push(doc);
        }
    }

    let (graph, parsed) =
        build_semantic_graph_from_documents(&documents).expect("semantic graph should build");
    Some((documents, uris, graph, parsed))
}

#[test]
fn webshop_view_expose_targets_resolve_unambiguously() {
    let Some((documents, _, graph, _)) = load_webshop_workspace() else {
        eprintln!("skipping: C:\\Git\\sysml-examples\\webshop not found");
        return;
    };

    let views_uri = documents
        .iter()
        .find(|doc| doc.path_hint.as_deref() == Some("Views.sysml"))
        .map(|doc| doc.uri.clone())
        .expect("Views.sysml");

    let expose_targets = [
        "WebShopExample::webshopSystem",
        "WebShopArchitecture::CheckoutFlow",
        "WebShopArchitecture::OrderEventFanout",
        "WebShopBehavior::OrderLifecycleStateMachine",
        "WebShopBehavior::CheckoutPipeline",
        "WebShopRequirements::CheckoutLatencyReq",
        "WebShopRequirements::CheckoutAvailabilityReq",
        "WebShopRequirements::PaymentReliabilityReq",
        "WebShopRequirements::EventDurabilityReq",
        "WebShopRequirements::SecurityBoundaryReq",
    ];

    for target in expose_targets {
        let expose = resolve_expose_target(&graph, Some(&views_uri), Some("Views"), target);
        match &expose {
            ExposeTargetResolution::Resolved(names) => {
                assert!(
                    !names.is_empty(),
                    "expose target {target} resolved to an empty member set"
                );
            }
            other => panic!("target {target} should resolve, got {other:?}"),
        }
    }
}

#[test]
fn webshop_view_expose_targets_resolve_with_domain_libraries_present() {
    let Some((documents, _, graph, _)) = build_webshop_documents(true) else {
        return;
    };

    let views_uri = documents
        .iter()
        .find(|doc| doc.path_hint.as_deref() == Some("Views.sysml"))
        .map(|doc| doc.uri.clone())
        .expect("Views.sysml");

    let duplicate_count = |target: &str| {
        graph
            .node_ids_for_qualified_name(target)
            .map(|ids| ids.len())
            .unwrap_or(0)
    };
    assert_eq!(
        duplicate_count("WebShopExample::webshopSystem"),
        1,
        "workspace package should shadow the domain-library duplicate"
    );

    let expose = resolve_expose_target(
        &graph,
        Some(&views_uri),
        Some("Views"),
        "WebShopExample::webshopSystem",
    );
    assert!(
        matches!(expose, ExposeTargetResolution::Resolved(_)),
        "expected webshopSystem expose to resolve with domain libraries present, got {expose:?}"
    );
}

#[test]
fn webshop_views_expose_non_empty_elements() {
    let Some((documents, uris, graph, parsed)) = load_webshop_workspace() else {
        eprintln!("skipping: C:\\Git\\sysml-examples\\webshop not found");
        return;
    };

    let views_uri = documents
        .iter()
        .find(|doc| doc.path_hint.as_deref() == Some("Views.sysml"))
        .map(|doc| doc.uri.clone())
        .expect("Views.sysml");

    let diagnostics = collect_diagnostics_from_graph(
        &graph,
        &views_uri,
        DiagnosticsOptions {
            include_hints: true,
        },
    );
    let ambiguous: Vec<_> = diagnostics
        .iter()
        .filter(|d| d.code == "view_expose_unresolved" && d.message.contains("ambiguous"))
        .map(|d| d.message.clone())
        .collect();
    assert!(
        ambiguous.is_empty(),
        "unexpected ambiguous expose diagnostics: {ambiguous:?}"
    );

    let catalog = build_view_catalog(&uris, &parsed);
    let graph_dto = build_workspace_graph_dto_for_uris(&graph, &uris);
    let evaluated = evaluate_views(&catalog, &graph, &graph_dto);
    let empty: Vec<_> = evaluated
        .iter()
        .filter(|view| view.exposed_ids.is_empty())
        .map(|view| view.name.clone())
        .collect();
    assert!(
        empty.is_empty(),
        "views with no exposed elements: {empty:?}; all views: {:?}",
        evaluated
            .iter()
            .map(|v| (v.name.clone(), v.exposed_ids.len()))
            .collect::<Vec<_>>()
    );
}
