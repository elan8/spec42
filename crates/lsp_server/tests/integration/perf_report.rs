//! Shared helpers for report-only LSP performance tests.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use super::harness::{next_id, read_message, send_message};
use serde::Serialize;
use walkdir::WalkDir;

pub fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repo root")
        .to_path_buf()
}

#[derive(Debug, Clone, Serialize)]
pub struct DurationSummary {
    pub total_ms: u128,
    pub p50_ms: u128,
    pub p95_ms: u128,
    pub max_ms: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct FilePerfEntry {
    pub path: String,
    pub bytes: u64,
    pub read_ms: u128,
    pub parse_ms: u128,
    pub parse_ok: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct FixturePerfSummary {
    pub files: usize,
    pub total_bytes: u64,
    pub scan_ms: u128,
    pub read: DurationSummary,
    pub parse: DurationSummary,
    pub slowest_files_by_parse: Vec<FilePerfEntry>,
    pub largest_files: Vec<FilePerfEntry>,
}

#[derive(Debug)]
pub struct CapturedResponse {
    pub raw: String,
    pub json: serde_json::Value,
    pub elapsed_ms: u128,
    pub perf_events: Vec<serde_json::Value>,
}

pub fn percentile_ms(values: &[u128], percentile: u32) -> u128 {
    if values.is_empty() {
        return 0;
    }
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let pct = percentile.clamp(0, 100) as usize;
    let index = ((sorted.len() - 1) * pct).div_ceil(100);
    sorted[index]
}

pub fn duration_summary(values: &[u128]) -> DurationSummary {
    DurationSummary {
        total_ms: values.iter().sum(),
        p50_ms: percentile_ms(values, 50),
        p95_ms: percentile_ms(values, 95),
        max_ms: values.iter().copied().max().unwrap_or(0),
    }
}

fn relative_perf_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub fn collect_fixture_perf(root: &Path) -> FixturePerfSummary {
    let scan_start = Instant::now();
    let mut files = Vec::new();
    for entry in WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(|entry| entry.ok())
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let ext = entry.path().extension().and_then(|ext| ext.to_str());
        if ext != Some("sysml") && ext != Some("kerml") {
            continue;
        }

        let read_start = Instant::now();
        let content = std::fs::read_to_string(entry.path()).unwrap_or_default();
        let read_ms = read_start.elapsed().as_millis();
        let bytes = content.len() as u64;

        let parse_start = Instant::now();
        let parse_ok = sysml_v2_parser::parse(&content).is_ok();
        let parse_ms = parse_start.elapsed().as_millis();

        files.push(FilePerfEntry {
            path: relative_perf_path(root, entry.path()),
            bytes,
            read_ms,
            parse_ms,
            parse_ok,
        });
    }
    let scan_ms = scan_start.elapsed().as_millis();
    files.sort_by(|left, right| left.path.cmp(&right.path));

    let read_values = files.iter().map(|file| file.read_ms).collect::<Vec<_>>();
    let parse_values = files.iter().map(|file| file.parse_ms).collect::<Vec<_>>();
    let total_bytes = files.iter().map(|file| file.bytes).sum();

    let mut slowest_files_by_parse = files.clone();
    slowest_files_by_parse.sort_by(|left, right| {
        right
            .parse_ms
            .cmp(&left.parse_ms)
            .then_with(|| left.path.cmp(&right.path))
    });
    slowest_files_by_parse.truncate(5);

    let mut largest_files = files.clone();
    largest_files.sort_by(|left, right| {
        right
            .bytes
            .cmp(&left.bytes)
            .then_with(|| left.path.cmp(&right.path))
    });
    largest_files.truncate(5);

    FixturePerfSummary {
        files: files.len(),
        total_bytes,
        scan_ms,
        read: duration_summary(&read_values),
        parse: duration_summary(&parse_values),
        slowest_files_by_parse,
        largest_files,
    }
}

fn perf_event_from_message(message: &str) -> Option<serde_json::Value> {
    const MARKER: &str = "[SysML][perf] ";
    let json_start = message.find(MARKER)? + MARKER.len();
    serde_json::from_str(&message[json_start..]).ok()
}

fn collect_perf_event(message: &str, events: &mut Vec<serde_json::Value>) {
    let Ok(json) = serde_json::from_str::<serde_json::Value>(message) else {
        return;
    };
    if json["method"].as_str() != Some("window/logMessage") {
        return;
    }
    let Some(message) = json["params"]["message"].as_str() else {
        return;
    };
    if let Some(event) = perf_event_from_message(message) {
        events.push(event);
    }
}

pub fn request_with_perf_capture(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut std::process::ChildStdout,
    method: &str,
    params: serde_json::Value,
) -> CapturedResponse {
    let id = next_id();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": id,
        "method": method,
        "params": params
    });
    let start = Instant::now();
    send_message(stdin, &req.to_string());
    let mut perf_events = Vec::new();
    loop {
        let msg = read_message(stdout).expect("expected JSON-RPC message");
        collect_perf_event(&msg, &mut perf_events);
        let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();
        if json.get("id").and_then(|value| value.as_i64()) == Some(id) {
            return CapturedResponse {
                raw: msg,
                json,
                elapsed_ms: start.elapsed().as_millis(),
                perf_events,
            };
        }
    }
}

pub fn latest_perf_event<'a>(
    events: &'a [serde_json::Value],
    event_name: &str,
) -> Option<&'a serde_json::Value> {
    events
        .iter()
        .rev()
        .find(|event| event["event"].as_str() == Some(event_name))
}

/// Block until startup indexing finishes so `semantic_state_version` is stable for cache tests.
pub fn wait_for_startup_scan(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut std::process::ChildStdout,
    existing_events: &[serde_json::Value],
    deadline: Duration,
) -> Vec<serde_json::Value> {
    use std::time::Instant;

    if latest_perf_event(existing_events, "backend:startupScanPhases").is_some() {
        return Vec::new();
    }

    let wait_start = Instant::now();
    let mut perf_events = Vec::new();
    loop {
        if latest_perf_event(&perf_events, "backend:startupScanPhases").is_some() {
            return perf_events;
        }
        if wait_start.elapsed() >= deadline {
            panic!(
                "startup scan did not complete within {}s",
                deadline.as_secs()
            );
        }
        let capture = request_with_perf_capture(
            stdin,
            stdout,
            "workspace/symbol",
            serde_json::json!({ "query": "" }),
        );
        perf_events.extend(capture.perf_events);
        if latest_perf_event(&perf_events, "backend:startupScanPhases").is_some() {
            return perf_events;
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}

pub fn value_ms(event: Option<&serde_json::Value>, key: &str) -> u128 {
    event
        .and_then(|event| event.get(key))
        .and_then(|value| value.as_u64())
        .unwrap_or(0) as u128
}

pub fn slowest_phase_entries(phases: &HashMap<&'static str, u128>) -> Vec<serde_json::Value> {
    let mut entries = phases
        .iter()
        .map(|(name, ms)| (*name, *ms))
        .collect::<Vec<_>>();
    entries.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(right.0)));
    entries
        .into_iter()
        .take(8)
        .map(|(name, ms)| serde_json::json!({ "name": name, "ms": ms }))
        .collect()
}

pub fn write_perf_report(report: &serde_json::Value, filename: &str) -> PathBuf {
    let output_dir = repo_root().join("target").join("spec42-perf");
    std::fs::create_dir_all(&output_dir).expect("create perf report dir");
    let output_path = output_dir.join(filename);
    std::fs::write(
        &output_path,
        serde_json::to_string_pretty(report).expect("serialize perf report"),
    )
    .expect("write perf report");
    output_path
}

pub fn emit_perf_report(report: &serde_json::Value, filename: &str) -> PathBuf {
    let output_path = write_perf_report(report, filename);
    eprintln!(
        "SPEC42_PERF_REPORT {}",
        serde_json::to_string(report).expect("serialize perf report line")
    );
    eprintln!("SPEC42_PERF_REPORT_PATH {}", output_path.display());
    output_path
}

pub fn workspace_loaded_files(response: &serde_json::Value) -> usize {
    response["result"]["workspaceModel"]["summary"]["loadedFiles"]
        .as_u64()
        .unwrap_or(0) as usize
}

pub fn graph_node_count(response: &serde_json::Value) -> usize {
    response["result"]["graph"]["nodes"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0)
}

pub fn graph_edge_count(response: &serde_json::Value) -> usize {
    response["result"]["graph"]["edges"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0)
}

pub fn visualization_model_build_time_ms(response: &serde_json::Value) -> u128 {
    response["result"]["stats"]["modelBuildTimeMs"]
        .as_u64()
        .unwrap_or(0) as u128
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn performance_percentile_helpers_are_stable() {
        assert_eq!(percentile_ms(&[], 95), 0);
        assert_eq!(percentile_ms(&[7], 50), 7);
        assert_eq!(percentile_ms(&[1, 10, 3, 7], 50), 7);
        assert_eq!(percentile_ms(&[1, 10, 3, 7], 95), 10);
    }

    #[test]
    fn performance_slowest_phase_entries_sort_by_duration_then_name() {
        let phases = HashMap::from([
            ("parse", 20),
            ("scan", 20),
            ("relink", 40),
            ("visualization", 5),
        ]);
        let entries = slowest_phase_entries(&phases);
        assert_eq!(entries[0]["name"], "relink");
        assert_eq!(entries[1]["name"], "parse");
        assert_eq!(entries[2]["name"], "scan");
    }
}
