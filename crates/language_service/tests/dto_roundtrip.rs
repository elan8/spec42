use language_service::{DefinitionResult, HoverResult, ReferencesResult, SourceLocation};
use semantic_core::{TextPosition, TextRange};

#[test]
fn dto_roundtrip_serde() {
    let hover = HoverResult {
        contents: "**part**".to_string(),
        range: Some(TextRange {
            start: TextPosition {
                line: 0,
                character: 2,
            },
            end: TextPosition {
                line: 0,
                character: 6,
            },
        }),
    };
    let json = serde_json::to_string(&hover).expect("serialize hover");
    let parsed: HoverResult = serde_json::from_str(&json).expect("deserialize hover");
    assert_eq!(hover, parsed);

    let definition = DefinitionResult {
        locations: vec![SourceLocation {
            path: "model.sysml".to_string(),
            range: hover.range.unwrap(),
        }],
    };
    let json = serde_json::to_string(&definition).expect("serialize definition");
    let parsed: DefinitionResult = serde_json::from_str(&json).expect("deserialize definition");
    assert_eq!(definition, parsed);

    let references = ReferencesResult {
        locations: definition.locations,
    };
    let json = serde_json::to_string(&references).expect("serialize references");
    let parsed: ReferencesResult = serde_json::from_str(&json).expect("deserialize references");
    assert_eq!(references, parsed);
}
