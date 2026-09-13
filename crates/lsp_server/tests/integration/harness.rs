//! Shared LSP integration test harness: spawn server, send/read JSON-RPC messages.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};

pub static NEXT_ID: AtomicI64 = AtomicI64::new(1);

pub const INTEGRATION_LAUNCH_MODE: &str = "spec42-core-test-binary";

pub fn server_binary_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_spec42_core_lsp_test"))
}

/// Owns a server process until it has been terminated and reaped, including during unwinding.
pub struct TestServer(Child);

impl std::ops::Deref for TestServer {
    type Target = Child;

    fn deref(&self) -> &Child {
        &self.0
    }
}

impl std::ops::DerefMut for TestServer {
    fn deref_mut(&mut self) -> &mut Child {
        &mut self.0
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

pub fn spawn_server() -> TestServer {
    spawn_server_with_env(&[])
}

pub fn spawn_server_with_env(env: &[(&str, &std::path::Path)]) -> TestServer {
    let server_path = server_binary_path();
    eprintln!("spec42 integration harness launch_mode={INTEGRATION_LAUNCH_MODE}");
    let mut command = Command::new(&server_path);
    for (name, value) in env {
        command.env(name, value);
        if *name == "SPEC42_LSP_TEST_STDLIB" {
            command.env("SPEC42_LIBRARY_FULL_SCAN", "1");
        }
    }
    TestServer(
        command
            // Keep debug diagnostics enabled during integration tests.
            .env("SPEC42_ELK_DEBUG", "1")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .unwrap_or_else(|err| panic!("spawn server binary {}: {err}", server_path.display())),
    )
}

#[test]
fn harness_launch_mode_uses_direct_binary() {
    assert_eq!(INTEGRATION_LAUNCH_MODE, "spec42-core-test-binary");
}

#[test]
fn harness_closes_server_stdout_on_drop_and_panic() {
    for panic_on_exit in [false, true] {
        let (sender, receiver) = std::sync::mpsc::channel();
        let result = std::panic::catch_unwind(|| {
            let mut server = spawn_server();
            let mut stdout = server.stdout.take().expect("stdout");
            std::thread::spawn(move || {
                let mut byte = [0];
                let _ = sender.send(stdout.read(&mut byte));
            });
            if panic_on_exit {
                panic!("exercise server cleanup while unwinding");
            }
        });
        assert_eq!(result.is_err(), panic_on_exit);
        assert_eq!(
            receiver
                .recv_timeout(std::time::Duration::from_secs(10))
                .expect("server must close stdout on exit")
                .expect("read server stdout"),
            0
        );
    }
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
pub fn lsp_barrier(stdin: &mut std::process::ChildStdin, stdout: &mut std::process::ChildStdout) {
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

/// Deterministic publication barrier: block until the server publishes diagnostics for `uri`.
///
/// Every publisher of `textDocument/publishDiagnostics` diagnoses a document from a captured
/// session publication and publishes only while that publication is still the live one — whether
/// it is the relink task after a `didOpen`/`didChange` on a ready session, or the startup scan's
/// sweep when the `didOpen` landed while the session was still `Indexing` and no relink token was
/// available. Either way the notification for a URI means: the publication that currently answers
/// requests has this document's admitted revision in it. Blocking on it therefore observes the
/// publication barrier itself, instead of guessing at wall-clock indexing latency with a
/// sleep/retry loop.
///
/// That equivalence is exactly what `rebuild_publication` guarantees by preparing its inputs and
/// taking its build token in one actor turn (see `session/handle.rs`); before that, a document
/// could be in the index — and so be diagnosed and published for — while a superseding build
/// prepared from a staler index kept it out of the publication, and requests answered empty.
///
/// Call this before any request whose `read_response` would otherwise discard the notification.
pub fn wait_for_publication(stdout: &mut std::process::ChildStdout, uri: &str) {
    wait_for_publications(stdout, &[uri]);
}

/// [`wait_for_publication`] for several documents whose publications may arrive in any order.
pub fn wait_for_publications(stdout: &mut std::process::ChildStdout, uris: &[&str]) {
    let mut pending: Vec<String> = uris.iter().map(|uri| normalized_uri(uri)).collect();
    while !pending.is_empty() {
        let msg = read_message(stdout).unwrap_or_else(|| {
            panic!("server closed before publishing diagnostics for {pending:?}")
        });
        let json: serde_json::Value = match serde_json::from_str(&msg) {
            Ok(json) => json,
            Err(_) => continue,
        };
        if json["method"].as_str() != Some("textDocument/publishDiagnostics") {
            continue;
        }
        if let Some(published) = json["params"]["uri"].as_str().map(normalized_uri) {
            pending.retain(|uri| *uri != published);
        }
    }
}

/// Compare URIs the way the server may re-serialize them when publishing.
fn normalized_uri(uri: &str) -> String {
    uri.trim_end_matches('/').to_ascii_lowercase()
}

pub struct TestSession {
    _child: TestServer,
    stdin: std::process::ChildStdin,
    stdout: std::process::ChildStdout,
}

impl TestSession {
    pub fn new() -> Self {
        Self::new_with_env(&[])
    }

    pub fn new_with_env(env: &[(&str, &std::path::Path)]) -> Self {
        let mut child = spawn_server_with_env(env);
        let stdin = child.stdin.take().expect("stdin");
        let stdout = child.stdout.take().expect("stdout");
        Self {
            _child: child,
            stdin,
            stdout,
        }
    }

    pub fn initialize_default(&mut self, client_name: &str) {
        self.initialize_with_root_and_options(client_name, None, None);
    }

    pub fn initialize_with_root(&mut self, client_name: &str, root_uri: &url::Url) {
        self.initialize_with_root_and_options(client_name, Some(root_uri), None);
    }

    pub fn initialize_with_options(
        &mut self,
        client_name: &str,
        initialization_options: Option<serde_json::Value>,
    ) {
        self.initialize_with_root_and_options(client_name, None, initialization_options);
    }

    fn initialize_with_root_and_options(
        &mut self,
        client_name: &str,
        root_uri: Option<&url::Url>,
        initialization_options: Option<serde_json::Value>,
    ) {
        let init_id = next_id();
        let mut params = serde_json::json!({
            "processId": null,
            "rootUri": root_uri,
            "capabilities": {},
            "clientInfo": { "name": client_name, "version": "0.1.0" }
        });
        if let Some(options) = initialization_options {
            params["initializationOptions"] = options;
        }
        let init_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": init_id,
            "method": "initialize",
            "params": params
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

    pub fn wait_for_publications(&mut self, uris: &[&str]) {
        wait_for_publications(&mut self.stdout, uris);
    }
}
