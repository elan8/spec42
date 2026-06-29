//! Golden-style semantic token checks on real fixture snippets.

use std::fs;
use std::path::PathBuf;

use sysml_tokens::{
    ast_semantic_ranges, semantic_tokens_full, TYPE_CLASS, TYPE_FUNCTION, TYPE_KEYWORD, TYPE_PROPERTY,
    TYPE_TYPE,
};
use sysml_v2_parser::parse_for_editor;

fn fixture_path(relative: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join(relative)
}

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

fn token_type_on_line(
    content: &str,
    tokens: &[(u32, u32, u32, u32)],
    line: u32,
    ident: &str,
) -> Option<u32> {
    let lines: Vec<&str> = content.lines().collect();
    tokens.iter().find_map(|(ln, start, len, ty)| {
        if *ln != line {
            return None;
        }
        let line_str = lines.get(*ln as usize)?;
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
fn state_machine_demo_fixture_tokens() {
    let path = fixture_path("vscode/testFixture/workspaces/state-view/StateMachineDemo.sysml");
    let content = fs::read_to_string(&path).expect("read StateMachineDemo.sysml");
    let parsed = parse_for_editor(&content);
    let ranges = ast_semantic_ranges(&parsed.root, &content);
    let (tokens, _) = semantic_tokens_full(&content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    // Top-level state defs: keywords stay keyword, names are class.
    assert_eq!(
        token_type_on_line(&content, &decoded, 6, "state"),
        Some(TYPE_KEYWORD)
    );
    assert_eq!(
        token_type_on_line(&content, &decoded, 6, "def"),
        Some(TYPE_KEYWORD)
    );
    assert_eq!(
        token_type_on_line(&content, &decoded, 6, "Idle"),
        Some(TYPE_CLASS)
    );

    // Nested state usage: `state idle : Idle;`
    assert_eq!(
        token_type_on_line(&content, &decoded, 12, "state"),
        Some(TYPE_KEYWORD)
    );
    assert_eq!(
        token_type_on_line(&content, &decoded, 12, "idle"),
        Some(TYPE_PROPERTY)
    );
    assert_eq!(
        token_type_on_line(&content, &decoded, 12, "Idle"),
        Some(TYPE_TYPE)
    );
}

#[test]
fn constraint_def_package_member_tokens() {
    let content = r#"package P {
  constraint def MassLimit;
  calc def TotalMass;
  enum def Status;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.root, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    for (line, name, expected) in [
        (1, "MassLimit", TYPE_CLASS),
        (2, "TotalMass", TYPE_FUNCTION),
        (3, "Status", TYPE_CLASS),
    ] {
        assert_eq!(
            token_type_on_line(content, &decoded, line, "def"),
            Some(TYPE_KEYWORD),
            "def on line {line}"
        );
        assert_eq!(
            token_type_on_line(content, &decoded, line, name),
            Some(expected),
            "{name} on line {line}"
        );
    }
}
