//! Completion integration tests.

use super::harness::TestSession;

#[test]
fn lsp_completion() {
    let mut session = TestSession::new();

    let uri = "file:///test2.sysml";
    let content = "package P { part def X; }";

    session.initialize_default("test");
    session.did_open(uri, content, 1);
    session.barrier();
    let compl_json = session.request(
        "textDocument/completion",
        serde_json::json!({
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 2 }
        }),
    );
    let items = compl_json["result"]
        .as_array()
        .or_else(|| compl_json["result"]["items"].as_array());
    assert!(
        items.is_some(),
        "completion should return array: {}",
        compl_json
    );
    let items = items.unwrap();
    assert!(
        !items.is_empty(),
        "completion should have at least one item"
    );
    let labels: Vec<String> = items
        .iter()
        .filter_map(|i| i["label"].as_str().map(String::from))
        .collect();
    assert!(
        labels.iter().any(|l| l == "part" || l == "package"),
        "completion should include keywords: {:?}",
        labels
    );

}
