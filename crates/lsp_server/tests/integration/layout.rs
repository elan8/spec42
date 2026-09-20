use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::harness::TestSession;

/// `spec42/layout` is stateless (issue #119): no document needs to be opened and no workspace
/// publication is involved, since layout is a pure function of the client-supplied graph.
fn layout_params(graph: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "modelDigest": "blake3:test-digest",
        "viewHandle": "general/root",
        "presentationRevision": 7,
        "graph": graph,
    })
}

fn small_graph() -> serde_json::Value {
    serde_json::json!({
        "id": "root",
        "layoutOptions": { "elk.algorithm": "layered" },
        "children": [
            { "id": "a", "width": 10.0, "height": 10.0 },
            { "id": "b", "width": 10.0, "height": 10.0 }
        ],
        "edges": [{ "id": "e", "sources": ["a"], "targets": ["b"] }]
    })
}

#[test]
fn spec42_layout_lays_out_a_graph_natively_and_echoes_identity() {
    let mut session = TestSession::new();
    session.initialize_default("layout-test");

    let response = session.request("spec42/layout", layout_params(small_graph()));
    assert!(response.get("error").is_none(), "LSP response: {response}");
    let result = &response["result"];
    assert_eq!(
        result["modelDigest"], "blake3:test-digest",
        "LSP response: {response}"
    );
    assert_eq!(
        result["viewHandle"], "general/root",
        "LSP response: {response}"
    );
    assert_eq!(
        result["presentationRevision"], 7,
        "LSP response: {response}"
    );
    assert_eq!(result["engine"], "native", "LSP response: {response}");
    assert!(
        result["layout"]["children"][0]["x"].is_number(),
        "LSP response: {response}"
    );
    assert!(
        result["layout"]["edges"][0]["sections"][0]["startPoint"]["x"].is_number(),
        "LSP response: {response}"
    );
}

#[test]
fn spec42_layout_declines_when_legacy_engine_is_requested() {
    // `new_with_env` takes `&Path` values because every other caller happens to set path-valued
    // vars; `Command::env` treats the value as an opaque OS string regardless, so a non-path
    // value works the same way.
    let mut session =
        TestSession::new_with_env(&[("SPEC42_LAYOUT_ENGINE", std::path::Path::new("legacy"))]);
    session.initialize_default("layout-legacy-test");

    let response = session.request("spec42/layout", layout_params(small_graph()));
    let error = response.get("error").unwrap_or_else(|| {
        panic!("SPEC42_LAYOUT_ENGINE=legacy must decline the request: {response}")
    });
    let message = error["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("legacy"),
        "decline message should explain why: {response}"
    );
}

#[test]
fn spec42_layout_surfaces_typed_contract_errors() {
    let mut session = TestSession::new();
    session.initialize_default("layout-contract-test");

    let nested_edges = serde_json::json!({
        "id": "root",
        "children": [{ "id": "container", "edges": [{ "id": "nested" }] }]
    });
    let response = session.request("spec42/layout", layout_params(nested_edges));
    let error = response
        .get("error")
        .unwrap_or_else(|| panic!("a graph with nested edges must fail: {response}"));
    let message = error["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("nested"),
        "error should name the diagram_layout contract violation: {response}"
    );
}

/// Issue #119's acceptance criterion: p95 <= 100ms on a representative packaged-extension model,
/// or reopen the WASM option. This measures the `spec42/layout` LSP round trip over the same
/// stdio transport the packaged extension actually uses (`TestSession` spawns the real `spec42`
/// binary as a subprocess) -- it does not include the further extension-host/webview hops
/// (`postMessage` + `vscode-languageclient`'s own marshalling), so it is a lower bound on the
/// full user-perceived latency, not a substitute for measuring the complete path from a real
/// VS Code session. The fixture is `timer_interconnection.json`, the largest checked-in ELK
/// graph the diagram-renderer builders emit (18KB) -- this test necessarily runs the debug
/// `spec42` binary the integration harness builds, so its absolute numbers are not
/// release-profile numbers; the budget is generous enough to still be a meaningful regression
/// guard either way.
#[test]
fn spec42_layout_p95_latency_is_within_the_interactive_budget() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../vscode/diagram-renderer/test-fixtures/elk-parity/corpus/timer_interconnection.json",
    );
    let graph: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(&fixture)
            .unwrap_or_else(|error| panic!("read {}: {error}", fixture.display())),
    )
    .unwrap_or_else(|error| panic!("parse {}: {error}", fixture.display()));

    let mut session = TestSession::new();
    session.initialize_default("layout-latency-test");

    const SAMPLES: usize = 20;
    const BUDGET: Duration = Duration::from_millis(100);
    let mut durations = Vec::with_capacity(SAMPLES);
    for _ in 0..SAMPLES {
        let started = Instant::now();
        let response = session.request("spec42/layout", layout_params(graph.clone()));
        durations.push(started.elapsed());
        assert!(response.get("error").is_none(), "LSP response: {response}");
    }
    durations.sort_unstable();
    let p95 = durations[(SAMPLES * 95)
        .div_ceil(100)
        .saturating_sub(1)
        .min(SAMPLES - 1)];
    assert!(
        p95 <= BUDGET,
        "p95 layout latency {p95:?} exceeds the {BUDGET:?} interactive budget over {SAMPLES} \
         samples on {}; durations: {durations:?}",
        fixture.display(),
    );
}
