//! Semantic tokens integration tests.

use std::path::PathBuf;

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

fn decode_semantic_tokens(data: &[serde_json::Value]) -> Vec<(u32, u32, u32, u32)> {
    let mut line: u32 = 0;
    let mut start_char: u32 = 0;
    let mut tokens: Vec<(u32, u32, u32, u32)> = Vec::new();
    let raw: Vec<u32> = data
        .iter()
        .filter_map(|v| v.as_u64().map(|u| u as u32))
        .collect();
    let mut i = 0;
    while i + 5 <= raw.len() {
        line += raw[i];
        start_char = if raw[i] == 0 {
            start_char + raw[i + 1]
        } else {
            raw[i + 1]
        };
        let length = raw[i + 2];
        let token_type = raw[i + 3];
        tokens.push((line, start_char, length, token_type));
        i += 5;
    }
    tokens
}

/// Integration test for semantic tokens: verifies that port definition syntax
/// (in/out, parameter names, types like Real/String) is tokenized correctly.
/// Helps diagnose syntax highlighting issues (e.g. "position" incorrectly as keyword).
#[test]
fn lsp_semantic_tokens_port_definitions() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///semantic_tokens_test.sysml";
    let content = r#"port def GimbalCommandPort {
    in panAngle : Real;
    in tiltAngle : Real;
}
port def PowerPort {
    out voltage : Real;
    out current : Real;
}
port def SensorDataPort {
    out position : String;
    out velocity : String;
}"#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "semantic_tokens_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let sem_id = next_id();
    let sem_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": sem_id,
        "method": "textDocument/semanticTokens/full",
        "params": {
            "textDocument": { "uri": uri }
        }
    });
    send_message(&mut stdin, &sem_req.to_string());
    let sem_resp = read_response(&mut stdout, sem_id).expect("semanticTokens/full response");
    let sem_json: serde_json::Value =
        serde_json::from_str(&sem_resp).expect("parse semanticTokens response");
    assert_eq!(sem_json["id"], sem_id);

    let data = sem_json["result"]["data"]
        .as_array()
        .expect("semanticTokens result should have data array");
    // Decode delta encoding: [deltaLine, deltaStartChar, length, tokenType, tokenModifiers] per token
    let tokens = decode_semantic_tokens(data);

    let lines: Vec<&str> = content.lines().collect();
    let token_text = |(ln, start, len, _ty): &(u32, u32, u32, u32)| -> String {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        let s = *start as usize;
        let e = (*start + *len) as usize;
        line_str.chars().take(e).skip(s).collect()
    };

    // Legend: 0=KEYWORD, 5=VARIABLE, 6=TYPE, 10=PROPERTY
    let in_out_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(token_text(t).as_str(), "in" | "out"))
        .collect();
    assert!(
        !in_out_tokens.is_empty(),
        "should tokenize in/out; tokens: {:?}",
        tokens
            .iter()
            .map(|t| (token_text(t), t.3))
            .collect::<Vec<_>>()
    );
    for t in &in_out_tokens {
        assert_eq!(
            t.3,
            0,
            "in/out should be KEYWORD (0), got {} for {:?}",
            t.3,
            token_text(t)
        );
    }

    for ident in ["position", "panAngle", "current", "velocity"] {
        let ident_tokens: Vec<_> = tokens.iter().filter(|t| token_text(t) == ident).collect();
        assert!(!ident_tokens.is_empty(), "should tokenize '{}'", ident);
        for t in &ident_tokens {
            assert_ne!(
                t.3, 0,
                "{} must NOT be KEYWORD (valid identifier); got type {}",
                ident, t.3
            );
        }
    }

    let real_string_tokens: Vec<_> = tokens
        .iter()
        .filter(|t| matches!(token_text(t).as_str(), "Real" | "String"))
        .collect();
    assert!(
        !real_string_tokens.is_empty(),
        "should tokenize Real/String types"
    );
    for t in &real_string_tokens {
        assert_eq!(
            t.3,
            6,
            "Real/String should be TYPE (6), got {} for {:?}",
            t.3,
            token_text(t)
        );
    }

    let _ = child.kill();
}

const SEMANTIC_TYPE_NAMES: &[&str] = &[
    "KEYWORD",
    "STRING",
    "NUMBER",
    "COMMENT",
    "OPERATOR",
    "VARIABLE",
    "TYPE",
    "NAMESPACE",
    "CLASS",
    "INTERFACE",
    "PROPERTY",
    "FUNCTION",
];

/// Integration test that dumps full semantic token output for investigation.
/// Writes target/semantic_tokens_investigation.txt with every token (line:col text -> type).
/// Run: cargo test -p spec42 --test lsp_integration lsp_semantic_tokens_investigation
#[test]
fn lsp_semantic_tokens_update_after_edit() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///semantic_tokens_edit.sysml";
    let original = "package P {\n  part def Engine;\n}\n";
    let edited = "package P {\n  part def EngineCore;\n}\n";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "semantic_tokens_edit_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": original }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let sem_before_id = next_id();
    let sem_before_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": sem_before_id,
        "method": "textDocument/semanticTokens/full",
        "params": { "textDocument": { "uri": uri } }
    });
    send_message(&mut stdin, &sem_before_req.to_string());
    let sem_before_resp = read_response(&mut stdout, sem_before_id)
        .expect("semanticTokens/full response before edit");
    let sem_before_json: serde_json::Value =
        serde_json::from_str(&sem_before_resp).expect("parse semanticTokens response before edit");
    let before_data = sem_before_json["result"]["data"]
        .as_array()
        .expect("semanticTokens result should have data array before edit");

    let did_change = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didChange",
        "params": {
            "textDocument": { "uri": uri, "version": 2 },
            "contentChanges": [
                { "text": edited }
            ]
        }
    });
    send_message(&mut stdin, &did_change.to_string());
    std::thread::sleep(std::time::Duration::from_millis(80));

    let sem_after_id = next_id();
    let sem_after_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": sem_after_id,
        "method": "textDocument/semanticTokens/full",
        "params": { "textDocument": { "uri": uri } }
    });
    send_message(&mut stdin, &sem_after_req.to_string());
    let sem_after_resp =
        read_response(&mut stdout, sem_after_id).expect("semanticTokens/full response after edit");
    let sem_after_json: serde_json::Value =
        serde_json::from_str(&sem_after_resp).expect("parse semanticTokens response after edit");
    let after_data = sem_after_json["result"]["data"]
        .as_array()
        .expect("semanticTokens result should have data array after edit");

    let before_tokens = decode_semantic_tokens(before_data);
    let after_tokens = decode_semantic_tokens(after_data);
    let before_lines: Vec<&str> = original.lines().collect();
    let after_lines: Vec<&str> = edited.lines().collect();
    let token_text = |lines: &[&str], (ln, start, len, _ty): &(u32, u32, u32, u32)| -> String {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        let s = *start as usize;
        let e = (*start + *len) as usize;
        line_str.chars().take(e).skip(s).collect()
    };

    assert!(
        before_tokens
            .iter()
            .any(|t| token_text(&before_lines, t) == "Engine"),
        "expected semantic token for Engine before edit"
    );
    assert!(
        !before_tokens
            .iter()
            .any(|t| token_text(&before_lines, t) == "EngineCore"),
        "did not expect EngineCore token before edit"
    );
    assert!(
        after_tokens
            .iter()
            .any(|t| token_text(&after_lines, t) == "EngineCore"),
        "expected semantic token for EngineCore after edit"
    );
    assert!(
        !after_tokens
            .iter()
            .any(|t| token_text(&after_lines, t) == "Engine"),
        "did not expect stale Engine token after edit"
    );

    let _ = child.kill();
}

#[test]
fn lsp_semantic_tokens_investigation() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///semantic_tokens_investigation.sysml";
    let content = r#"port def GimbalCommandPort {
    in panAngle : Real;
    in tiltAngle : Real;
    in mode : String;
}
port def PowerPort {
    out voltage : Real;
    out current : Real;
}
port def SensorDataPort {
    out position : String;
    out velocity : String;
    out attitude : String;
    out altitude : Real;
}"#;

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "semantic_tokens_investigation", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": content }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(150));

    let sem_id = next_id();
    let sem_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": sem_id,
        "method": "textDocument/semanticTokens/full",
        "params": {
            "textDocument": { "uri": uri }
        }
    });
    send_message(&mut stdin, &sem_req.to_string());
    let sem_resp = read_response(&mut stdout, sem_id).expect("semanticTokens/full response");
    let sem_json: serde_json::Value =
        serde_json::from_str(&sem_resp).expect("parse semanticTokens response");

    let _ = child.kill();

    let result = &sem_json["result"];
    let data = result["data"]
        .as_array()
        .expect("semanticTokens result should have data array");

    let tokens = decode_semantic_tokens(data);

    let lines: Vec<&str> = content.lines().collect();
    let mut report = String::new();
    report.push_str("Semantic tokens investigation dump\n");
    report.push_str("=================================\n");
    report.push_str("Legend: 0=KEYWORD, 5=VARIABLE, 6=TYPE, 10=PROPERTY\n");
    report.push_str("Expected: in/out=KEYWORD, panAngle/current/velocity/position=!KEYWORD, Real/String=TYPE\n\n");
    report.push_str("Source:\n");
    for (i, l) in lines.iter().enumerate() {
        report.push_str(&format!("  {:2}: {}\n", i, l));
    }
    report.push_str("\nTokens:\n");
    for (ln, start, len, ty) in &tokens {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        let s = *start as usize;
        let e = (*start + *len) as usize;
        let text: String = line_str.chars().take(e).skip(s).collect();
        let ty_name = SEMANTIC_TYPE_NAMES
            .get(*ty as usize)
            .copied()
            .unwrap_or("?");
        report.push_str(&format!(
            "  {}:{} len={:2} type={} {:?}\n",
            ln, start, len, ty_name, text
        ));
    }
    report.push_str("\nProblematic identifiers (should NOT be KEYWORD):\n");
    report.push_str(
        "(If all show 'ok' below, the backend is correct. Highlight issues are client-side.)\n",
    );
    let token_text = |(ln, start, len, _ty): &(u32, u32, u32, u32)| -> String {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        let s = *start as usize;
        let e = (*start + *len) as usize;
        line_str.chars().take(e).skip(s).collect()
    };
    for ident in [
        "panAngle",
        "current",
        "velocity",
        "position",
        "tiltAngle",
        "mode",
        "voltage",
        "attitude",
        "altitude",
    ] {
        let ident_tokens: Vec<_> = tokens.iter().filter(|t| token_text(t) == ident).collect();
        if ident_tokens.is_empty() {
            report.push_str(&format!("  {}: NOT FOUND\n", ident));
        } else {
            for t in &ident_tokens {
                let ty_name = SEMANTIC_TYPE_NAMES
                    .get(t.3 as usize)
                    .copied()
                    .unwrap_or("?");
                let status = if t.3 == 0 {
                    " *** WRONG (KEYWORD) ***"
                } else {
                    " ok"
                };
                report.push_str(&format!(
                    "  {}: {} at {}:{}{}\n",
                    ident, ty_name, t.0, t.1, status
                ));
            }
        }
    }
    report.push_str("\nWhen backend is correct but highlighting still wrong, check:\n");
    report.push_str("  - editor.semanticHighlighting.enabled = true\n");
    report.push_str("  - No other SysML extension providing conflicting highlighting\n");
    report.push_str("  - Reload window after extension/grammar changes\n");

    let target_dir = std::env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .expect("server has parent")
                .join("target")
        });
    let _ = std::fs::create_dir_all(&target_dir);
    let out_path = target_dir.join("semantic_tokens_investigation.txt");
    std::fs::write(&out_path, &report).expect("write investigation dump");
    eprintln!("Investigation dump written to {:?}", out_path);

    assert!(
        sem_json["error"].is_null(),
        "semanticTokens/full should not return error: {:?}",
        sem_json["error"]
    );
    for ident in ["panAngle", "current", "velocity", "position"] {
        let ident_tokens: Vec<_> = tokens.iter().filter(|t| token_text(t) == ident).collect();
        assert!(
            !ident_tokens.is_empty(),
            "should tokenize '{}'; see {:?}",
            ident,
            out_path
        );
        for t in &ident_tokens {
            assert_ne!(
                t.3, 0,
                "{} must NOT be KEYWORD; got KEYWORD. Full dump: {:?}",
                ident, out_path
            );
        }
    }
}
