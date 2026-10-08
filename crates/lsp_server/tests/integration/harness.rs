//! Shared LSP integration test harness: spawn server, send/read JSON-RPC messages.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::mpsc;
use std::time::Duration;

pub static NEXT_ID: AtomicI64 = AtomicI64::new(1);

pub const INTEGRATION_LAUNCH_MODE: &str = "spec42-core-test-binary";

pub fn server_binary_path() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_BIN_EXE_spec42_core_lsp_test"))
}

/// How long one spawned server may live before the watchdog ends it.
///
/// A healthy test finishes in seconds. A server that outlives this budget is one a test is
/// blocked on: every harness read is a blocking read of the server's stdout, so a notification
/// or response that never comes would otherwise hang the test run forever (issue #248).
const SERVER_BUDGET: Duration = Duration::from_secs(300);

fn server_budget() -> Duration {
    std::env::var("SPEC42_LSP_TEST_SERVER_BUDGET_SECS")
        .ok()
        .and_then(|secs| secs.parse().ok())
        .map_or(SERVER_BUDGET, Duration::from_secs)
}

/// Owns a server process until it has been terminated and reaped, including during unwinding.
///
/// A watchdog thread ends a server that outlives [`SERVER_BUDGET`]. That closes its stdout, so
/// the blocked harness read returns and the test fails at the wait it was stuck in, with that
/// wait's own message (the pending URIs, the awaited response). The watchdog first reports what
/// it can see of the stalled server: its threads' states and the tail of its stderr.
pub struct TestServer {
    child: Child,
    stderr_path: std::path::PathBuf,
    // Dropped with the server: disconnecting the channel is what stops the watchdog.
    _finished: mpsc::Sender<()>,
}

impl std::ops::Deref for TestServer {
    type Target = Child;

    fn deref(&self) -> &Child {
        &self.child
    }
}

impl std::ops::DerefMut for TestServer {
    fn deref_mut(&mut self) -> &mut Child {
        &mut self.child
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = std::fs::remove_file(&self.stderr_path);
    }
}

fn watch_server(pid: u32, stderr_path: std::path::PathBuf, budget: Duration) -> mpsc::Sender<()> {
    let (finished, watchdog) = mpsc::channel::<()>();
    std::thread::spawn(move || {
        // The sender is dropped with the `TestServer`, which ends the wait with `Disconnected`.
        if watchdog.recv_timeout(budget) != Err(mpsc::RecvTimeoutError::Timeout) {
            return;
        }
        eprintln!(
            "spec42 integration harness: server {pid} is still running after {}s; a test is \
             blocked on it. Ending it so the test fails where it is waiting.",
            budget.as_secs()
        );
        eprintln!("{}", stalled_server_report(pid, &stderr_path));
        end_process(pid);
    });
    finished
}

/// What is observable of a stalled server from outside: its threads' scheduler states (Linux)
/// and the end of its stderr. An idle server and a deadlocked one both show every thread asleep
/// (`S`), so the states only rule out a busy loop; which wait the test was in tells them apart.
fn stalled_server_report(pid: u32, stderr_path: &std::path::Path) -> String {
    let mut report = String::new();
    if let Ok(tasks) = std::fs::read_dir(format!("/proc/{pid}/task")) {
        let mut threads = tasks
            .filter_map(Result::ok)
            .filter_map(|task| {
                let stat = std::fs::read_to_string(task.path().join("stat")).ok()?;
                // `pid (comm) state ...`; the command may itself contain spaces or parentheses.
                let (head, tail) = stat.rsplit_once(") ")?;
                let name = head.split_once('(')?.1.to_owned();
                Some((name, tail.chars().next()?))
            })
            .collect::<Vec<_>>();
        threads.sort();
        let running = threads.iter().filter(|(_, state)| *state == 'R').count();
        report.push_str(&format!(
            "server threads: {} total, {running} runnable\n",
            threads.len()
        ));
        // One line per thread name and state, with how many threads share it.
        for group in threads.chunk_by(|left, right| left == right) {
            let (name, state) = &group[0];
            report.push_str(&format!("  {state} {name} x{}\n", group.len()));
        }
    }
    let stderr = std::fs::read_to_string(stderr_path).unwrap_or_default();
    let tail = stderr.lines().rev().take(40).collect::<Vec<_>>();
    report.push_str(&format!("server stderr (last {} lines):\n", tail.len()));
    for line in tail.into_iter().rev() {
        report.push_str(&format!("  {line}\n"));
    }
    report
}

/// Ends a process by id. The watchdog thread has no handle on the `Child`, which the test owns.
fn end_process(pid: u32) {
    let pid = pid.to_string();
    let _ = if cfg!(windows) {
        Command::new("taskkill").args(["/F", "/PID", &pid]).output()
    } else {
        Command::new("kill").args(["-KILL", &pid]).output()
    };
}

static NEXT_SERVER: AtomicI64 = AtomicI64::new(0);

pub fn spawn_server() -> TestServer {
    spawn_server_with_env(&[])
}

pub fn spawn_server_with_env(env: &[(&str, &std::path::Path)]) -> TestServer {
    spawn_server_with_budget(env, server_budget())
}

fn spawn_server_with_budget(env: &[(&str, &std::path::Path)], budget: Duration) -> TestServer {
    let server_path = server_binary_path();
    eprintln!("spec42 integration harness launch_mode={INTEGRATION_LAUNCH_MODE}");
    let mut command = Command::new(&server_path);
    for (name, value) in env {
        command.env(name, value);
        if *name == "SPEC42_LSP_TEST_STDLIB" {
            command.env("SPEC42_LIBRARY_FULL_SCAN", "1");
        }
    }
    // The server's stderr goes to a file rather than nowhere, so the watchdog can show what a
    // stalled server last logged. A file cannot fill up and block the server the way a pipe can.
    let stderr_path = std::env::temp_dir().join(format!(
        "spec42-lsp-test-{}-{}.stderr",
        std::process::id(),
        NEXT_SERVER.fetch_add(1, Ordering::SeqCst)
    ));
    let stderr = std::fs::File::create(&stderr_path)
        .map(Stdio::from)
        .unwrap_or_else(|_| Stdio::null());
    let child = command
        // Keep debug diagnostics enabled during integration tests.
        .env("SPEC42_ELK_DEBUG", "1")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(stderr)
        .spawn()
        .unwrap_or_else(|err| panic!("spawn server binary {}: {err}", server_path.display()));
    let finished = watch_server(child.id(), stderr_path.clone(), budget);
    TestServer {
        child,
        stderr_path,
        _finished: finished,
    }
}

/// A read that would block forever returns once the watchdog ends the server (issue #248).
#[test]
fn watchdog_ends_a_server_a_test_is_blocked_on() {
    let mut server = spawn_server_with_budget(&[], Duration::from_secs(1));
    let mut stdout = server.stdout.take().expect("stdout");
    // Nothing was sent, so the server never writes: without the watchdog this read never returns.
    let started = std::time::Instant::now();
    assert_eq!(read_message(&mut stdout), None);
    assert!(
        started.elapsed() >= Duration::from_secs(1),
        "the server must live for its whole budget"
    );
}

/// The report names what is observable from outside: thread states (Linux) and stderr.
#[test]
fn watchdog_reports_thread_states_and_stderr_of_a_live_server() {
    let server = spawn_server();
    let report = stalled_server_report(server.id(), &server.stderr_path);
    assert!(report.contains("server stderr (last"), "{report}");
    if cfg!(target_os = "linux") {
        assert!(report.contains("server threads:"), "{report}");
    }
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

/// Block until the server announces a workspace publication with `spec42/publicationChanged`.
///
/// `publish_workspace_diagnostics` sends that notification only after every per-document
/// diagnostics task of its sweep has finished, so it is the barrier for the whole sweep. Waiting
/// for one document's `publishDiagnostics` is not: the announcement can still be in flight after
/// that document's diagnostics, and after the response to a request sent in between.
pub fn wait_for_publication_changed(stdout: &mut std::process::ChildStdout) {
    loop {
        let msg = read_message(stdout)
            .unwrap_or_else(|| panic!("server closed before announcing a publication"));
        let Ok(json) = serde_json::from_str::<serde_json::Value>(&msg) else {
            continue;
        };
        if json["method"].as_str() == Some("spec42/publicationChanged") {
            return;
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
