use sysml_tokens::{ast_semantic_ranges, semantic_tokens_full, TYPE_PROPERTY};
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

fn token_type_for(content: &str, tokens: &[(u32, u32, u32, u32)], ident: &str) -> Option<u32> {
    let lines: Vec<&str> = content.lines().collect();
    tokens.iter().find_map(|(ln, start, len, ty)| {
        let line_str = lines.get(*ln as usize).unwrap_or(&"");
        let text: String = line_str
            .chars()
            .skip(*start as usize)
            .take(*len as usize)
            .collect();
        if text == ident {
            Some(*ty)
        } else {
            None
        }
    })
}

#[test]
fn part_def_body_tokenizes_ref_and_part_usage_names() {
    let content = r#"package P {
  part def Wheel;
  part def Vehicle {
    ref axle : Wheel;
    part wheel : Wheel;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(token_text(content, &decoded, "axle"));
    assert!(token_text(content, &decoded, "wheel"));
    assert!(token_text(content, &decoded, "Wheel"));
}

#[test]
fn item_def_body_tokenizes_inner_attribute_name() {
    let content = r#"package P {
  item def Payload {
    attribute weight : Real;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(
        token_text(content, &decoded, "weight"),
        "item def inner attribute name should be tokenized"
    );
}

#[test]
fn item_def_body_tokenizes_short_and_long_attribute_names_as_properties() {
    let content = r#"package P {
  item def OccupancyGrid {
    attribute xMm : Real;
    attribute cleanedCellPercent : Real;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_for(content, &decoded, "xMm"),
        Some(TYPE_PROPERTY)
    );
    assert_eq!(
        token_type_for(content, &decoded, "cleanedCellPercent"),
        Some(TYPE_PROPERTY)
    );
}

#[test]
fn metadata_def_body_tokenizes_inner_attribute_name() {
    let content = r#"package P {
  metadata def Safety {
    attribute isMandatory : Boolean;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);
    assert!(
        token_text(content, &decoded, "isMandatory"),
        "metadata def inner attribute name should be tokenized"
    );
}
