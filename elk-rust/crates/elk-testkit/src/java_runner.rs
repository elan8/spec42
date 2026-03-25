use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::Value;

fn elk_rust_root() -> PathBuf {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // elk-rust/crates/elk-testkit -> elk-rust/crates -> elk-rust
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("elk-rust root not found")
        .to_path_buf()
}

fn parse_json_from_mixed_output(stdout: &str) -> Result<Value, String> {
    let bytes = stdout.as_bytes();
    let mut start_positions = Vec::new();
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'{' || b == b'[' {
            start_positions.push(idx);
        }
    }
    if start_positions.is_empty() {
        return Err("java runner returned invalid JSON: expected value at line 1 column 1".to_string());
    }

    let mut end_positions = Vec::new();
    for (idx, &b) in bytes.iter().enumerate() {
        if b == b'}' || b == b']' {
            end_positions.push(idx);
        }
    }

    for start in &start_positions {
        for end in end_positions.iter().rev() {
            if end < start {
                continue;
            }
            let candidate = stdout[*start..=*end].trim();
            if candidate.is_empty() {
                continue;
            }
            if let Ok(value) = serde_json::from_str::<Value>(candidate) {
                return Ok(value);
            }
        }
    }

    Err("java runner returned invalid JSON: expected value at line 1 column 2".to_string())
}

fn run_java_runner_command(root: &Path, input_json_path: &Path) -> Result<Output, String> {
    #[cfg(windows)]
    {
        let script = root.join("scripts").join("run-elk-java-json.ps1");
        for shell in ["pwsh", "powershell"] {
            match Command::new(shell)
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    script.to_string_lossy().as_ref(),
                    "-InputJson",
                    input_json_path.to_string_lossy().as_ref(),
                ])
                .output()
            {
                Ok(output) => return Ok(output),
                Err(err) if err.kind() == std::io::ErrorKind::NotFound => continue,
                Err(err) => {
                    return Err(format!(
                        "failed to launch Java runner shell {}: {}",
                        shell, err
                    ));
                }
            }
        }
        Err("Java runner is unavailable in the current environment".to_string())
    }

    #[cfg(not(windows))]
    {
        let script = root.join("scripts").join("run-elk-java-json.sh");
        Command::new("sh")
            .arg(script)
            .arg(input_json_path)
            .output()
            .map_err(|err| {
                if err.kind() == std::io::ErrorKind::NotFound {
                    "Java runner is unavailable in the current environment".to_string()
                } else {
                    format!("failed to launch Java runner shell: {}", err)
                }
            })
    }
}

/// Run the Java ELK JSON runner on an ELK Graph JSON value and return the Java-produced output JSON.
///
/// This is intended for debugging and parity repros where we want to run *Java* ELK on exactly the
/// same input graph/options used by Rust.
pub fn run_java_elk_json(input_json: &Value) -> Result<Value, String> {
    let root = elk_rust_root();

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    let pid = std::process::id();
    let file_name = format!("spec42-elk-java-input-{pid}-{ts}.json");
    let in_path = std::env::temp_dir().join(file_name);

    let input_str =
        serde_json::to_string_pretty(input_json).map_err(|e| format!("serialize input json: {e}"))?;
    fs::write(&in_path, input_str).map_err(|e| format!("write temp input json: {e}"))?;

    let out = run_java_runner_command(&root, &in_path)?;
    // Best-effort cleanup.
    let _ = fs::remove_file(&in_path);

    let stdout = String::from_utf8_lossy(&out.stdout);
    let stderr = String::from_utf8_lossy(&out.stderr);
    let combined = format!("{stdout}\n{stderr}");

    if !out.status.success() {
        return Err(format!(
            "java runner failed (exit {}):\nstdout:\n{}\nstderr:\n{}",
            out.status, stdout, stderr
        ));
    }

    // When the runner exits successfully, trust the exit code and attempt to parse JSON from stdout.
    // Runner diagnostics should go to stderr; stdout is reserved for the JSON payload.
    parse_json_from_mixed_output(&stdout).map_err(|err| {
        format!(
            "{err}\n\nRunner stdout/stderr (first 200 lines):\n{}",
            combined.lines().take(200).collect::<Vec<_>>().join("\n")
        )
    })
}

