use sysml_tokens::{
    ast_semantic_ranges, semantic_tokens_full, TYPE_CLASS, TYPE_KEYWORD, TYPE_PROPERTY, TYPE_TYPE,
};
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

fn tokens_on_line(content: &str, tokens: &[(u32, u32, u32, u32)], line: u32) -> Vec<(String, u32)> {
    let lines: Vec<&str> = content.lines().collect();
    tokens
        .iter()
        .filter(|(ln, _, _, _)| *ln == line)
        .map(|(ln, start, len, ty)| {
            let line_str = lines.get(*ln as usize).unwrap_or(&"");
            let text: String = line_str
                .chars()
                .skip(*start as usize)
                .take(*len as usize)
                .collect();
            (text, *ty)
        })
        .collect()
}

#[test]
fn definition_keywords_stay_keyword_after_ast_merge() {
    let content = r#"package P {
  state def Idle;
  item def StartMissionEvent;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    let state_line = tokens_on_line(content, &decoded, 1);
    assert_eq!(
        state_line
            .iter()
            .find(|(text, _)| text == "state")
            .map(|(_, ty)| *ty),
        Some(TYPE_KEYWORD),
        "state keyword on state def line: {:?}",
        state_line
    );
    assert_eq!(
        state_line
            .iter()
            .find(|(text, _)| text == "def")
            .map(|(_, ty)| *ty),
        Some(TYPE_KEYWORD),
        "def keyword on state def line: {:?}",
        state_line
    );
    assert_eq!(
        state_line
            .iter()
            .find(|(text, _)| text == "Idle")
            .map(|(_, ty)| *ty),
        Some(TYPE_CLASS),
        "definition name on state def line: {:?}",
        state_line
    );

    let item_line = tokens_on_line(content, &decoded, 2);
    assert_eq!(
        item_line
            .iter()
            .find(|(text, _)| text == "item")
            .map(|(_, ty)| *ty),
        Some(TYPE_KEYWORD),
        "item keyword on item def line: {:?}",
        item_line
    );
    assert_eq!(
        item_line
            .iter()
            .find(|(text, _)| text == "def")
            .map(|(_, ty)| *ty),
        Some(TYPE_KEYWORD),
        "def keyword on item def line: {:?}",
        item_line
    );
    assert_eq!(
        item_line
            .iter()
            .find(|(text, _)| text == "StartMissionEvent")
            .map(|(_, ty)| *ty),
        Some(TYPE_CLASS),
        "definition name on item def line: {:?}",
        item_line
    );
}

#[test]
fn state_def_body_tokenizes_final_state_and_transition_target() {
    let content = r#"package P {
  state def Lamp {
    state off;
    final state done;
    transition off_to_done first off accept ButtonPress then done;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "done"), "final state name");
    assert!(
        token_text(content, &decoded, "off"),
        "transition source/target"
    );
}

#[test]
fn nested_state_usage_tokenizes_name_and_type() {
    let content = r#"package P {
  state def Machine {
    state idle : Idle;
  }
  state def Idle;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    let usage_line = tokens_on_line(content, &decoded, 2);
    assert_eq!(
        usage_line
            .iter()
            .find(|(t, _)| t == "idle")
            .map(|(_, ty)| *ty),
        Some(TYPE_PROPERTY),
        "usage line: {:?}",
        usage_line
    );
    assert_eq!(
        usage_line
            .iter()
            .find(|(t, _)| t == "Idle")
            .map(|(_, ty)| *ty),
        Some(TYPE_TYPE),
        "usage line: {:?}",
        usage_line
    );
}

#[test]
fn action_usage_tokenizes_send_payload() {
    let content = r#"package P {
  action def Notify {
    action sendAlert : AlertAction send payload : Message;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "payload"));
    assert!(token_text(content, &decoded, "Message"));
}

#[test]
fn interface_def_body_tokenizes_end_names() {
    let content = r#"package P {
  interface def Link {
    end source : SourcePort;
    end sink : SinkPort;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "source"));
    assert!(token_text(content, &decoded, "sink"));
    assert!(token_text(content, &decoded, "SourcePort"));
}
