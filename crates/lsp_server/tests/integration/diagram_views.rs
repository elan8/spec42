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

    for view in views {
        assert_eq!(view["namespace"], "Views", "canonical owner: {view}");
        let range = &view["source"]["range"];
        assert!(
            range["startLine"].is_u64() && range["endLine"].is_u64(),
            "definition range: {view}"
        );
    }

    // A catalog handle yields the natively built diagram product on the same publication.
    let pipeline = views
        .iter()
        .find(|view| view["name"] == "checkoutPipeline")
        .unwrap_or_else(|| panic!("checkoutPipeline view: {response}"));
    let diagram = session.request(
        "spec42/diagram",
        serde_json::json!({
            "modelUri": views_uri,
            "handle": pipeline["handle"],
            "expectedModelDigest": response["result"]["modelDigest"],
        }),
    );
    assert!(diagram.get("error").is_none(), "LSP response: {diagram}");
    assert_eq!(
        diagram["result"]["modelDigest"], response["result"]["modelDigest"],
        "LSP response: {diagram}"
    );
    let product_json = diagram["result"]["productJson"]
        .as_str()
        .unwrap_or_else(|| panic!("product JSON: {diagram}"));
    // Indentation was about 40% of every product the editor received.
    assert!(
        !product_json.contains('\n'),
        "the editor receives the compact product"
    );
    let product: serde_json::Value =
        serde_json::from_str(product_json).expect("the product is JSON");
    assert_eq!(product["schemaVersion"], diagram_product::SCHEMA_VERSION);
    assert_eq!(product["selectedView"]["name"], "checkoutPipeline");
    assert_eq!(product["modelDigest"], response["result"]["modelDigest"]);

    let stale = session.request(
        "spec42/diagram",
        serde_json::json!({
            "modelUri": views_uri,
            "handle": pipeline["handle"],
            "expectedModelDigest": "blake3:stale",
        }),
    );
    assert!(
        stale.get("error").is_some(),
        "a stale selection is refused: {stale}"
    );

    assert_eq!(
        models.len(),
        5,
        "the regression must exercise the whole webshop model"
    );
}

#[test]
fn sequence_products_survive_live_anonymous_nested_and_recovery_edits() {
    use diagram_product::wire::{CompletenessStatus, DiagramProduct, Projection};
    let stdlib = tempfile::tempdir().unwrap();
    standard_library_fixture(stdlib.path());
    let workspace = tempfile::tempdir().unwrap();
    let path = workspace.path().join("Model.sysml");
    let uri = url::Url::from_file_path(&path).unwrap().to_string();
    let mut session = TestSession::new_with_env(&[("SPEC42_LSP_TEST_STDLIB", stdlib.path())]);
    session.initialize_with_root(
        "sequence-live-edits",
        &url::Url::from_directory_path(workspace.path()).unwrap(),
    );
    let fixtures = [
        include_str!("../../../../tests/snapshots/generation/sequence_shared_part_endpoints.md"),
        include_str!("../../../../tests/snapshots/generation/sequence_anonymous_messages.md"),
        include_str!("../../../../tests/snapshots/generation/sequence_omitted_nested.md"),
        include_str!("../../../../tests/snapshots/generation/sequence_recovery.md"),
        include_str!("../../../../tests/snapshots/generation/sequence_recovery_repaired.md"),
    ];
    let mut previous_digest = None;
    for (stage, fixture) in fixtures.iter().enumerate() {
        let fixture = fixture.replace("\r\n", "\n");
        let source = fixture
            .split_once("## model.sysml\n~~~sysml\n")
            .unwrap()
            .1
            .split_once("\n~~~")
            .unwrap()
            .0;
        if stage == 0 {
            session.did_open(&uri, source, 1);
        } else {
            session.did_change_full(&uri, source, stage as i32 + 1);
        }
        session.wait_for_publications(&[&uri]);
        session.barrier();
        let catalog = session.request("spec42/diagramViews", serde_json::json!({"modelUri": uri}));
        let view = catalog["result"]["views"]
            .as_array()
            .unwrap()
            .iter()
            .find(|view| view["name"] == "selected")
            .expect("selected view survives the edit");
        let digest = catalog["result"]["modelDigest"].clone();
        let response = session.request(
            "spec42/diagram",
            serde_json::json!({
                "modelUri": uri, "handle": view["handle"], "expectedModelDigest": digest,
            }),
        );
        assert!(response.get("error").is_none(), "stage {stage}: {response}");
        let product: DiagramProduct =
            serde_json::from_str(response["result"]["productJson"].as_str().unwrap()).unwrap();
        assert_eq!(product.schema_version, diagram_product::SCHEMA_VERSION);
        assert_eq!(product.model_digest, digest.as_str().unwrap());
        let golden: DiagramProduct = serde_json::from_str(
            fixture
                .split_once("## diagram.json\n~~~json\n")
                .unwrap()
                .1
                .split_once("\n~~~")
                .unwrap()
                .0,
        )
        .unwrap();
        assert_eq!(
            matches!(product.completeness.status, CompletenessStatus::Complete),
            matches!(golden.completeness.status, CompletenessStatus::Complete)
        );
        let labels = |projection: &Projection| match projection {
            Projection::Sequence { scene, .. } => scene
                .messages
                .iter()
                .map(|message| message.label.clone())
                .collect::<Vec<_>>(),
            _ => panic!("expected sequence product"),
        };
        assert_eq!(labels(&product.projection), labels(&golden.projection));
        if let Some(previous) = previous_digest {
            assert_ne!(digest, previous, "each edit publishes a new identity");
            let stale = session.request(
                "spec42/diagram",
                serde_json::json!({
                    "modelUri": uri, "handle": view["handle"], "expectedModelDigest": previous,
                }),
            );
            assert!(
                stale.get("error").is_some(),
                "superseded selection: {stale}"
            );
        }
        previous_digest = Some(digest);
    }
}
