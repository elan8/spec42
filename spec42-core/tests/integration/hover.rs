//! Hover integration tests.

use super::harness::{next_id, read_message, read_response, send_message, spawn_server};

fn position_for(content: &str, needle: &str) -> (usize, usize) {
    for (line_index, line) in content.lines().enumerate() {
        if let Some(character) = line.find(needle) {
            return (line_index, character);
        }
    }
    panic!("needle not found in content: {needle}");
}

fn position_for_within(content: &str, needle: &str, inner: &str) -> (usize, usize) {
    let (line, character) = position_for(content, needle);
    let inner_offset = needle
        .find(inner)
        .unwrap_or_else(|| panic!("inner needle not found: {inner}"));
    (line, character + inner_offset)
}

#[test]
fn lsp_initialize_and_hover() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///test.sysml";
    let content = "package P { part def X; }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": null,
            "capabilities": {},
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let init_resp = read_message(&mut stdout).expect("initialize response");
    let init_json: serde_json::Value =
        serde_json::from_str(&init_resp).expect("parse init response");
    assert_eq!(init_json["id"], init_id);
    assert!(init_json["result"]["capabilities"].is_object());

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());

    std::thread::sleep(std::time::Duration::from_millis(50));

    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": 0, "character": 14 }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    assert_eq!(hover_json["id"], hover_id);
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str());
    assert!(
        contents.is_some(),
        "hover should return contents: {}",
        hover_resp
    );
    let contents = contents.unwrap();
    assert!(
        contents.to_lowercase().contains("part"),
        "hover should mention 'part': {}",
        contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_resolves_typed_usage_and_nested_symbols() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///hover-rich.sysml";
    let content = r#"package DroneLibrary {
    package DroneParts {
        part def Airframe;
        part def PropulsionUnit;
    }

    part def SurveillanceQuadrotorDrone {
        part frame : DroneParts::Airframe;
        part propulsion[4] : DroneParts::PropulsionUnit;
    }
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
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(75));

    let (usage_line, usage_char) = position_for(content, "frame :");
    let (type_line, type_char) = position_for(content, "PropulsionUnit;");

    let hover_on_usage_id = next_id();
    let hover_on_usage_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_on_usage_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": usage_line, "character": usage_char }
        }
    });
    send_message(&mut stdin, &hover_on_usage_req.to_string());
    let hover_on_usage_resp =
        read_response(&mut stdout, hover_on_usage_id).expect("hover usage response");
    let hover_on_usage_json: serde_json::Value =
        serde_json::from_str(&hover_on_usage_resp).expect("parse hover usage response");
    let usage_contents = hover_on_usage_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_on_usage_json["result"]["contents"].as_str())
        .expect("hover on usage should have contents");
    assert!(
        usage_contents.contains("part") && usage_contents.contains("frame"),
        "hover on part usage should describe the usage node: {}",
        usage_contents
    );
    assert!(
        usage_contents.contains("Airframe") || usage_contents.contains("Resolves to"),
        "hover on part usage should mention the resolved type: {}",
        usage_contents
    );

    let hover_on_type_id = next_id();
    let hover_on_type_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_on_type_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": type_line, "character": type_char }
        }
    });
    send_message(&mut stdin, &hover_on_type_req.to_string());
    let hover_on_type_resp =
        read_response(&mut stdout, hover_on_type_id).expect("hover type response");
    let hover_on_type_json: serde_json::Value =
        serde_json::from_str(&hover_on_type_resp).expect("parse hover type response");
    let type_contents = hover_on_type_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_on_type_json["result"]["contents"].as_str())
        .expect("hover on type should have contents");
    assert!(
        type_contents.contains("PropulsionUnit") && type_contents.contains("part def"),
        "hover on type reference should resolve to the type definition: {}",
        type_contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_uses_exact_symbol_under_cursor_within_typed_usage() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///hover-exact-symbol.sysml";
    let content = r#"package DroneLibrary {
    package DroneParts {
        part def Airframe;
    }

    part def SurveillanceQuadrotorDrone {
        part frame : DroneParts::Airframe;
    }
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
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(75));

    let (usage_line, usage_char) = position_for_within(content, "part frame : DroneParts::Airframe;", "frame");
    let hover_usage_id = next_id();
    let hover_usage_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_usage_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": usage_line, "character": usage_char }
        }
    });
    send_message(&mut stdin, &hover_usage_req.to_string());
    let hover_usage_resp = read_response(&mut stdout, hover_usage_id).expect("hover usage response");
    let hover_usage_json: serde_json::Value =
        serde_json::from_str(&hover_usage_resp).expect("parse hover usage response");
    let usage_contents = hover_usage_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_usage_json["result"]["contents"].as_str())
        .expect("hover on usage symbol should have contents");
    assert!(
        usage_contents.contains("part") && usage_contents.contains("frame"),
        "hover on usage name should describe the usage symbol: {}",
        usage_contents
    );
    assert!(
        usage_contents.contains("DroneParts::Airframe") || usage_contents.contains("Resolves to"),
        "hover on usage name should retain typing context: {}",
        usage_contents
    );

    let (type_line, type_char) =
        position_for_within(content, "part frame : DroneParts::Airframe;", "Airframe");
    let hover_type_id = next_id();
    let hover_type_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_type_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": type_line, "character": type_char }
        }
    });
    send_message(&mut stdin, &hover_type_req.to_string());
    let hover_type_resp = read_response(&mut stdout, hover_type_id).expect("hover type response");
    let hover_type_json: serde_json::Value =
        serde_json::from_str(&hover_type_resp).expect("parse hover type response");
    let type_contents = hover_type_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_type_json["result"]["contents"].as_str())
        .expect("hover on referenced type symbol should have contents");
    assert!(
        type_contents.contains("part def") && type_contents.contains("Airframe"),
        "hover on type token should describe the referenced definition: {}",
        type_contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_resolves_requirement_subject_in_context_instead_of_showing_ambiguous_defs() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///hover-requirement-subject.sysml";
    let content = r#"package DronePackage {
    part def Communication;

    part def Drone {
        part communication : Communication;

        requirement def VideoLatencyReq {
            subject communication;
        }
    }

    part def Container {
        part droneInstance : Drone {
            part communication : Communication;
        }
    }
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
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(75));

    let (line, character) = position_for_within(content, "subject communication;", "communication");
    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str())
        .expect("hover should return contents");
    assert!(
        contents.contains("part") && contents.contains("communication"),
        "hover should resolve to the in-context subject part usage: {}",
        contents
    );
    assert!(
        contents.contains("part communication : Communication;") && contents.contains("*In:* `Drone`"),
        "hover should still point at the local communication part after simplifying the hover: {}",
        contents
    );
    assert!(
        !contents.contains("2 definitions"),
        "hover should not show the ambiguous symbol-list fallback here: {}",
        contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_returns_subject_declaration_hover_for_requirement_subject_name() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///hover-requirement-subject-name.sysml";
    let content = r#"package DronePackage {
    part def SurveillanceQuadrotorDrone;

    requirement def MaxAltitudeAGLReq {
        subject drone : SurveillanceQuadrotorDrone;
    }
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
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(75));

    let (line, character) =
        position_for_within(content, "subject drone : SurveillanceQuadrotorDrone;", "drone");
    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": line, "character": character }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str())
        .expect("hover should return contents");
    assert!(
        contents.contains("subject drone : SurveillanceQuadrotorDrone;"),
        "hover should describe the subject declaration itself: {}",
        contents
    );
    assert!(
        contents.contains("*In:* `MaxAltitudeAGLReq`"),
        "hover should keep only concise parent context: {}",
        contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_resolves_port_and_attribute_type_references() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri = "file:///hover-port-attribute.sysml";
    let content = r#"package P {
    port def CommandPort;
    attribute def Voltage;

    part def Controller {
        port cmd : CommandPort;
        attribute def BatteryVoltage : Voltage;
    }
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
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");

    let initialized = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "initialized",
        "params": {}
    });
    send_message(&mut stdin, &initialized.to_string());

    let did_open = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {
            "textDocument": {
                "uri": uri,
                "languageId": "sysml",
                "version": 1,
                "text": content
            }
        }
    });
    send_message(&mut stdin, &did_open.to_string());
    std::thread::sleep(std::time::Duration::from_millis(75));

    let (port_line, port_char) = position_for(content, "CommandPort;");
    let hover_port_id = next_id();
    let hover_port_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_port_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": port_line, "character": port_char }
        }
    });
    send_message(&mut stdin, &hover_port_req.to_string());
    let hover_port_resp = read_response(&mut stdout, hover_port_id).expect("hover port response");
    let hover_port_json: serde_json::Value =
        serde_json::from_str(&hover_port_resp).expect("parse hover port response");
    let port_contents = hover_port_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_port_json["result"]["contents"].as_str())
        .expect("hover on port type should have contents");
    assert!(
        port_contents.contains("CommandPort") && port_contents.contains("port def"),
        "hover on port type reference should resolve to the port definition: {}",
        port_contents
    );

    let (attribute_line, attribute_char) =
        position_for_within(content, "BatteryVoltage : Voltage;", "Voltage");
    let hover_attribute_id = next_id();
    let hover_attribute_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_attribute_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri },
            "position": { "line": attribute_line, "character": attribute_char }
        }
    });
    send_message(&mut stdin, &hover_attribute_req.to_string());
    let hover_attribute_resp =
        read_response(&mut stdout, hover_attribute_id).expect("hover attribute response");
    let hover_attribute_json: serde_json::Value =
        serde_json::from_str(&hover_attribute_resp).expect("parse hover attribute response");
    let attribute_contents = hover_attribute_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_attribute_json["result"]["contents"].as_str())
        .expect("hover on attribute type should have contents");
    assert!(
        attribute_contents.contains("Voltage"),
        "hover on attribute type reference should resolve to the type definition: {}",
        attribute_contents
    );

    let _ = child.kill();
}

#[test]
fn lsp_hover_resolves_public_reexported_type_reference() {
    let mut child = spawn_server();
    let mut stdin = child.stdin.take().expect("stdin");
    let mut stdout = child.stdout.take().expect("stdout");

    let uri_core = "file:///workspace/core.sysml";
    let uri_domain = "file:///workspace/domain.sysml";
    let uri_use = "file:///workspace/use.sysml";
    let content_core = "package Core { attribute def Name; }";
    let content_domain = "package Domain { public import Core::*; }";
    let content_use =
        "package Demo { import Domain::*; part def Consumer { attribute groupName : Name; } }";

    let init_id = next_id();
    let init_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": init_id,
        "method": "initialize",
        "params": {
            "processId": null,
            "rootUri": "file:///workspace",
            "capabilities": {},
            "clientInfo": { "name": "lsp_integration_test", "version": "0.1.0" }
        }
    });
    send_message(&mut stdin, &init_req.to_string());
    let _ = read_response(&mut stdout, init_id).expect("initialize response");
    send_message(
        &mut stdin,
        &serde_json::json!({ "jsonrpc": "2.0", "method": "initialized", "params": {} }).to_string(),
    );

    for (uri, text) in [
        (uri_core, content_core),
        (uri_domain, content_domain),
        (uri_use, content_use),
    ] {
        send_message(
            &mut stdin,
            &serde_json::json!({
                "jsonrpc": "2.0",
                "method": "textDocument/didOpen",
                "params": {
                    "textDocument": { "uri": uri, "languageId": "sysml", "version": 1, "text": text }
                }
            })
            .to_string(),
        );
    }
    std::thread::sleep(std::time::Duration::from_millis(100));

    let hover_id = next_id();
    let hover_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": hover_id,
        "method": "textDocument/hover",
        "params": {
            "textDocument": { "uri": uri_use },
            "position": { "line": 0, "character": 75 }
        }
    });
    send_message(&mut stdin, &hover_req.to_string());
    let hover_resp = read_response(&mut stdout, hover_id).expect("hover response");
    let hover_json: serde_json::Value =
        serde_json::from_str(&hover_resp).expect("parse hover response");
    let contents = hover_json["result"]["contents"]["value"]
        .as_str()
        .or_else(|| hover_json["result"]["contents"].as_str())
        .expect("hover should return contents");
    assert!(
        contents.contains("Name") && contents.contains("attribute def"),
        "hover on public re-exported type should resolve to the definition: {}",
        contents
    );

    let _ = child.kill();
}
