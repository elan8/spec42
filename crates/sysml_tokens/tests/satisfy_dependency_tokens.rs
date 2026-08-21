//! Golden-style checks for Satisfy / Dependency / KerML package-member semantic tokens.

use sysml_resolution::syntax::parse_for_editor;
use sysml_tokens::{
    ast_semantic_ranges, semantic_tokens_full, TYPE_CLASS, TYPE_INTERFACE, TYPE_KEYWORD,
    TYPE_PROPERTY, TYPE_TYPE,
};

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
fn satisfy_member_tokenizes_source_and_target() {
    let content = r#"package Demo {
  requirement def EnduranceReq;
  part def Drone;
  requirement endurance : EnduranceReq;
  part droneInstance : Drone;
  satisfy endurance by droneInstance;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_on_line(content, &decoded, 5, "satisfy"),
        Some(TYPE_KEYWORD)
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 5, "endurance"),
        Some(TYPE_PROPERTY),
        "satisfy source should be a property"
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 5, "droneInstance"),
        Some(TYPE_PROPERTY),
        "satisfy target should be a property"
    );
}

#[test]
fn dependency_member_tokenizes_clients_and_suppliers() {
    let content = r#"package Demo {
  part def Client;
  part def Supplier;
  part client : Client;
  part supplier : Supplier;
  dependency from client to supplier;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_on_line(content, &decoded, 5, "dependency"),
        Some(TYPE_KEYWORD)
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 5, "client"),
        Some(TYPE_PROPERTY),
        "dependency client should be a property"
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 5, "supplier"),
        Some(TYPE_PROPERTY),
        "dependency supplier should be a property"
    );
}

#[test]
fn kerml_classifier_and_feature_decls_tokenized() {
    let content = r#"package KerMLDecls {
  datatype Magnitude;
  feature baseType;
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    assert_eq!(
        token_type_on_line(content, &decoded, 1, "Magnitude"),
        Some(TYPE_CLASS),
        "KerML classifier decl name should be class, got {:?}",
        token_type_on_line(content, &decoded, 1, "Magnitude")
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 2, "baseType"),
        Some(TYPE_PROPERTY),
        "KerML feature decl name should be property, got {:?}",
        token_type_on_line(content, &decoded, 2, "baseType")
    );
}

#[test]
fn vehicle_definitions_fixture_tokenizes_part_port_and_interface_names() {
    // Self-contained excerpt patterned on the OMG Vehicle Example's VehicleDefinitions.sysml
    // (no library imports required for these declaration headers).
    let content = r#"package VehicleDefinitions {
  part def Vehicle {
    attribute mass : Real;
  }
  part def Transmission;
  part def AxleAssembly;
  part def Axle {
    port leftMountingPoint : AxleMountIF;
    port rightMountingPoint : AxleMountIF;
  }
  part def Wheel {
    port hub : WheelHubIF;
  }
  port def AxleMountIF;
  port def WheelHubIF;
  interface def Mounting {
    end axleMount : AxleMountIF;
    end hub : WheelHubIF;
  }
}"#;
    let parsed = parse_for_editor(content);
    let ranges = ast_semantic_ranges(&parsed.document, content);
    let (tokens, _) = semantic_tokens_full(content, Some(&ranges));
    let decoded = decode_semantic_tokens(&tokens.data);

    for (line, name, expected) in [
        (1, "Vehicle", TYPE_CLASS),
        (4, "Transmission", TYPE_CLASS),
        (5, "AxleAssembly", TYPE_CLASS),
        (6, "Axle", TYPE_CLASS),
        (10, "Wheel", TYPE_CLASS),
        (13, "AxleMountIF", TYPE_TYPE),
        (14, "WheelHubIF", TYPE_TYPE),
        (15, "Mounting", TYPE_INTERFACE),
    ] {
        assert_eq!(
            token_type_on_line(content, &decoded, line, name),
            Some(expected),
            "{name} on line {line}"
        );
    }

    assert_eq!(
        token_type_on_line(content, &decoded, 2, "mass"),
        Some(TYPE_PROPERTY)
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 7, "leftMountingPoint"),
        Some(TYPE_PROPERTY)
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 7, "AxleMountIF"),
        Some(TYPE_TYPE)
    );
    assert_eq!(
        token_type_on_line(content, &decoded, 16, "axleMount"),
        Some(TYPE_PROPERTY)
    );
}
