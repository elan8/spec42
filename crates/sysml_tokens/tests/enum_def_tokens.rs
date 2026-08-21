use sysml_tokens::{ast_semantic_ranges, semantic_tokens_full, TYPE_CLASS, TYPE_PROPERTY};
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

/// Regression test for a real bug: a multi-line `enum def` body's literal members
/// (`navigationSensorSuite`, `cleaningPerformance`, ...) inherited the whole enum node's
/// `TYPE_CLASS` token, because `narrow_declaration_name_range` used to give up on multi-line
/// spans and leave the wide, unnarrowed range in place. All of the enum's members rendered in
/// the same color as the enum's own name, indistinguishable from a real type reference.
#[test]
fn multiline_enum_def_narrows_class_token_to_name_only() {
    let content = r#"package P {
  enum def VariantConcernKind {
    productTier;
    navigationSensorSuite;
    cleaningPerformance;
    batteryPack;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_for(content, &decoded, "VariantConcernKind"),
        Some(TYPE_CLASS),
        "the enum's own name should still be classified as a class/type"
    );
    for member in [
        "productTier",
        "navigationSensorSuite",
        "cleaningPerformance",
        "batteryPack",
    ] {
        assert_ne!(
            token_type_for(content, &decoded, member),
            Some(TYPE_CLASS),
            "enum member `{member}` must not inherit the enum's class token"
        );
    }
}

#[test]
fn multiline_enum_def_members_get_their_own_token() {
    let content = r#"package P {
  enum def ProductTier {
    entryLevel;
    flagship;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_for(content, &decoded, "entryLevel"),
        Some(TYPE_PROPERTY),
        "enum members should be classified like named constants, not left uncolored"
    );
    assert_eq!(
        token_type_for(content, &decoded, "flagship"),
        Some(TYPE_PROPERTY)
    );
}

/// Regression test: `entry`, `standard`, and `concern` are reserved keywords in other SysML
/// contexts (`entry`/`exit` actions, `concern def`), so the lexer tags them TYPE_KEYWORD purely
/// by spelling. When used as enum members or an attribute name, the AST pass correctly computes
/// TYPE_PROPERTY for them, but the merge guard in `apply_ast_semantic_ranges` used to discard any
/// AST override on a lexer-tagged keyword token, so these identifiers rendered as keywords
/// instead of properties.
#[test]
fn keyword_lookalike_identifiers_are_classified_as_properties() {
    let content = r#"package P {
  enum def ProductTier {
    entry;
    standard;
    premium;
  }
  part def VariantOption {
    attribute concern : String;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    for ident in ["entry", "standard", "concern"] {
        assert_eq!(
            token_type_for(content, &decoded, ident),
            Some(TYPE_PROPERTY),
            "`{ident}` is used here as an identifier, not a keyword, and must be colored as a property"
        );
    }
}

#[test]
fn single_line_enum_def_still_narrows_correctly() {
    let content = "package P {\n  enum def Status { pending; released; }\n}";
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_for(content, &decoded, "Status"),
        Some(TYPE_CLASS)
    );
    assert_eq!(
        token_type_for(content, &decoded, "pending"),
        Some(TYPE_PROPERTY)
    );
}
