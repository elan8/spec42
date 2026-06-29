use sysml_tokens::{ast_semantic_ranges, semantic_tokens_full};
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

fn token_text(content: &str, tokens: &[(u32, u32, u32, u32)], ident: &str) -> bool {
    let lines: Vec<&str> = content.lines().collect();
    tokens.iter().any(|(ln, start, len, _ty)| {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        line_str
            .chars()
            .skip(*start as usize)
            .take(*len as usize)
            .collect::<String>()
            == ident
    })
}

#[test]
fn flow_def_body_tokenizes_inner_attribute_name() {
    let content = r#"package P {
  flow def Power {
    attribute rate : Real;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(
        token_text(content, &decoded, "rate"),
        "flow def inner attribute name should be tokenized"
    );
}

#[test]
fn flow_def_body_tokenizes_nested_part_usage_name() {
    let content = r#"package P {
  part def Wheel;
  flow def PowerFlow {
    part wheel : Wheel;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "wheel"));
    assert!(token_text(content, &decoded, "Wheel"));
}
