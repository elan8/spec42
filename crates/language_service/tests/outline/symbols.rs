use language_service::{document_symbols, folding_ranges, FoldingRangeKindDto, OutlineSymbol};
use sysml_v2_parser::{parse, RootNamespace};

fn multiline_outline_regions(symbols: &[OutlineSymbol]) -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    for symbol in symbols {
        if symbol.range.end.line > symbol.range.start.line {
            out.push((symbol.range.start.line, symbol.range.end.line));
        }
        out.extend(multiline_outline_regions(&symbol.children));
    }
    out
}

#[test]
fn folding_ranges_match_multiline_outline_regions() {
    let content = "package P {\n    part def Engine {\n        part cylinder;\n    }\n}\n";
    let root = parse(content).expect("parse");
    let symbols = document_symbols(&root);
    let expected = multiline_outline_regions(&symbols);
    let ranges = folding_ranges(&root);
    assert_eq!(
        ranges.len(),
        expected.len(),
        "fold count should match multiline outline regions (expected {:?}, got {:?})",
        expected,
        ranges
    );
    for (start, end) in expected {
        assert!(
            ranges
                .iter()
                .any(|r| r.start_line == start && r.end_line == end),
            "missing fold {start}-{end} in {:?}",
            ranges
        );
    }
    assert!(
        ranges
            .iter()
            .all(|r| r.kind == Some(FoldingRangeKindDto::Region)),
        "outline folds should be region kind: {:?}",
        ranges
    );
}

#[test]
fn document_symbols_empty() {
    let root = RootNamespace { elements: vec![] };
    let symbols = document_symbols(&root);
    assert!(symbols.is_empty());
}

#[test]
fn document_symbols_package() {
    let text = "package P { }";
    let root = sysml_v2_parser::parse(text).expect("parse");
    let symbols = document_symbols(&root);
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name, "P");
    assert_eq!(symbols[0].kind, "package");
}

#[test]
fn document_symbols_nested() {
    let text = "package P { part def Engine { } }";
    let root = sysml_v2_parser::parse(text).expect("parse");
    let symbols = document_symbols(&root);
    assert_eq!(symbols.len(), 1);
    assert_eq!(symbols[0].name, "P");
    let children = &symbols[0].children;
    assert_eq!(children.len(), 1);
    assert_eq!(children[0].name, "Engine");
    assert_eq!(children[0].kind, "part def");
}

#[test]
fn document_symbols_feature_and_classifier_decls() {
    let text = "package P { feature myFeature : BaseFeature; class VehicleClass; }";
    let root = sysml_v2_parser::parse(text).expect("parse");
    let symbols = document_symbols(&root);
    let children = &symbols[0].children;
    assert!(children.iter().any(|child| {
        child.name == "myFeature" && child.kind == "feature decl"
    }));
    assert!(children.iter().any(|child| {
        child.name == "VehicleClass" && child.kind == "classifier decl"
    }));
}
