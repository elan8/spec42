use std::path::Path;

use super::harness::{next_id, read_response, send_message, spawn_server, spawn_server_with_env};

const SYSTEMS_RESOURCE: &str = "https://www.omg.org/spec/SysML/20250201/Systems-Library.kpar";

fn write_manifest(root: &Path, usage: serde_json::Value) {
    std::fs::write(
        root.join(".project.json"),
        serde_json::json!({
            "name": root.file_name().unwrap().to_string_lossy(),
            "version": "1.0.0",
            "usage": usage,
        })
        .to_string(),
    )
    .unwrap();
}

fn initialize(
    child: &mut std::process::Child,
    root: &Path,
) -> (std::process::ChildStdin, std::process::ChildStdout) {
    let mut stdin = child.stdin.take().unwrap();
    let mut stdout = child.stdout.take().unwrap();
    let id = next_id();
    send_message(
        &mut stdin,
        &serde_json::json!({
            "jsonrpc":"2.0", "id":id, "method":"initialize",
            "params": {
                "processId":null,
                "rootUri":url::Url::from_directory_path(root).unwrap(),
                "capabilities":{},
                "clientInfo":{"name":"project-workspaces-test","version":"1"}
            }
        })
        .to_string(),
    );
    read_response(&mut stdout, id).expect("initialize response");
    send_message(
        &mut stdin,
        &serde_json::json!({"jsonrpc":"2.0","method":"initialized","params":{}}).to_string(),
    );
    (stdin, stdout)
}

fn open(stdin: &mut std::process::ChildStdin, uri: &url::Url, text: &str) {
    send_message(
        stdin,
        &serde_json::json!({
            "jsonrpc":"2.0", "method":"textDocument/didOpen",
            "params":{"textDocument":{"uri":uri,"languageId":"sysml","version":1,"text":text}}
        })
        .to_string(),
    );
}

fn definition(
    stdin: &mut std::process::ChildStdin,
    stdout: &mut std::process::ChildStdout,
    uri: &url::Url,
    character: u32,
) -> serde_json::Value {
    let id = next_id();
    send_message(
        stdin,
        &serde_json::json!({
            "jsonrpc":"2.0", "id":id, "method":"textDocument/definition",
            "params":{"textDocument":{"uri":uri},"position":{"line":0,"character":character}}
        })
        .to_string(),
    );
    serde_json::from_str(&read_response(stdout, id).unwrap()).unwrap()
}

fn position_of(text: &str, needle: &str) -> u32 {
    (text.find(needle).unwrap() + 1) as u32
}

fn fake_stdlib(root: &Path) {
    for name in [
        "Kernel_Semantic_Library-1.0.0",
        "Kernel_Data_Type_Library-1.0.0",
        "Kernel_Function_Library-1.0.0",
        "SysML_Systems_Library-2.0.0",
        "SysML_Quantities_and_Units_Library-2.0.0",
        "SysML_Analysis_Library-2.0.0",
        "SysML_Cause_and_Effect_Library-2.0.0",
        "SysML_Geometry_Library-2.0.0",
        "SysML_Metadata_Library-2.0.0",
        "SysML_Requirement_Derivation_Library-2.0.0",
    ] {
        std::fs::create_dir_all(root.join(name)).unwrap();
    }
    std::fs::write(
        root.join("SysML_Systems_Library-2.0.0/Systems.sysml"),
        "package Systems { part def SystemThing; }",
    )
    .unwrap();
    std::fs::write(
        root.join("SysML_Analysis_Library-2.0.0/Analysis.sysml"),
        "package Analysis { part def AnalysisThing; }",
    )
    .unwrap();
}

#[test]
fn sibling_manifest_projects_are_navigation_isolated() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().canonicalize().unwrap();
    let a = repo.join("a");
    let b = repo.join("b");
    std::fs::create_dir_all(&a).unwrap();
    std::fs::create_dir_all(&b).unwrap();
    write_manifest(&a, serde_json::json!([]));
    write_manifest(&b, serde_json::json!([]));
    std::fs::write(a.join("defs.sysml"), "package Same { part def Shared; }").unwrap();
    std::fs::write(
        b.join("defs.sysml"),
        "package Same { part def Shared; part def BOnly; }",
    )
    .unwrap();
    let use_text = "package Use { private import Same::*; part own : Shared; part leak : BOnly; }";
    let use_path = a.join("use.sysml");
    std::fs::write(&use_path, use_text).unwrap();
    let use_uri = url::Url::from_file_path(use_path).unwrap();

    let mut child = spawn_server();
    let (mut stdin, mut stdout) = initialize(&mut child, &repo);
    std::thread::sleep(std::time::Duration::from_millis(500));
    open(&mut stdin, &use_uri, use_text);
    std::thread::sleep(std::time::Duration::from_millis(300));

    let own = definition(
        &mut stdin,
        &mut stdout,
        &use_uri,
        position_of(use_text, "Shared"),
    );
    assert!(own["result"]["uri"]
        .as_str()
        .unwrap()
        .contains("/a/defs.sysml"));
    let leak = definition(
        &mut stdin,
        &mut stdout,
        &use_uri,
        position_of(use_text, "BOnly"),
    );
    assert!(leak["result"].is_null(), "sibling symbol leaked: {leak}");
    let _ = child.kill();
}

#[test]
fn manifest_usage_adds_to_mandatory_libraries_and_manifestless_keeps_defaults() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    let stdlib = temp.path().join("stdlib");
    std::fs::create_dir_all(&repo).unwrap();
    fake_stdlib(&stdlib);
    let declared = repo.join("declared");
    let loose = repo.join("loose");
    std::fs::create_dir_all(&declared).unwrap();
    std::fs::create_dir_all(&loose).unwrap();
    write_manifest(
        &declared,
        serde_json::json!([{"resource":SYSTEMS_RESOURCE,"versionConstraint":"2.0.0"}]),
    );
    let declared_text = "package P { private import Systems::*; private import Analysis::*; part s : SystemThing; part a : AnalysisThing; }";
    let declared_path = declared.join("model.sysml");
    std::fs::write(&declared_path, declared_text).unwrap();
    let loose_text = "package Q { private import Analysis::*; part a : AnalysisThing; }";
    let loose_path = loose.join("model.sysml");
    std::fs::write(&loose_path, loose_text).unwrap();
    let declared_uri = url::Url::from_file_path(declared_path).unwrap();
    let loose_uri = url::Url::from_file_path(loose_path).unwrap();

    let mut child = spawn_server_with_env(&[("SPEC42_LSP_TEST_STDLIB", &stdlib)]);
    let (mut stdin, mut stdout) = initialize(&mut child, &repo);
    std::thread::sleep(std::time::Duration::from_millis(800));
    open(&mut stdin, &declared_uri, declared_text);
    open(&mut stdin, &loose_uri, loose_text);
    std::thread::sleep(std::time::Duration::from_millis(300));

    let systems = definition(
        &mut stdin,
        &mut stdout,
        &declared_uri,
        position_of(declared_text, "SystemThing"),
    );
    assert!(systems["result"]["uri"]
        .as_str()
        .unwrap()
        .contains("Systems.sysml"));
    let mandatory = definition(
        &mut stdin,
        &mut stdout,
        &declared_uri,
        position_of(declared_text, "AnalysisThing"),
    );
    assert!(mandatory["result"]["uri"]
        .as_str()
        .unwrap()
        .contains("Analysis.sysml"));
    let default_analysis = definition(
        &mut stdin,
        &mut stdout,
        &loose_uri,
        position_of(loose_text, "AnalysisThing"),
    );
    assert!(default_analysis["result"]["uri"]
        .as_str()
        .unwrap()
        .contains("Analysis.sysml"));

    let _ = child.kill();
}

#[test]
fn empty_manifest_usage_retains_mandatory_standard_libraries() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    let stdlib = temp.path().join("stdlib");
    let project = repo.join("model");
    std::fs::create_dir_all(&project).unwrap();
    fake_stdlib(&stdlib);
    write_manifest(&project, serde_json::json!([]));
    let text = "package P { private import Analysis::*; part a : AnalysisThing; }";
    let path = project.join("model.sysml");
    std::fs::write(&path, text).unwrap();
    let uri = url::Url::from_file_path(path).unwrap();

    let mut child = spawn_server_with_env(&[("SPEC42_LSP_TEST_STDLIB", &stdlib)]);
    let (mut stdin, mut stdout) = initialize(&mut child, &repo);
    std::thread::sleep(std::time::Duration::from_millis(500));
    open(&mut stdin, &uri, text);
    std::thread::sleep(std::time::Duration::from_millis(300));
    let resolved = definition(
        &mut stdin,
        &mut stdout,
        &uri,
        position_of(text, "AnalysisThing"),
    );
    assert!(resolved["result"]["uri"]
        .as_str()
        .unwrap()
        .contains("Analysis.sysml"));
    let _ = child.kill();
}

#[test]
fn dependency_mismatch_is_an_explicit_lsp_error_without_fallback() {
    let temp = tempfile::tempdir().unwrap();
    let repo = temp.path().join("repo");
    let stdlib = temp.path().join("stdlib");
    let project = repo.join("bad");
    std::fs::create_dir_all(&project).unwrap();
    fake_stdlib(&stdlib);
    write_manifest(
        &project,
        serde_json::json!([{"resource":SYSTEMS_RESOURCE,"versionConstraint":"9.0.0"}]),
    );
    let text = "package P { private import Systems::*; part s : SystemThing; }";
    let path = project.join("model.sysml");
    std::fs::write(&path, text).unwrap();
    let uri = url::Url::from_file_path(path).unwrap();

    let mut child = spawn_server_with_env(&[("SPEC42_LSP_TEST_STDLIB", &stdlib)]);
    let (mut stdin, mut stdout) = initialize(&mut child, &repo);
    std::thread::sleep(std::time::Duration::from_millis(300));
    open(&mut stdin, &uri, text);
    let response = definition(
        &mut stdin,
        &mut stdout,
        &uri,
        position_of(text, "SystemThing"),
    );
    let message = response["error"]["message"].as_str().unwrap_or_default();
    assert!(
        message.contains("not satisfied"),
        "unexpected response: {response}"
    );
    assert!(!response.to_string().contains("Systems.sysml"));
    let _ = child.kill();
}
