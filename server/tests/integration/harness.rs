//! Shared LSP integration test harness: spawn server, send/read JSON-RPC messages.

use std::io::{Read, Write};
use std::process::{Child, Command, Stdio};
use std::sync::atomic::{AtomicI64, Ordering};

pub static NEXT_ID: AtomicI64 = AtomicI64::new(1);

pub fn server_binary_path() -> std::path::PathBuf {
    let current_exe = std::env::current_exe().expect("current_exe");
    let dir = current_exe
        .parent()
        .and_then(|p| p.parent())
        .expect("test binary has parent dir (target/debug)");
    let name = std::env::consts::EXE_SUFFIX;
    let server_name = if name.is_empty() {
        "spec42".to_string()
    } else {
        format!("spec42{}", name)
    };
    dir.join(server_name)
}

pub fn spawn_server() -> Child {
    let bin = server_binary_path();
    assert!(bin.exists(), "server binary not found at {:?}", bin);
    Command::new(&bin)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn server")
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
