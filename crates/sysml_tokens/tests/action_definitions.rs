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
fn action_def_body_tokenizes_nested_action_usage_names() {
    let content = r#"package P {
  action def Step;
  action def Pipeline {
    action step1 : Step;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(
        token_text(content, &decoded, "step1"),
        "nested action usage name should be tokenized"
    );
}

#[test]
fn action_def_body_tokenizes_then_action_step_and_type() {
    let content = r#"package P {
  action def A;
  action def Pipeline {
    then action step : A;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "step"));
    assert!(token_text(content, &decoded, "A"));
}

#[test]
fn requirement_def_body_tokenizes_subject_and_stakeholder() {
    let content = r#"package P {
  part def Vehicle;
  requirement def SafetyReq {
    subject vehicle : Vehicle;
    stakeholder auditor;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "vehicle"));
    assert!(token_text(content, &decoded, "auditor"));
}
