use language_service::hover;

use super::support::{multi_doc, position_for, position_for_within, single_doc};

#[test]
fn hover_on_part_def_keyword() {
    let content = "package P { part def X; }";
    let ws = single_doc("test.sysml", content);
    let pos = position_for(content, "part");
    let result = hover(&ws, "test.sysml", pos).expect("hover");
    assert!(
        result.contents.contains("part"),
        "expected part keyword docs, got: {}",
        result.contents
    );
}

#[test]
fn hover_on_part_definition_symbol() {
    let content = "package P { part def X; }";
    let ws = single_doc("test.sysml", content);
    let pos = position_for_within(content, "part def X", "X");
    let result = hover(&ws, "test.sysml", pos).expect("hover");
    assert!(
        result.contents.to_lowercase().contains("part"),
        "hover should mention part definition: {}",
        result.contents
    );
}

#[test]
fn hover_resolves_typed_usage_and_nested_symbols() {
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
    let ws = single_doc("hover-rich.sysml", content);
    let usage_pos = position_for(content, "frame :");
    let usage_hover = hover(&ws, "hover-rich.sysml", usage_pos)
        .expect("hover on usage")
        .contents;
    assert!(
        usage_hover.contains("part") && usage_hover.contains("frame"),
        "hover on part usage should describe the usage node: {usage_hover}"
    );
    assert!(
        usage_hover.contains("Airframe") || usage_hover.contains("Resolves to"),
        "hover on part usage should mention the resolved type: {usage_hover}"
    );

    let type_pos = position_for(content, "PropulsionUnit;");
    let type_hover = hover(&ws, "hover-rich.sysml", type_pos)
        .expect("hover on type")
        .contents;
    assert!(
        type_hover.contains("PropulsionUnit") && type_hover.contains("part def"),
        "hover on type reference should resolve to the type definition: {type_hover}"
    );
}

#[test]
fn hover_uses_exact_symbol_under_cursor_within_typed_usage() {
    let content = r#"package DroneLibrary {
    package DroneParts {
        part def Airframe;
    }

    part def SurveillanceQuadrotorDrone {
        part frame : DroneParts::Airframe;
    }
}"#;
    let ws = single_doc("hover-exact-symbol.sysml", content);
    let pos = position_for_within(content, "DroneParts::Airframe", "Airframe");
    let result = hover(&ws, "hover-exact-symbol.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("Airframe") && result.contains("part def"),
        "hover on qualified type reference should resolve to Airframe definition: {result}"
    );
}

#[test]
fn hover_resolves_requirement_subject_in_context() {
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
    let ws = single_doc("hover-requirement-subject.sysml", content);
    let pos = position_for_within(content, "subject communication;", "communication");
    let result = hover(&ws, "hover-requirement-subject.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("part") && result.contains("communication"),
        "hover should resolve to the in-context subject part usage: {result}"
    );
    assert!(
        result.contains("part communication : Communication;")
            && result.contains("*Container:* `DronePackage::Drone`"),
        "hover should show local communication part with container context: {result}"
    );
    assert!(
        !result.contains("2 definitions"),
        "hover should not show ambiguous symbol-list fallback: {result}"
    );
}

#[test]
fn hover_returns_subject_declaration_for_requirement_subject_name() {
    let content = r#"package DronePackage {
    part def SurveillanceQuadrotorDrone;

    requirement def MaxAltitudeAGLReq {
        subject drone : SurveillanceQuadrotorDrone;
    }
}"#;
    let ws = single_doc("hover-requirement-subject-name.sysml", content);
    let pos = position_for_within(
        content,
        "subject drone : SurveillanceQuadrotorDrone;",
        "drone",
    );
    let result = hover(&ws, "hover-requirement-subject-name.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("subject drone : SurveillanceQuadrotorDrone;"),
        "hover should describe the subject declaration itself: {result}"
    );
    assert!(
        result.contains("*Container:* `DronePackage::MaxAltitudeAGLReq`"),
        "hover should include qualified parent context: {result}"
    );
}

#[test]
fn hover_resolves_port_and_attribute_type_references() {
    let content = r#"package P {
    port def CommandPort;
    attribute def Voltage;

    part def Controller {
        port cmd : CommandPort;
        attribute def BatteryVoltage : Voltage;
    }
}"#;
    let ws = single_doc("hover-port-attribute.sysml", content);
    let port_pos = position_for(content, "CommandPort;");
    let port_hover = hover(&ws, "hover-port-attribute.sysml", port_pos)
        .expect("hover port")
        .contents;
    assert!(
        port_hover.contains("CommandPort") && port_hover.contains("port def"),
        "hover on port type reference should resolve to port definition: {port_hover}"
    );

    let attribute_pos = position_for_within(content, "BatteryVoltage : Voltage;", "Voltage");
    let attribute_hover = hover(&ws, "hover-port-attribute.sysml", attribute_pos)
        .expect("hover attribute")
        .contents;
    assert!(
        attribute_hover.contains("Voltage"),
        "hover on attribute type reference should resolve to type definition: {attribute_hover}"
    );
}

#[test]
fn hover_resolves_public_reexported_type_reference() {
    let content_core = "package Core { attribute def Name; }";
    let content_domain = "package Domain { public import Core::*; }";
    let content_use =
        "package Demo { import Domain::*; part def Consumer { attribute label : Name; } }";
    let ws = multi_doc(&[
        ("core.sysml", content_core),
        ("domain.sysml", content_domain),
        ("use.sysml", content_use),
    ]);
    let pos = position_for(content_use, "Name");
    let result = hover(&ws, "use.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("Name") && result.contains("attribute def"),
        "hover on public re-exported type should resolve to definition: {result}"
    );
}

#[test]
fn hover_includes_semantic_context_fields() {
    let content = r#"package Demo {
    part def Engine;
    part vehicle {
        part engine : Engine;
    }
}"#;
    let ws = single_doc("hover-context-fields.sysml", content);
    let pos = position_for_within(content, "part engine : Engine;", "engine");
    let result = hover(&ws, "hover-context-fields.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("Qualified name")
            && result.contains("Demo::vehicle::engine")
            && result.contains("Declared type")
            && result.contains("Engine")
            && result.contains("Container"),
        "expected richer semantic hover fields: {result}"
    );
}

#[test]
fn hover_returns_unresolved_reference_fallback() {
    let content = r#"package Demo {
    part vehicle : MissingType;
}"#;
    let ws = single_doc("hover-unresolved.sysml", content);
    let pos = position_for_within(content, "MissingType", "MissingType");
    let result = hover(&ws, "hover-unresolved.sysml", pos)
        .expect("hover")
        .contents;
    assert!(
        result.contains("Unresolved reference") && result.contains("MissingType"),
        "expected unresolved hover fallback: {result}"
    );
}
