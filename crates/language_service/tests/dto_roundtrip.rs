use language_service::{
    CompletionItemDto, CompletionItemKindDto, CompletionResult, FoldingRangeDto,
    FoldingRangeKindDto, OutlineSymbol, TextEditDto, TextEditSuggestion, WorkspaceSymbolMatch,
};
use semantic_core::{TextPosition, TextRange};

#[test]
fn dto_roundtrip_serde_phase1() {
    use language_service::{DefinitionResult, HoverResult, ReferencesResult, SourceLocation};

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

#[test]
fn dto_roundtrip_serde_extended() {
    let completion = CompletionResult {
        items: vec![CompletionItemDto {
            label: "part def".to_string(),
            kind: Some(CompletionItemKindDto::Snippet),
            detail: None,
            documentation: None,
            documentation_is_markdown: false,
            label_details: None,
            filter_text: None,
            text_edit: None,
            insert_text_format_snippet: true,
            sort_text: None,
            preselect: false,
            deprecated: false,
            resolve_detail: None,
            resolve_documentation: None,
        }],
        is_incomplete: false,
    };
    let json = serde_json::to_string(&completion).expect("serialize completion");
    let parsed: CompletionResult = serde_json::from_str(&json).expect("deserialize completion");
    assert_eq!(completion, parsed);

    let outline = OutlineSymbol {
        name: "P".to_string(),
        kind: "package".to_string(),
        range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 10)),
        selection_range: TextRange::new(TextPosition::new(0, 8), TextPosition::new(0, 9)),
        children: vec![],
    };
    let json = serde_json::to_string(&outline).expect("serialize outline");
    let parsed: OutlineSymbol = serde_json::from_str(&json).expect("deserialize outline");
    assert_eq!(outline, parsed);

    let folding = FoldingRangeDto {
        start_line: 0,
        end_line: 3,
        kind: Some(FoldingRangeKindDto::Region),
    };
    let json = serde_json::to_string(&folding).expect("serialize folding");
    let parsed: FoldingRangeDto = serde_json::from_str(&json).expect("deserialize folding");
    assert_eq!(folding, parsed);

    let workspace_match = WorkspaceSymbolMatch {
        name: "Engine".to_string(),
        path: "model.sysml".to_string(),
        uri: "file:///model.sysml".to_string(),
        range: TextRange::new(TextPosition::new(1, 4), TextPosition::new(1, 10)),
        container: Some("P".to_string()),
        detail: Some("part def".to_string()),
    };
    let json = serde_json::to_string(&workspace_match).expect("serialize workspace match");
    let parsed: WorkspaceSymbolMatch = serde_json::from_str(&json).expect("deserialize workspace match");
    assert_eq!(workspace_match, parsed);

    let suggestion = TextEditSuggestion {
        title: "Wrap in package".to_string(),
        edits: vec![TextEditDto {
            path: "test.sysml".to_string(),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(0, 5)),
            replacement: "package P {}".to_string(),
        }],
    };
    let json = serde_json::to_string(&suggestion).expect("serialize suggestion");
    let parsed: TextEditSuggestion = serde_json::from_str(&json).expect("deserialize suggestion");
    assert_eq!(suggestion, parsed);
}
