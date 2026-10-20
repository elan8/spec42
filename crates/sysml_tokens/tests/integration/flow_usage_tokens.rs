//! `flow`/`message` usages paint their declared name and payload type in every owning body,
//! and leave their keywords alone.
use sysml_tokens::{ast_semantic_ranges, legend_token_types, semantic_tokens_full};

/// `(text, token type)` for every token on `line` (0-based).
fn line_tokens(content: &str, line: u32) -> Vec<(String, &'static str)> {
    let parsed = sysml_query::syntax::SyntaxService::new().parse_text(content);
    let ranges = ast_semantic_ranges(&parsed, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let lines: Vec<&str> = content.lines().collect();
    let (mut current, mut start) = (0u32, 0u32);
    let mut out = Vec::new();
    for token in tokens.data.chunks(5) {
        current += token[0];
        start = if token[0] == 0 {
            start + token[1]
        } else {
            token[1]
        };
        if current == line {
            let text = lines[current as usize]
                .chars()
                .skip(start as usize)
                .take(token[2] as usize)
                .collect();
            out.push((text, legend_token_types()[token[3] as usize]));
        }
    }
    out
}

fn kind<'a>(tokens: &'a [(String, &'static str)], text: &str) -> &'a str {
    tokens
        .iter()
        .find(|(token, _)| token == text)
        .unwrap_or_else(|| panic!("no token {text:?} in {tokens:?}"))
        .1
}

fn assert_message_line(content: &str, line: u32) {
    let tokens = line_tokens(content, line);
    assert_eq!(kind(&tokens, "message"), "keyword", "{tokens:?}");
    assert_eq!(kind(&tokens, "of"), "keyword", "{tokens:?}");
    assert_eq!(
        kind(&tokens, "transmitDeliveryCode"),
        "property",
        "{tokens:?}"
    );
    assert_eq!(kind(&tokens, "DeliveryCode"), "type", "{tokens:?}");
}

const MESSAGE: &str = "message transmitDeliveryCode of DeliveryCode
            from owner.deliveryCodeSent
            to drone.deliveryCodeReceived;";

#[test]
fn message_in_part_usage_body_paints_name_and_payload_type() {
    let content = format!("package P {{\n    item def DeliveryCode;\n    part system {{\n        {MESSAGE}\n    }}\n}}\n");
    assert_message_line(&content, 3);
}

#[test]
fn message_in_part_definition_body_keeps_keywords() {
    let content = format!("package P {{\n    item def DeliveryCode;\n    part def System {{\n        {MESSAGE}\n    }}\n}}\n");
    assert_message_line(&content, 3);
    let endpoints = line_tokens(&content, 4);
    assert_eq!(kind(&endpoints, "from"), "keyword", "{endpoints:?}");
}

#[test]
fn message_in_package_body_paints_name_and_payload_type() {
    let content = format!("package P {{\n    item def DeliveryCode;\n    {MESSAGE}\n}}\n");
    assert_message_line(&content, 2);
}

#[test]
fn flow_in_action_body_paints_name_and_payload_type() {
    let content = "package P {
    item def Fuel;
    action def Supply {
        action pump { out fuel : Fuel; }
        action engine { in fuel : Fuel; }
        flow fuelFlow of Fuel from pump.fuel to engine.fuel;
    }
}
";
    let tokens = line_tokens(content, 5);
    assert_eq!(kind(&tokens, "flow"), "keyword", "{tokens:?}");
    assert_eq!(kind(&tokens, "fuelFlow"), "property", "{tokens:?}");
    assert_eq!(kind(&tokens, "Fuel"), "type", "{tokens:?}");
}

#[test]
fn endpoint_only_flow_keeps_its_keyword() {
    let content = "package P {
    part def System {
        flow source.out to sink.in;
    }
}
";
    let tokens = line_tokens(content, 2);
    assert_eq!(kind(&tokens, "flow"), "keyword", "{tokens:?}");
    assert_eq!(kind(&tokens, "to"), "keyword", "{tokens:?}");
}
