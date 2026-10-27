//! Opt-in performance observations are versioned protocol records, not semantic projections.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};
use serde_json::{json, Value};

#[test]
fn edit_performance_reports_correlated_owner_phase_measurements() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path().canonicalize().unwrap();
    let path = root.join("model.sysml");
    let source = "package P { part def A; }";
    std::fs::write(&path, source).unwrap();
    let uri = url::Url::from_file_path(&path).unwrap().to_string();
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let id = next_id();
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "id":id, "method":"initialize", "params":{
            "processId":null, "rootUri":url::Url::from_directory_path(&root).unwrap(),
            "capabilities":{}, "initializationOptions":{"performanceLogging":{"enabled":true}}
        }})
        .to_string(),
    );
    read_response(&mut stdout, id).unwrap();
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "method":"initialized", "params":{}}).to_string(),
    );
    send_message(
        &mut stdin,
        &json!({"jsonrpc":"2.0", "method":"textDocument/didOpen", "params":{
            "textDocument":{"uri":uri,"languageId":"sysml","version":1,"text":source}
        }})
        .to_string(),
    );
    send_message(&mut stdin, &json!({"jsonrpc":"2.0", "method":"textDocument/didChange", "params":{
        "textDocument":{"uri":uri,"version":2}, "contentChanges":[{"text":"package P { part def B; }"}]
    }}).to_string());
    let mut publication = None;
    loop {
        let message: Value =
            serde_json::from_str(&read_message(&mut stdout).expect("performance observation"))
                .unwrap();
        let Some(raw) = message["params"]["message"]
            .as_str()
            .and_then(|raw| raw.strip_prefix("[SysML][perf] "))
        else {
            continue;
        };
        let record: Value = serde_json::from_str(raw).expect("structured performance record");
        if record["version"] != 2 {
            continue;
        }
        if record["event"] == "backend:publication" {
            publication = Some(record);
        } else if record["event"] == "backend:didChange" {
            break;
        }
    }
    let publication =
        publication.expect("phase record must precede completion for the same version");
    assert!(publication["uri"]
        .as_str()
        .unwrap()
        .eq_ignore_ascii_case(&uri));
    assert_eq!(publication["outcome"], "Published");
    for field in [
        "rebuildMs",
        "preparationMs",
        "preparationQueueMs",
        "constructionWallMs",
        "admissionWallMs",
        "buildParseMs",
        "loweringMs",
        "resolveMs",
        "evaluationMs",
        "indexingMs",
        "checksMs",
        "diagnosticFanoutMs",
    ] {
        assert!(
            publication[field]
                .as_f64()
                .is_some_and(|value| value.is_finite() && value >= 0.0),
            "invalid timing {field}: {publication}"
        );
    }
    for field in ["documentsLowered", "documentsReused", "sourcesParsed"] {
        assert!(
            publication[field].as_u64().is_some(),
            "invalid counted fact {field}: {publication}"
        );
    }
    let subphases: f64 = ["resolveMs", "evaluationMs", "indexingMs", "checksMs"]
        .iter()
        .map(|field| publication[field].as_f64().unwrap())
        .sum();
    assert!(publication["resolutionTotalMs"].as_f64().unwrap() >= subphases);
}
