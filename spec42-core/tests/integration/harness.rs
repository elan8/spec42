//! Shared LSP integration test harness: spawn server, send/read JSON-RPC messages.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};

pub static NEXT_ID: AtomicI64 = AtomicI64::new(1);

pub const INTEGRATION_LAUNCH_MODE: &str = "spec42-core-test-binary";

pub fn server_binary_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_spec42_core_lsp_test"))
}

pub fn spawn_server() -> Child {
    let server_path = server_binary_path();
    eprintln!("spec42 integration harness launch_mode={INTEGRATION_LAUNCH_MODE}");
    Command::new(&server_path)
        // Keep debug diagnostics enabled during integration tests.
        .env("SPEC42_ELK_DEBUG", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap_or_else(|err| panic!("spawn server binary {}: {err}", server_path.display()))
}

#[test]
fn harness_launch_mode_uses_direct_binary() {
    assert_eq!(INTEGRATION_LAUNCH_MODE, "spec42-core-test-binary");
}

/// LSP message framing: "Content-Length: N\r\n\r\n" + body (UTF-8).
pub fn send_message(stdin: &mut std::process::ChildStdin, body: &str) {
    let bytes = body.as_bytes();
    let header = format!("Content-Length: {}\r\n\r\n", bytes.len());
    stdin.write_all(header.as_bytes()).expect("write header");
    stdin.write_all(bytes).expect("write body");
    stdin.flush().expect("flush");
}

pub fn read_message(stdout: &mut std::process::ChildStdout) -> Option<String> {
    let mut header = Vec::new();
    let mut buf = [0u8; 1];
    let mut content_length: Option<usize> = None;
    loop {
        if stdout.read(&mut buf).ok()? == 0 {
            return None;
        }
        header.push(buf[0]);
        if header.ends_with(b"\r\n\r\n") {
            let s = String::from_utf8_lossy(&header);
            for line in s.lines() {
                if line.to_lowercase().starts_with("content-length:") {
                    let num = line
                        .split(':')
                        .nth(1)
                        .and_then(|s| s.trim().parse::<usize>().ok())?;
                    content_length = Some(num);
                    break;
                }
            }
            break;
        }
        if header.len() > 1024 {
            return None;
        }
    }
    let len = content_length?;
    let mut body = vec![0u8; len];
    stdout.read_exact(&mut body).ok()?;
    String::from_utf8(body).ok()
}

/// Read messages until we get a JSON-RPC response with the given id (request response).
pub fn read_response(stdout: &mut std::process::ChildStdout, expect_id: i64) -> Option<String> {
    loop {
        let msg = read_message(stdout)?;
        let json: serde_json::Value = serde_json::from_str(&msg).ok()?;
        if json.get("id").and_then(|v| v.as_i64()) == Some(expect_id) {
            return Some(msg);
        }
    }
}

pub fn next_id() -> i64 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

/// Synchronization barrier for tests that use raw stdin/stdout helpers.
///
/// Sends a cheap request and waits for the response so prior notifications
/// (such as didOpen/didChange) are processed before assertions.
pub fn lsp_barrier(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut std::process::ChildStdout,
) {
    let id = next_id();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": "workspace/symbol",
        "params": { "query": "" }
    });
    send_message(stdin, &req.to_string());
    let _ = read_response(stdout, id).expect("workspace barrier response");
}

pub struct TestSession {
    child: Child,
    stdin: std::process::ChildStdin,
    stdout: std::process::ChildStdout,
}

impl TestSession {
    pub fn new() -> Self {
        let mut child = spawn_server();
        let stdin = child.stdin.take().expect("stdin");
        let stdout = child.stdout.take().expect("stdout");
        Self {
            child,
            stdin,
            stdout,
        }
    }

    pub fn initialize_default(&mut self, client_name: &str) {
        let init_id = next_id();
        let init_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": {
                "processId": null,
                "rootUri": null,
                "capabilities": {},
                "clientInfo": { "name": client_name, "version": "0.1.0" }
            }
        });
        send_message(&mut self.stdin, &init_req.to_string());
        let _ = read_response(&mut self.stdout, init_id).expect("initialize response");
        send_message(
            &mut self.stdin,
            &serde_json::json!({
                "jsonrpc":"2.0",
                "method":"initialized",
                "params":{}
            })
            .to_string(),
        );
    }

    pub fn did_open(&mut self, uri: &str, text: &str, version: i32) {
        send_message(
            &mut self.stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": { "uri": uri, "languageId": "sysml", "version": version, "text": text }
                }
            })
            .to_string(),
        );
    }

    #[allow(dead_code)]
    pub fn did_change_full(&mut self, uri: &str, text: &str, version: i32) {
        send_message(
            &mut self.stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didChange",
                "params": {
                    "textDocument": { "uri": uri, "version": version },
                    "contentChanges": [{ "text": text }]
                }
            })
            .to_string(),
        );
    }

    pub fn request(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        let id = next_id();
        let req = serde_json::json!({
            "jsonrpc":"2.0",
            "id": id,
            "method": method,
            "params": params
        });
        send_message(&mut self.stdin, &req.to_string());
        let raw = read_response(&mut self.stdout, id).expect("request response");
        serde_json::from_str(&raw).expect("json response")
    }

    /// Synchronization barrier for integration tests.
    ///
    /// Sends a cheap request and waits for its response so prior notifications
    /// (e.g. didOpen/didChange) are processed in-order before assertions.
    pub fn barrier(&mut self) {
        let _ = self.request("workspace/symbol", serde_json::json!({ "query": "" }));
    }
}

impl Drop for TestSession {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}
