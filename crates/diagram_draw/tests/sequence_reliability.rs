//! Presentation regressions over the canonical source-to-product corpus.
use diagram_draw::{draw_input_from_payload, render_svg_from_payload};
use serde_json::Value;

fn product(name: &str) -> Value {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/snapshots/generation")
        .join(format!("{name}.md"));
    let text = std::fs::read_to_string(path).unwrap().replace("\r\n", "\n");
    let json = text
        .split_once("## diagram.json\n~~~json\n")
        .unwrap()
        .1
        .split_once("\n~~~")
        .unwrap()
        .0;
    serde_json::from_str(json).unwrap()
}

#[test]
fn incompatible_products_do_not_fall_through_to_legacy_empty_drawings() {
    for version in [
        serde_json::json!(diagram_product::SCHEMA_VERSION - 1),
        serde_json::json!(diagram_product::SCHEMA_VERSION + 1),
        Value::Null,
    ] {
        let mut payload = product("sequence_shared_part_endpoints");
        payload["schemaVersion"] = version;
        let error = render_svg_from_payload(&payload, 1000.0, 700.0).unwrap_err();
        assert!(
            matches!(error, diagram_draw::pipeline::PipelineError::InvalidPayload(message)
            if message.contains("unsupported diagram product schema version"))
        );
    }
}

#[test]
fn supported_shared_anonymous_reply_and_self_messages_draw() {
    for (name, count) in [
        ("sequence_shared_part_endpoints", 2),
        ("sequence_shared_events_fan_in_out", 4),
        ("sequence_anonymous_messages", 3),
        ("sequence_reply_and_self", 3),
    ] {
        let svg = render_svg_from_payload(&product(name), 1000.0, 700.0).unwrap();
        assert_eq!(
            svg.matches("class=\"sequence-message\"").count()
                + svg.matches("class=\"sequence-message ").count(),
            count,
            "{name}: {svg}"
        );
        assert!(!svg.contains("No sequence"));
    }
}

#[test]
fn anonymous_labels_stay_empty_and_payload_labels_remain_visible() {
    let product = product("sequence_anonymous_messages");
    let input = draw_input_from_payload(&product).unwrap();
    let messages = &input["meta"]["sequenceDiagram"]["messages"];
    assert_eq!(messages[0]["name"], "");
    assert_eq!(messages[1]["name"], "");
    assert_eq!(messages[1]["payloadName"], "Funny");
    assert_eq!(messages[2]["name"], "Named");
    let svg = render_svg_from_payload(&product, 1000.0, 700.0).unwrap();
    assert!(svg.contains("Named : Funny"));
}

#[test]
fn partial_sequences_preserve_useful_messages_and_show_the_limitation() {
    for (name, code) in [
        ("sequence_event_cycle", "sequence-ordering-cycle"),
        ("sequence_receive_event_cycle", "sequence-ordering-cycle"),
        (
            "sequence_message_cycle_and_blocked",
            "sequence-ordering-cycle",
        ),
        ("sequence_omitted_nested", "sequence-content-omitted"),
        ("sequence_omitted_structured", "sequence-content-omitted"),
        ("sequence_recovery", "parse-recovery"),
    ] {
        let svg = render_svg_from_payload(&product(name), 1000.0, 700.0).unwrap();
        assert_eq!(
            svg.matches("class=\"sequence-message\"").count()
                + svg.matches("class=\"sequence-message ").count(),
            1,
            "{name}: {svg}"
        );
        assert!(svg.contains("Incomplete projection"));
        assert!(svg.contains(code));
        assert!(!svg.contains("No sequence"));
    }
}
