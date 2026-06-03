use sysml_semantic_tokens::{ast_semantic_ranges, semantic_tokens_full};
use sysml_v2_parser::parse_for_editor;

fn decode_semantic_tokens(data: &[u32]) -> Vec<(u32, u32, u32, u32)> {
    let mut line: u32 = 0;
    let mut start_char: u32 = 0;
    let mut tokens = Vec::new();
    let mut i = 0;
    while i + 5 <= data.len() {
        line += data[i];
        start_char = if data[i] == 0 {
            start_char + data[i + 1]
        } else {
            data[i + 1]
        };
        let length = data[i + 2];
        let token_type = data[i + 3];
        tokens.push((line, start_char, length, token_type));
        i += 5;
    }
    tokens
}

#[test]
fn port_definitions_tokenize_parameter_names_and_types() {
    let content = r#"port def GimbalCommandPort {
    in panAngle : Real;
    in tiltAngle : Real;
}
port def SensorDataPort {
    out position : String;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    let lines: Vec<&str> = content.lines().collect();
    let token_text = |(ln, start, len, _ty): &(u32, u32, u32, u32)| -> String {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        line_str
            .chars()
            .skip(*start as usize)
            .take(*len as usize)
            .collect()
    };

    for ident in ["position", "panAngle", "tiltAngle"] {
        let ident_tokens: Vec<_> = decoded.iter().filter(|t| token_text(t) == ident).collect();
        assert!(!ident_tokens.is_empty(), "should tokenize '{ident}'");
        for t in &ident_tokens {
            assert_ne!(
                t.3, 0,
                "{ident} must NOT be KEYWORD (valid identifier); got type {}",
                t.3
            );
        }
    }
}

#[test]
fn nested_port_usage_body_tokenizes_member_names() {
    let content = r#"package P {
  part vehicle {
    port vehicleToRoadPort {
      port leftWheelToRoadPort;
      port rightWheelToRoadPort;
    }
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    let lines: Vec<&str> = content.lines().collect();
    let token_text = |(ln, start, len, _ty): &(u32, u32, u32, u32)| -> String {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        line_str
            .chars()
            .skip(*start as usize)
            .take(*len as usize)
            .collect()
    };

    for ident in ["vehicleToRoadPort", "leftWheelToRoadPort", "rightWheelToRoadPort"] {
        let ident_tokens: Vec<_> = decoded.iter().filter(|t| token_text(t) == ident).collect();
        assert!(
            !ident_tokens.is_empty(),
            "should tokenize nested port member '{ident}'"
        );
    }
}
