//! Workspace scan integration tests.

use std::collections::HashMap;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};
use super::perf_report::{
    collect_fixture_perf, emit_perf_report, graph_edge_count, graph_node_count, latest_perf_event,
    request_with_perf_capture, slowest_phase_entries, value_ms, workspace_loaded_files,
};
use lsp_server::common::util;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("repo root")
        .to_path_buf()
}

/// Workspace scan: definition file exists only on disk; we never didOpen it.
/// Proves the server indexes files from the workspace root and goto_definition resolves across them.
#[test]
fn lsp_workspace_scan_goto_definition() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");

    std::fs::write(root.join("def.sysml"), "package P { part def Engine; }").expect("write def");
    std::fs::write(root.join("use.sysml"), "package Q { part e : Engine; }").expect("write use");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let uri_use = url::Url::from_file_path(root.join("use.sysml")).expect("use uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Wait for workspace scan to index def.sysml and use.sysml from disk
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Open only the file that contains the usage; def.sysml is only in the index from the scan
    let did_open_use = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri_use.as_str(),
                "languageId": "sysml",
                "version": 1,
                "text": "package Q { part e : Engine; }"
            }
        }
    });
    send_message(&mut stdin, &did_open_use.to_string());
    std::thread::sleep(std::time::Duration::from_millis(50));

    let def_id = next_id();
    let def_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": def_id,
        "method": "textDocument/definition",
        "params": {
            "textDocument": { "uri": uri_use.as_str() },
            "position": { "line": 0, "character": 22 }
        }
    });
    send_message(&mut stdin, &def_req.to_string());
    let def_resp = read_response(&mut stdout, def_id).expect("definition response");
    let def_json: serde_json::Value =
        serde_json::from_str(&def_resp).expect("parse definition response");
    assert_eq!(def_json["id"], def_id);
    let result = &def_json["result"];
    let uri = result["uri"]
        .as_str()
        .expect("definition should return location with uri");
    assert!(
        uri.contains("def.sysml"),
        "goto_definition must resolve to def.sysml (loaded by workspace scan), got uri: {}",
        uri
    );

    let _ = child.kill();
}

#[test]
fn lsp_goto_definition_resolves_qualified_name_reference() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");
    let lib_dir = root.join("lib");
    std::fs::create_dir_all(&lib_dir).expect("create lib dir");

    std::fs::write(
        lib_dir.join("si.sysml"),
        "standard library package SI { attribute def V; }",
    )
    .expect("write SI library");
    std::fs::write(root.join("use.sysml"), "package P { attribute x : SI::V; }")
        .expect("write use");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let use_uri = url::Url::from_file_path(root.join("use.sysml")).expect("use uri");
    let lib_path = lib_dir.canonicalize().expect("canonical lib path");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" },
            "initializationOptions": {
                "libraryPaths": [lib_path.to_string_lossy().to_string()]
            }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());
    std::thread::sleep(std::time::Duration::from_millis(700));

    let did_open_use = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": use_uri.as_str(),
                "languageId": "sysml",
                "version": 1,
                "text": "package P { attribute x : SI::V; }"
            }
        }
    });
    send_message(&mut stdin, &did_open_use.to_string());
    std::thread::sleep(std::time::Duration::from_millis(100));

    let def_id = next_id();
    let def_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": def_id,
        "method": "textDocument/definition",
        "params": {
            "textDocument": { "uri": use_uri.as_str() },
            "position": { "line": 0, "character": 29 }
        }
    });
    send_message(&mut stdin, &def_req.to_string());
    let def_resp = read_response(&mut stdout, def_id).expect("definition response");
    let def_json: serde_json::Value =
        serde_json::from_str(&def_resp).expect("parse definition response");
    let result = &def_json["result"];
    let uri = result["uri"]
        .as_str()
        .expect("definition should return scalar location");
    assert!(
        uri.contains("si.sysml"),
        "qualified SI::V should resolve into SI library file, got uri: {}",
        uri
    );

    let _ = child.kill();
}

#[test]
fn lsp_publishes_diagnostics_for_loose_file_on_did_open() {
    // did_open stores the document and publishes diagnostics using whatever graph is
    // already in memory. It does NOT trigger library scanning — libraries are only
    // loaded during the startup scan. A loose file with unresolved library imports
    // will show semantic diagnostics; that is the correct behaviour.
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");
    let lib_dir = root.join("lib");
    std::fs::create_dir_all(&lib_dir).expect("create lib dir");

    std::fs::write(
        lib_dir.join("ScalarValues.sysml"),
        "standard library package ScalarValues { attribute def Real; }",
    )
    .expect("write ScalarValues library");

    let lib_path = lib_dir.canonicalize().expect("canonical lib path");
    let loose_uri = "file:///outside-workspace/loose.sysml";
    let loose_text = r#"
        package P {
            private import ScalarValues::Real;
            attribute x : Real;
        }
    "#;

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" },
            "initializationOptions": {
                "libraryPaths": [lib_path.to_string_lossy().to_string()]
            }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    let did_open_loose = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": loose_uri,
                "languageId": "sysml",
                "version": 1,
                "text": loose_text
            }
        }
    });
    send_message(&mut stdin, &did_open_loose.to_string());

    // Wait for publishDiagnostics for the loose file using a barrier request.
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut saw_loose_publish = false;
    while Instant::now() < deadline && !saw_loose_publish {
        let barrier_id = next_id();
        let barrier_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": barrier_id,
            "method": "workspace/symbol",
            "params": { "query": "" }
        });
        send_message(&mut stdin, &barrier_req.to_string());

        loop {
            let msg =
                read_message(&mut stdout).expect("expected message while waiting for barrier");
            let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();
            if json["method"].as_str() == Some("textDocument/publishDiagnostics")
                && json["params"]["uri"]
                    .as_str()
                    .map(|uri| uri.eq_ignore_ascii_case(loose_uri))
                    .unwrap_or(false)
            {
                saw_loose_publish = true;
            }
            if json["id"].as_i64() == Some(barrier_id) {
                break;
            }
        }
    }

    assert!(
        saw_loose_publish,
        "expected diagnostics to be published for loose file after did_open"
    );

    let _ = child.kill();
}

#[test]
fn lsp_workspace_scan_clears_unresolved_for_wildcard_imported_workspace_types() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");

    std::fs::write(
        root.join("RoboticsCore.sysml"),
        r#"
        package RoboticsCore {
            attribute def Name;
        }
        "#,
    )
    .expect("write RoboticsCore");
    std::fs::write(
        root.join("RobotAutonomy.sysml"),
        r#"
        package RobotAutonomy {
            import RoboticsCore::*;

            part def BehaviorModule {
                attribute behaviorName : Name;
            }
        }
        "#,
    )
    .expect("write RobotAutonomy");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let autonomy_uri =
        url::Url::from_file_path(root.join("RobotAutonomy.sysml")).expect("autonomy uri");
    let autonomy_text =
        std::fs::read_to_string(root.join("RobotAutonomy.sysml")).expect("read RobotAutonomy");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
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
            "textDocument": {
                "uri": autonomy_uri.as_str(),
                "languageId": "sysml",
                "version": 1,
                "text": autonomy_text
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());

    std::thread::sleep(std::time::Duration::from_millis(700));

    let barrier_id = next_id();
    let barrier_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": barrier_id,
        "method": "workspace/symbol",
        "params": { "query": "" }
    });
    send_message(&mut stdin, &barrier_req.to_string());

    let mut saw_publish = false;
    let mut last_diagnostics = Vec::new();
    loop {
        let msg = read_message(&mut stdout).expect("expected message while waiting for barrier");
        let json: serde_json::Value = serde_json::from_str(&msg).unwrap_or_default();
        if json["method"].as_str() == Some("textDocument/publishDiagnostics")
            && json["params"]["uri"]
                .as_str()
                .map(|uri| uri.eq_ignore_ascii_case(autonomy_uri.as_str()))
                .unwrap_or(false)
        {
            saw_publish = true;
            last_diagnostics = json["params"]["diagnostics"]
                .as_array()
                .cloned()
                .unwrap_or_default();
        }
        if json["id"].as_i64() == Some(barrier_id) {
            break;
        }
    }

    assert!(
        saw_publish,
        "expected diagnostics to be published for RobotAutonomy"
    );
    assert!(
        !last_diagnostics.iter().any(|d| {
            d["source"].as_str() == Some("semantic")
                && d["code"].as_str() == Some("unresolved_type_reference")
        }),
        "expected no unresolved_type_reference diagnostics after workspace scan, got: {last_diagnostics:#?}"
    );

    let _ = child.kill();
}

/// When SYSML_V2_RELEASE_DIR is set, index that folder and assert workspace/symbol finds symbols.
/// Validates workspace awareness against the official OMG SysML v2 repo.
const SYSML_V2_RELEASE_DIR_ENV: &str = "SYSML_V2_RELEASE_DIR";
const SYSML_STD_LIB_DIR_ENV: &str = "SYSML_STD_LIB_DIR";
const PINNED_STDLIB_CONFIG_RAW: &str = include_str!("../../../../config/standard-library.json");

fn is_si_sysml_path(path: &str) -> bool {
    path.ends_with("/Domain%20Libraries/Quantities%20and%20Units/SI.sysml")
        || path.ends_with("/Domain Libraries/Quantities and Units/SI.sysml")
}

fn flatten_library_tree_symbols(result: &serde_json::Value) -> Vec<&serde_json::Value> {
    let mut out = Vec::new();
    if let Some(sources) = result["sources"].as_array() {
        for source in sources {
            if let Some(packages) = source["packages"].as_array() {
                for package in packages {
                    if let Some(symbols) = package["symbols"].as_array() {
                        out.extend(symbols.iter());
                    }
                }
            }
        }
    }
    out
}

fn resolve_sysml_library_root_for_tests() -> Option<PathBuf> {
    if let Some(v) = std::env::var_os(SYSML_STD_LIB_DIR_ENV) {
        let p = PathBuf::from(v);
        if p.is_dir() {
            return Some(p);
        }
    }

    if let Some(v) = std::env::var_os(SYSML_V2_RELEASE_DIR_ENV) {
        let release_root = PathBuf::from(v);
        let candidate = release_root.join("sysml.library");
        if candidate.is_dir() {
            return Some(candidate);
        }
    }

    if let Some(default_path) = legacy_vscode_stdlib_fallback_path() {
        if default_path.is_dir() {
            return Some(default_path);
        }
    }
    None
}

fn legacy_vscode_stdlib_fallback_path() -> Option<PathBuf> {
    #[derive(serde::Deserialize)]
    struct PinnedStdlibConfig {
        version: String,
        #[serde(rename = "contentPath")]
        content_path: String,
    }

    let config: PinnedStdlibConfig = serde_json::from_str(PINNED_STDLIB_CONFIG_RAW).ok()?;
    let app_data = std::env::var_os("APPDATA")?;
    Some(
        PathBuf::from(app_data)
            .join("Code")
            .join("User")
            .join("globalStorage")
            .join("elan8.spec42")
            .join("standard-library")
            .join(config.version)
            .join(config.content_path),
    )
}

#[test]
fn lsp_workspace_scan_sysml_release() {
    let release_root = match std::env::var_os(SYSML_V2_RELEASE_DIR_ENV) {
        Some(v) => PathBuf::from(v),
        None => {
            eprintln!(
                "Skipping lsp_workspace_scan_sysml_release: set {} to the SysML-v2-Release clone root",
                SYSML_V2_RELEASE_DIR_ENV
            );
            return;
        }
    };
    if !release_root.is_dir() {
        eprintln!("Skipping: {} is not a directory", release_root.display());
        return;
    }

    let root_uri = match url::Url::from_file_path(&release_root) {
        Ok(u) => u,
        Err(_) => {
            eprintln!(
                "Skipping: cannot build file URL for {}",
                release_root.display()
            );
            return;
        }
    };

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Allow time for scanning a large repo before the first attempt.
    std::thread::sleep(std::time::Duration::from_secs(3));

    // Scanning the full SysML-v2-Release corpus can take longer than a fixed sleep under
    // CPU-constrained CI runners; retry instead of asserting on the first response.
    let mut result_count = 0usize;
    for _ in 0..20 {
        let sym_id = next_id();
        let sym_req = serde_json::json!({
            "jsonrpc": "2.0",
            "id": sym_id,
            "method": "workspace/symbol",
            "params": { "query": "Part" }
        });
        send_message(&mut stdin, &sym_req.to_string());
        let sym_resp = read_response(&mut stdout, sym_id).expect("workspace/symbol response");
        let sym_json: serde_json::Value =
            serde_json::from_str(&sym_resp).expect("parse workspace/symbol response");
        assert_eq!(sym_json["id"], sym_id);
        let results = sym_json["result"]
            .as_array()
            .expect("workspace/symbol returns array");
        result_count = results.len();
        if result_count > 0 {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    assert!(
        result_count > 0,
        "workspace/symbol over SysML-v2-Release should return at least one symbol for query 'Part'"
    );

    let _ = child.kill();
}

/// Validates that SI.sysml contributes more than a trivial number of symbols to librarySearch.
/// This catches regressions where parser succeeds but graph/symbol coverage is too low.
#[test]
fn lsp_library_search_si_file_has_rich_symbol_coverage() {
    let library_root = match resolve_sysml_library_root_for_tests() {
        Some(v) => v,
        None => {
            let fallback = legacy_vscode_stdlib_fallback_path()
                .map(|p| p.display().to_string())
                .unwrap_or_else(|| "<not available>".to_string());
            eprintln!(
                "Skipping lsp_library_search_si_file_has_rich_symbol_coverage: set {} (sysml.library root) or {} (SysML-v2-Release root); fallback path not found: {}",
                SYSML_STD_LIB_DIR_ENV,
                SYSML_V2_RELEASE_DIR_ENV,
                fallback
            );
            return;
        }
    };

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" },
            "initializationOptions": {
                "libraryPaths": [library_root.to_string_lossy().to_string()]
            }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // allow library indexing to complete for large release trees
    std::thread::sleep(std::time::Duration::from_secs(3));

    let req_id = next_id();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": req_id,
        "method": "sysml/librarySearch",
        "params": {
            "query": "",
            "limit": 5000
        }
    });
    send_message(&mut stdin, &req.to_string());
    let resp = read_response(&mut stdout, req_id).expect("librarySearch response");
    let json: serde_json::Value = serde_json::from_str(&resp).expect("parse librarySearch");
    let items = flatten_library_tree_symbols(&json["result"]);

    let si_items: Vec<&serde_json::Value> = items
        .into_iter()
        .filter(|item| item["path"].as_str().map(is_si_sysml_path).unwrap_or(false))
        .collect();
    let si_count = si_items.len();

    let mut si_by_detail: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut si_names: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
    for item in &si_items {
        if let Some(detail) = item["kind"].as_str() {
            *si_by_detail.entry(detail.to_string()).or_insert(0) += 1;
        }
        if let Some(name) = item["name"].as_str() {
            si_names.insert(name.to_string());
        }
    }

    let expected_names = ["metre", "kilogram", "second", "tonne", "arcmin", "arcsec"];
    let missing_names: Vec<&str> = expected_names
        .iter()
        .copied()
        .filter(|name| !si_names.contains(*name))
        .collect();

    assert!(
        si_count > 20,
        "Expected SI.sysml to contribute >20 symbols, got {}. detailCounts={:?}, sampleNames={:?}, samplePaths={:?}",
        si_count,
        si_by_detail,
        si_names.iter().take(20).cloned().collect::<Vec<_>>(),
        si_items
            .iter()
            .take(20)
            .filter_map(|item| item["path"].as_str().map(str::to_string))
            .collect::<Vec<_>>()
    );
    assert!(
        missing_names.is_empty(),
        "Expected SI.sysml to include symbols {:?}, missing {:?}. detailCounts={:?}, sampleNames={:?}",
        expected_names,
        missing_names,
        si_by_detail,
        si_names.iter().take(60).cloned().collect::<Vec<_>>()
    );

    let _ = child.kill();
}

#[test]
fn lsp_library_search_custom_method_returns_library_results() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");
    let lib_dir = root.join("lib");
    std::fs::create_dir_all(&lib_dir).expect("create lib dir");
    std::fs::write(
        lib_dir.join("standard.sysml"),
        "standard library package Lib { part def Engine; part def EngineController; }",
    )
    .expect("write library file");
    std::fs::write(root.join("main.sysml"), "package Main { part x : Engine; }")
        .expect("write main file");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let lib_path = lib_dir.canonicalize().expect("canonical lib path");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" },
            "initializationOptions": {
                "libraryPaths": [lib_path.to_string_lossy().to_string()]
            }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());
    std::thread::sleep(std::time::Duration::from_millis(700));

    let req_id = next_id();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": req_id,
        "method": "sysml/librarySearch",
        "params": {
            "query": "engine",
            "limit": 20
        }
    });
    send_message(&mut stdin, &req.to_string());
    let resp = read_response(&mut stdout, req_id).expect("library search response");
    let json: serde_json::Value = serde_json::from_str(&resp).expect("parse response");
    let items = flatten_library_tree_symbols(&json["result"]);
    assert!(!items.is_empty(), "library search should return results");
    let has_engine = items.iter().any(|item| {
        item["name"]
            .as_str()
            .map(|name| name.eq_ignore_ascii_case("Engine"))
            .unwrap_or(false)
    });
    assert!(has_engine, "Engine should be in library search results");

    // Tree contract: package nodes should not include duplicate module child equal to package name.
    let sources = json["result"]["sources"].as_array().expect("sources array");
    let has_duplicate_module = sources.iter().any(|source| {
        source["packages"]
            .as_array()
            .into_iter()
            .flatten()
            .any(|pkg| {
                let pkg_name = pkg["name"].as_str().unwrap_or_default();
                pkg["symbols"].as_array().into_iter().flatten().any(|sym| {
                    sym["kind"].as_str() == Some("module")
                        && sym["name"]
                            .as_str()
                            .map(|n| n.eq_ignore_ascii_case(pkg_name))
                            .unwrap_or(false)
                })
            })
    });
    assert!(
        !has_duplicate_module,
        "package should not repeat its module symbol as a child"
    );

    let _ = child.kill();
}

#[test]
fn lsp_library_search_uses_declared_name_for_allocation_def() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");
    let lib_dir = root.join("lib");
    std::fs::create_dir_all(&lib_dir).expect("create lib dir");
    std::fs::write(
        lib_dir.join("allocations.sysml"),
        "standard library package Allocations { allocation def Allocation :> BinaryConnection; }",
    )
    .expect("write allocation library file");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let lib_path = lib_dir.canonicalize().expect("canonical lib path");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" },
            "initializationOptions": {
                "libraryPaths": [lib_path.to_string_lossy().to_string()]
            }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());
    std::thread::sleep(std::time::Duration::from_millis(700));

    let req_id = next_id();
    let req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": req_id,
        "method": "sysml/librarySearch",
        "params": {
            "query": "allocation",
            "limit": 20
        }
    });
    send_message(&mut stdin, &req.to_string());
    let resp = read_response(&mut stdout, req_id).expect("library search response");
    let json: serde_json::Value = serde_json::from_str(&resp).expect("parse response");
    let items = flatten_library_tree_symbols(&json["result"]);
    let has_named_allocation = items.iter().any(|item| {
        item["name"].as_str() == Some("Allocation") && item["name"].as_str() != Some("def")
    });
    assert!(
        has_named_allocation,
        "expected allocation symbol to appear as 'Allocation' (not generic 'def'), got {:?}",
        items
            .iter()
            .map(|item| {
                (
                    item["name"].as_str().unwrap_or_default().to_string(),
                    item["kind"].as_str().unwrap_or_default().to_string(),
                )
            })
            .collect::<Vec<_>>()
    );

    let _ = child.kill();
}

#[test]
fn lsp_workspace_visualization_model_includes_all_workspace_systems() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");

    let drone_dir = root.join("drone").join("sysml");
    let timer_dir = root.join("timer").join("sysml");
    let intersection_dir = root.join("intersection").join("sysml");
    let computer_dir = root.join("computer");
    std::fs::create_dir_all(&drone_dir).expect("create drone dir");
    std::fs::create_dir_all(&timer_dir).expect("create timer dir");
    std::fs::create_dir_all(&intersection_dir).expect("create intersection dir");
    std::fs::create_dir_all(&computer_dir).expect("create computer dir");

    let drone_path = drone_dir.join("SurveillanceDrone.sysml");
    let timer_path = timer_dir.join("KitchenTimer.sysml");
    let intersection_path = intersection_dir.join("TrafficLightIntersection.sysml");
    let computer_path = computer_dir.join("laptop.sysml");

    std::fs::write(
        &drone_path,
        "package SurveillanceDrone { part def SurveillanceQuadrotorDrone; part droneInstance : SurveillanceQuadrotorDrone; }",
    )
    .expect("write drone");
    std::fs::write(
        &timer_path,
        "package KitchenTimer { part def KitchenTimer; part timerInstance : KitchenTimer; }",
    )
    .expect("write timer");
    std::fs::write(
        &intersection_path,
        "package TrafficLightIntersection { part def TrafficLightIntersection; part intersectionInstance : TrafficLightIntersection; }",
    )
    .expect("write intersection");
    std::fs::write(
        &computer_path,
        "package ComputerDemo { part def Laptop; part myComputer : Laptop; }",
    )
    .expect("write computer");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");
    let drone_uri = url::Url::from_file_path(&drone_path).expect("drone uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());

    // Allow workspace scan to index all files before requesting workspace visualization model.
    std::thread::sleep(std::time::Duration::from_millis(800));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": drone_uri.as_str() },
            "scope": ["graph", "workspaceVisualization"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let graph_nodes = model_json["result"]["graph"]["nodes"]
        .as_array()
        .expect("graph nodes array");
    let node_names: std::collections::HashSet<&str> = graph_nodes
        .iter()
        .filter_map(|node| node["name"].as_str())
        .collect();

    assert!(
        node_names.contains("SurveillanceDrone"),
        "workspace graph should include SurveillanceDrone package: {:?}",
        node_names
    );
    assert!(
        node_names.contains("KitchenTimer"),
        "workspace graph should include KitchenTimer package: {:?}",
        node_names
    );
    assert!(
        node_names.contains("TrafficLightIntersection"),
        "workspace graph should include TrafficLightIntersection package: {:?}",
        node_names
    );
    assert!(
        node_names.contains("ComputerDemo"),
        "workspace graph should include ComputerDemo package: {:?}",
        node_names
    );

    let _ = child.kill();
}

#[test]
fn performance_report_schema_keeps_required_top_level_keys() {
    let report = serde_json::json!({
        "schemaVersion": 1,
        "fixture": {},
        "phases": {},
        "modelRequests": {},
        "visualization": {},
        "counts": {},
        "budgets": {},
        "bottlenecks": {}
    });
    for key in [
        "schemaVersion",
        "fixture",
        "phases",
        "modelRequests",
        "visualization",
        "counts",
        "budgets",
        "bottlenecks",
    ] {
        assert!(
            report.get(key).is_some(),
            "missing required report key {key}"
        );
    }
}

#[test]
#[ignore = "local smoke only; fixture is tiny (vscode/testFixture/large-workspace) and not wired to CI — run manually to spot-check workspace model + document model request latency"]
fn lsp_large_workspace_performance_report() {
    let root = repo_root()
        .join("vscode")
        .join("testFixture")
        .join("workspaces")
        .join("large-workspace");
    if !root.is_dir() {
        eprintln!(
            "Skipping lsp_large_workspace_performance_report: {} is not a directory",
            root.display()
        );
        return;
    }

    let fixture_perf = collect_fixture_perf(&root);
    let root_uri = url::Url::from_file_path(&root).expect("large workspace root uri");
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "initializationOptions": {
                "performanceLogging": { "enabled": true },
                "workspace": { "maxFilesPerPattern": 1000 }
            },
            "clientInfo": { "name": "perf-report", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    let workspace_model_params = serde_json::json!({
        "textDocument": { "uri": root_uri.as_str() },
        "scope": ["graph", "workspaceVisualization"]
    });
    let workspace_model_capture = {
        let wait_start = Instant::now();
        loop {
            let capture = request_with_perf_capture(
                &mut stdin,
                &mut stdout,
                "sysml/model",
                workspace_model_params.clone(),
            );
            let loaded_files = workspace_loaded_files(&capture.json);
            let graph_nodes = graph_node_count(&capture.json);
            if loaded_files > 0 && graph_nodes > 0 {
                break capture;
            }
            if wait_start.elapsed() >= Duration::from_secs(20) {
                panic!(
                    "workspace model did not become ready within 20s; last response: {:#?}",
                    capture.json
                );
            }
            std::thread::sleep(Duration::from_millis(250));
        }
    };
    let mut perf_events = workspace_model_capture.perf_events.clone();
    let workspace_model_json = &workspace_model_capture.json;
    let indexed_documents = workspace_loaded_files(workspace_model_json);
    let workspace_graph_nodes = graph_node_count(workspace_model_json);
    let workspace_graph_edges = graph_edge_count(workspace_model_json);

    let alpha_uri = url::Url::from_file_path(root.join("Alpha.sysml")).expect("alpha uri");
    let model_capture = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/model",
        serde_json::json!({
            "textDocument": { "uri": alpha_uri.as_str() },
            "scope": ["graph", "stats"]
        }),
    );
    perf_events.extend(model_capture.perf_events.clone());
    let model_json = &model_capture.json;
    let graph_nodes = graph_node_count(model_json);
    let graph_edges = graph_edge_count(model_json);

    let visualization_capture = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "sysml/visualization",
        serde_json::json!({
            "workspaceRootUri": root_uri.as_str(),
            "view": "general-view"
        }),
    );
    perf_events.extend(visualization_capture.perf_events.clone());
    let post_visualization_barrier = request_with_perf_capture(
        &mut stdin,
        &mut stdout,
        "workspace/symbol",
        serde_json::json!({ "query": "" }),
    );
    perf_events.extend(post_visualization_barrier.perf_events.clone());
    let visualization_json = &visualization_capture.json;
    let view_candidates = visualization_json["result"]["viewCandidates"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);

    let startup_event = latest_perf_event(&perf_events, "backend:startupScanPhases");
    let workspace_response_event = latest_perf_event(
        &workspace_model_capture.perf_events,
        "backend:buildSysmlModelResponse",
    );
    let document_response_event = latest_perf_event(
        &model_capture.perf_events,
        "backend:buildSysmlModelResponse",
    );
    let visualization_event = latest_perf_event(&perf_events, "backend:sysmlVisualizationRequest");

    let mut phases = HashMap::new();
    phases.insert("fixtureScan", fixture_perf.scan_ms);
    phases.insert("fixtureReadTotal", fixture_perf.read.total_ms);
    phases.insert("fixtureParseTotal", fixture_perf.parse.total_ms);
    phases.insert(
        "startupDiscoverRead",
        value_ms(startup_event, "discoverReadMs"),
    );
    phases.insert(
        "startupParseWorkers",
        value_ms(startup_event, "parseWorkersMs"),
    );
    phases.insert("startupIngest", value_ms(startup_event, "ingestMs"));
    phases.insert("relinkTotal", value_ms(startup_event, "relinkTotalMs"));
    phases.insert(
        "relinkRebuildGraphs",
        value_ms(startup_event, "relinkRebuildGraphsMs"),
    );
    phases.insert(
        "relinkCrossEdgeResolution",
        value_ms(startup_event, "relinkCrossEdgeResolutionMs"),
    );
    phases.insert(
        "relinkWorkspaceRelationshipLinking",
        value_ms(startup_event, "relinkWorkspaceRelationshipLinkingMs"),
    );
    phases.insert(
        "relinkPendingRelationshipResolution",
        value_ms(startup_event, "relinkPendingRelationshipResolutionMs"),
    );
    phases.insert(
        "relinkExpressionEvaluation",
        value_ms(startup_event, "relinkExpressionEvaluationMs"),
    );
    phases.insert(
        "relinkRefreshSymbols",
        value_ms(startup_event, "relinkRefreshSymbolsMs"),
    );
    phases.insert("diagnostics", value_ms(startup_event, "diagnosticsMs"));
    phases.insert("workspaceModelRequest", workspace_model_capture.elapsed_ms);
    phases.insert("documentModelRequest", model_capture.elapsed_ms);
    phases.insert("visualizationRequest", visualization_capture.elapsed_ms);

    let report = serde_json::json!({
        "schemaVersion": 1,
        "fixture": {
            "name": "large-workspace",
            "path": "vscode/testFixture/workspaces/large-workspace",
            "files": fixture_perf.files,
            "totalBytes": fixture_perf.total_bytes,
            "localScanParse": fixture_perf.clone(),
        },
        "context": {
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
            "ci": std::env::var_os("CI").is_some(),
            "profile": "debug-test"
        },
        "phases": {
            "startup": startup_event.cloned().unwrap_or_else(|| serde_json::json!({})),
            "fixtureScanParse": {
                "scanMs": phases["fixtureScan"],
                "readTotalMs": phases["fixtureReadTotal"],
                "parseTotalMs": phases["fixtureParseTotal"],
                "readP95Ms": fixture_perf.read.p95_ms,
                "parseP95Ms": fixture_perf.parse.p95_ms
            },
            "modelResponse": {
                "workspace": workspace_response_event.cloned().unwrap_or_else(|| serde_json::json!({})),
                "document": document_response_event.cloned().unwrap_or_else(|| serde_json::json!({}))
            }
        },
        "modelRequests": {
            "workspace": {
                "elapsedMs": workspace_model_capture.elapsed_ms,
                "responseBytes": workspace_model_capture.raw.len(),
                "loadedFiles": indexed_documents,
                "graphNodes": workspace_graph_nodes,
                "graphEdges": workspace_graph_edges
            },
            "document": {
                "elapsedMs": model_capture.elapsed_ms,
                "responseBytes": model_capture.raw.len(),
                "graphNodes": graph_nodes,
                "graphEdges": graph_edges
            }
        },
        "visualization": {
            "elapsedMs": visualization_capture.elapsed_ms,
            "responseBytes": visualization_capture.raw.len(),
            "viewCandidates": view_candidates,
            "event": visualization_event.cloned().unwrap_or_else(|| serde_json::json!({}))
        },
        "counts": {
            "indexedDocuments": indexed_documents,
            "workspaceGraphNodes": workspace_graph_nodes,
            "workspaceGraphEdges": workspace_graph_edges,
            "graphNodes": graph_nodes,
            "graphEdges": graph_edges,
            "viewCandidates": view_candidates,
            "perfEvents": perf_events.len()
        },
        "budgets": {
            "mode": "report-only",
            "workspaceModelRequestMs": 5000,
            "documentModelRequestMs": 2500,
            "visualizationRequestMs": 1500
        },
        "bottlenecks": {
            "slowestPhases": slowest_phase_entries(&phases),
            "slowestFilesByParse": fixture_perf.slowest_files_by_parse.clone(),
            "largestFiles": fixture_perf.largest_files.clone()
        },
        "events": perf_events.clone()
    });
    emit_perf_report(&report, "large-workspace-performance.json");

    assert!(
        indexed_documents > 0 || workspace_graph_nodes > 0,
        "expected indexed workspace documents or workspace graph nodes"
    );
    assert!(graph_nodes > 0, "expected non-empty sysml/model graph");

    let _ = child.kill();
}

#[test]
fn lsp_workspace_visualization_returns_workspace_model_payload_for_workspace_root_uri() {
    let temp = tempfile::tempdir().expect("temp dir");
    let root: PathBuf = temp.path().canonicalize().expect("canonical root");

    let alpha_path = root.join("Alpha.sysml");
    let beta_path = root.join("Beta.sysml");
    std::fs::write(
        &alpha_path,
        "package Alpha { part def AlphaPart; part alphaInstance : AlphaPart; }",
    )
    .expect("write alpha");
    std::fs::write(
        &beta_path,
        "package Beta { part def BetaPart; part betaInstance : BetaPart; }",
    )
    .expect("write beta");

    let root_uri = url::Url::from_file_path(&root).expect("root uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");

    let initialized =
        serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} });
    send_message(&mut stdin, &initialized.to_string());
    std::thread::sleep(std::time::Duration::from_millis(800));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": root_uri.as_str() },
            "scope": ["graph", "workspaceVisualization"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");

    let workspace_files = model_json["result"]["workspaceModel"]["files"]
        .as_array()
        .expect("workspaceModel.files array");
    let workspace_semantic = model_json["result"]["workspaceModel"]["semantic"]
        .as_array()
        .expect("workspaceModel.semantic array");
    let summary = &model_json["result"]["workspaceModel"]["summary"];

    assert_eq!(
        summary["scannedFiles"].as_u64(),
        Some(2),
        "expected summary to report both workspace files"
    );
    assert_eq!(
        summary["loadedFiles"].as_u64(),
        Some(2),
        "expected summary to report both workspace files as loaded"
    );
    assert_eq!(
        workspace_files.len(),
        2,
        "expected one file entry per workspace file"
    );
    let file_uris: std::collections::HashSet<&str> = workspace_files
        .iter()
        .filter_map(|entry| entry["uri"].as_str())
        .collect();
    assert!(
        file_uris.iter().any(|uri| uri.contains("Alpha.sysml")),
        "workspace payload should include Alpha.sysml, got {file_uris:?}"
    );
    assert!(
        file_uris.iter().any(|uri| uri.contains("Beta.sysml")),
        "workspace payload should include Beta.sysml, got {file_uris:?}"
    );
    let semantic_names: std::collections::HashSet<&str> = workspace_semantic
        .iter()
        .filter_map(|entry| entry["name"].as_str())
        .collect();
    assert!(
        semantic_names.contains("Alpha"),
        "semantic workspace payload should include Alpha package"
    );
    assert!(
        semantic_names.contains("Beta"),
        "semantic workspace payload should include Beta package"
    );

    let _ = child.kill();
}

#[test]
#[ignore = "optional local drill-down; set SYSML_POWERSYSTEMS_DIR to an external grid fixture checkout"]
fn lsp_powersystems_parent_workspace_root_grid_connections_has_ibd_content() {
    let Some(repo_root) = std::env::var_os("SYSML_POWERSYSTEMS_DIR").map(PathBuf::from) else {
        eprintln!(
            "Skipping lsp_powersystems_parent_workspace_root_grid_connections_has_ibd_content: SYSML_POWERSYSTEMS_DIR is unset"
        );
        return;
    };
    if !repo_root.is_dir() {
        eprintln!(
            "Skipping lsp_powersystems_parent_workspace_root_grid_connections_has_ibd_content: {} is not a directory",
            repo_root.display()
        );
        return;
    }

    let response = lsp_server::views::build_sysml_visualization_for_paths(
        &repo_root,
        Some(&repo_root),
        &[],
        "interconnection-view",
        Some("gridConnections"),
    )
    .expect("build power systems visualization");

    assert_eq!(
        response.selected_view_name.as_deref(),
        Some("gridConnections")
    );
    let ibd = response.ibd.expect("ibd payload");
    assert!(
        ibd.parts.len() >= 10,
        "expected gridConnections to keep architecture parts, got {}: {:#?}",
        ibd.parts.len(),
        ibd
    );
    assert!(
        ibd.connectors.len() >= 15,
        "expected gridConnections to keep architecture connectors, got {}: {:#?}",
        ibd.connectors.len(),
        ibd
    );
}

#[test]
fn lsp_workspace_visualization_model_includes_all_sysml_examples_packages_when_configured() {
    let examples_root = std::env::var_os("SYSML_EXAMPLES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("C:/Git/sysml-examples"));
    if !examples_root.is_dir() {
        eprintln!(
            "Skipping lsp_workspace_visualization_model_includes_all_sysml_examples_packages_when_configured: {} is not a directory (set SYSML_EXAMPLES_DIR if needed)",
            examples_root.display()
        );
        return;
    }

    let expected_packages = [
        "SurveillanceDrone",
        "KitchenTimer",
        "TrafficLightIntersection",
    ];
    let drone_path = examples_root
        .join("drone")
        .join("sysml")
        .join("SurveillanceDrone.sysml");
    if !drone_path.is_file() {
        eprintln!(
            "Skipping lsp_workspace_visualization_model_includes_all_sysml_examples_packages_when_configured: expected fixture file missing {}",
            drone_path.display()
        );
        return;
    }

    let root_uri = url::Url::from_file_path(&examples_root).expect("examples root uri");
    let drone_uri = url::Url::from_file_path(&drone_path).expect("drone uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );
    std::thread::sleep(std::time::Duration::from_millis(1300));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": drone_uri.as_str() },
            "scope": ["graph", "workspaceVisualization"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");
    let graph_nodes = model_json["result"]["graph"]["nodes"]
        .as_array()
        .expect("graph nodes array");
    let node_names: std::collections::HashSet<&str> = graph_nodes
        .iter()
        .filter_map(|node| node["name"].as_str())
        .collect();

    let missing: Vec<&str> = expected_packages
        .iter()
        .copied()
        .filter(|pkg| !node_names.contains(pkg))
        .collect();
    assert!(
        missing.is_empty(),
        "workspace graph missing expected packages {:?}; available names sample: {:?}",
        missing,
        node_names.iter().take(40).copied().collect::<Vec<_>>()
    );

    let _ = child.kill();
}

#[test]
fn lsp_sysml_model_activity_diagrams_from_surveillance_drone_example_are_non_empty() {
    if std::env::var_os("SPEC42_RUN_SYSML_EXAMPLES_ACTIVITY_TESTS").is_none() {
        eprintln!(
            "Skipping lsp_sysml_model_activity_diagrams_from_surveillance_drone_example_are_non_empty: set SPEC42_RUN_SYSML_EXAMPLES_ACTIVITY_TESTS=1 to enable"
        );
        return;
    }
    let examples_root = std::env::var_os("SYSML_EXAMPLES_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("C:/Git/sysml-examples"));
    if !examples_root.is_dir() {
        eprintln!(
            "Skipping lsp_sysml_model_activity_diagrams_from_surveillance_drone_example_are_non_empty: {} is not a directory (set SYSML_EXAMPLES_DIR if needed)",
            examples_root.display()
        );
        return;
    }

    let drone_path = examples_root
        .join("drone")
        .join("sysml")
        .join("SurveillanceDrone.sysml");
    if !drone_path.is_file() {
        eprintln!(
            "Skipping lsp_sysml_model_activity_diagrams_from_surveillance_drone_example_are_non_empty: expected fixture file missing {}",
            drone_path.display()
        );
        return;
    }
    let drone_content = std::fs::read_to_string(&drone_path).expect("read SurveillanceDrone.sysml");
    if sysml_v2_parser::parse(&drone_content).is_err() {
        panic!(
            "sysml_v2_parser::parse failed for SurveillanceDrone.sysml; first errors: {:?}",
            util::parse_failure_diagnostics(&drone_content, 5)
        );
    }

    let root_uri = url::Url::from_file_path(&examples_root).expect("examples root uri");
    let drone_uri = url::Url::from_file_path(&drone_path).expect("drone uri");

    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": root_uri.as_str(),
            "capabilities": {},
            "clientInfo": { "name": "test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_message(&mut stdout).expect("init response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    // Allow workspace scan + initial indexing.
    std::thread::sleep(std::time::Duration::from_millis(1300));

    // Mirror the editor workflow: open the document so the server stores a parsed AST for sysml/model.
    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": drone_uri.as_str(),
                "languageId": "sysml",
                "version": 1,
                "text": drone_content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(50));

    let model_id = next_id();
    let model_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": model_id,
        "method": "sysml/model",
        "params": {
            "textDocument": { "uri": drone_uri.as_str() },
            "scope": ["activityDiagrams"]
        }
    });
    send_message(&mut stdin, &model_req.to_string());
    let model_resp = read_response(&mut stdout, model_id).expect("sysml/model response");
    let model_json: serde_json::Value =
        serde_json::from_str(&model_resp).expect("parse sysml/model response");

    let diagrams = model_json["result"]["activityDiagrams"]
        .as_array()
        .expect("activityDiagrams array");
    assert!(
        !diagrams.is_empty(),
        "expected activityDiagrams to be non-empty for SurveillanceDrone.sysml"
    );

    let diagram_names: std::collections::HashSet<&str> =
        diagrams.iter().filter_map(|d| d["name"].as_str()).collect();
    assert!(
        diagram_names.contains("ExecutePatrol") || diagram_names.contains("CaptureVideo"),
        "expected ExecutePatrol or CaptureVideo activity diagram; got names: {:?}",
        diagram_names
    );
    let execute_patrol = diagrams
        .iter()
        .find(|diagram| diagram["name"].as_str() == Some("ExecutePatrol"))
        .expect("ExecutePatrol activity diagram");
    assert!(
        execute_patrol["flows"]
            .as_array()
            .map(|flows| !flows.is_empty())
            .unwrap_or(false),
        "expected ExecutePatrol activity diagram to include explicit flows"
    );
    assert!(
        execute_patrol["actions"]
            .as_array()
            .map(|actions| !actions.is_empty())
            .unwrap_or(false),
        "expected ExecutePatrol activity diagram to include explicit action steps"
    );

    let _ = child.kill();
}
