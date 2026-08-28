use std::path::{Path, PathBuf};

use super::harness::TestSession;

fn standard_library_fixture(root: &Path) {
    let systems = root.join("SysML_Systems_Library-2.0.0");
    std::fs::create_dir_all(&systems).expect("standard-library root");
    std::fs::write(
        systems.join("StandardViewDefinitions.sysml"),
        concat!(
            "standard library package StandardViewDefinitions { ",
            "view def GeneralView; ",
            "view def InterconnectionView; ",
            "view def SequenceView; ",
            "view def StateTransitionView; ",
            "view def ActionFlowView; ",
            "}\n",
        ),
    )
    .expect("standard view definitions");
}

fn open_workspace_models(session: &mut TestSession, root: &Path) -> Vec<(String, String)> {
    let mut models = std::fs::read_dir(root)
        .expect("webshop fixture directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "sysml")
        })
        .map(|path| {
            let uri = url::Url::from_file_path(&path)
                .expect("model URI")
                .to_string();
            let text = std::fs::read_to_string(&path).expect("model source");
            (uri, text)
        })
        .collect::<Vec<_>>();
    models.sort_by(|left, right| left.0.cmp(&right.0));
    for (uri, text) in &models {
        session.did_open(uri, text, 1);
    }
    models
}

#[test]
fn diagram_views_returns_the_webshop_catalog_over_the_lsp() {
    let stdlib = tempfile::tempdir().expect("standard-library fixture");
    standard_library_fixture(stdlib.path());
    let webshop = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/webshop")
        .canonicalize()
        .expect("webshop fixture root");
    let views_uri = url::Url::from_file_path(webshop.join("Views.sysml"))
        .expect("Views.sysml URI")
        .to_string();

    let mut session = TestSession::new_with_env(&[("SPEC42_LSP_TEST_STDLIB", stdlib.path())]);
    let root_uri = url::Url::from_directory_path(&webshop).expect("workspace root URI");
    session.initialize_with_root("diagram-views-test", &root_uri);
    let models = open_workspace_models(&mut session, &webshop);
    session.wait_for_publications(
        &models
            .iter()
            .map(|(uri, _)| uri.as_str())
            .collect::<Vec<_>>(),
    );

    let definition = session.request(
        "textDocument/definition",
        serde_json::json!({
            "textDocument": { "uri": views_uri },
            "position": { "line": 7, "character": 24 }
        }),
    );
    assert!(
        !definition["result"].is_null(),
        "GeneralView must resolve through the LSP publication: {definition}"
    );

    let response = session.request(
        "spec42/diagramViews",
        serde_json::json!({ "modelUri": views_uri }),
    );
    assert!(response.get("error").is_none(), "LSP response: {response}");
    assert_eq!(
        response["result"]["semanticStatus"]["available"], true,
        "LSP response: {response}"
    );
    let views = response["result"]["views"]
        .as_array()
        .unwrap_or_else(|| panic!("diagram views array: {response}"));
    assert_eq!(views.len(), 7, "LSP response: {response}");

    let mut names = views
        .iter()
        .filter_map(|view| view["name"].as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(
        names,
        [
            "checkoutFlow",
            "checkoutPipeline",
            "connections",
            "orderEventFanout",
            "orderLifecycle",
            "requirements",
            "structure",
        ]
    );

    assert_eq!(
        models.len(),
        5,
        "the regression must exercise the whole webshop model"
    );
}
