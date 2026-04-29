use tower_lsp::lsp_types::{SymbolKind, Url};

use crate::language::SymbolEntry;

use crate::semantic::SemanticGraph;

use super::signature_from_node;

/// Maps element_kind from the semantic model to LSP SymbolKind.
fn element_kind_to_symbol_kind(kind: &str) -> SymbolKind {
    match kind {
        "package" => SymbolKind::MODULE,
        "part def" => SymbolKind::CLASS,
        "part" => SymbolKind::VARIABLE,
        "attribute def" => SymbolKind::PROPERTY,
        "attribute" => SymbolKind::PROPERTY,
        "port def" => SymbolKind::INTERFACE,
        "port" => SymbolKind::INTERFACE,
        "interface" => SymbolKind::INTERFACE,
        "alias" => SymbolKind::KEY,
        "connection" => SymbolKind::VARIABLE,
        "connection def" => SymbolKind::INTERFACE,
        "item def" => SymbolKind::CONSTANT,
        "item" => SymbolKind::CONSTANT,
        "individual def" => SymbolKind::OBJECT,
        "requirement def" => SymbolKind::STRING,
        "requirement" => SymbolKind::STRING,
        "action def" => SymbolKind::FUNCTION,
        "metadata def" => SymbolKind::STRUCT,
        "enum def" => SymbolKind::ENUM,
        "occurrence def" => SymbolKind::CLASS,
        "occurrence" => SymbolKind::VARIABLE,
        "flow def" => SymbolKind::INTERFACE,
        "flow" => SymbolKind::VARIABLE,
        "allocation def" => SymbolKind::INTERFACE,
        "allocation" => SymbolKind::VARIABLE,
        "dependency" => SymbolKind::OPERATOR,
        "constraint def" => SymbolKind::FUNCTION,
        "calc def" => SymbolKind::FUNCTION,
        "case def" => SymbolKind::EVENT,
        "case" => SymbolKind::EVENT,
        "analysis def" => SymbolKind::EVENT,
        "analysis" => SymbolKind::EVENT,
        "verification def" => SymbolKind::EVENT,
        "verification" => SymbolKind::EVENT,
        "generic decl" => SymbolKind::NAMESPACE,
        "state def" => SymbolKind::ENUM_MEMBER,
        "state" => SymbolKind::ENUM_MEMBER,
        "use case def" => SymbolKind::EVENT,
        "actor def" => SymbolKind::CONSTRUCTOR,
        _ => SymbolKind::NULL,
    }
}

/// Collects symbol entries for a URI from the semantic graph (replaces AST-based collect_symbol_entries).
pub fn symbol_entries_for_uri(graph: &SemanticGraph, uri: &Url) -> Vec<SymbolEntry> {
    let mut out = Vec::new();
    for node in graph.nodes_for_uri(uri) {
        let container_name = node
            .parent_id
            .as_ref()
            .and_then(|pid| graph.get_node(pid))
            .map(|p| p.name.clone());
        let description = format!("{} '{}'", node.element_kind, node.name);
        let signature = signature_from_node(node);
        out.push(SymbolEntry {
            name: node.name.clone(),
            uri: node.id.uri.clone(),
            range: node.range,
            kind: element_kind_to_symbol_kind(&node.element_kind),
            container_name,
            detail: Some(node.element_kind.clone()),
            description: Some(description),
            signature,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use tower_lsp::lsp_types::Url;

    use sysml_v2_parser::parse;

    use crate::semantic::build_graph_from_doc;

    use super::symbol_entries_for_uri;

    #[test]
    fn symbol_entries_include_aliases_and_definitions() {
        let input = r#"
            standard library package SI {
                attribute <m> metre : LengthUnit;
                attribute <kg> kilogram : MassUnit;
                attribute tonne : MassUnit;
                alias 'metric ton' for tonne;
                alias arcmin for metre;
                alias arcsec for kilogram;
            }
        "#;
        let root = parse(input).expect("parse");
        let uri = Url::parse("file:///si.sysml").expect("uri");
        let graph = build_graph_from_doc(&root, &uri);
        let symbols = symbol_entries_for_uri(&graph, &uri);
        let names: std::collections::HashSet<String> =
            symbols.iter().map(|s| s.name.clone()).collect();
        let has_name = |needle: &str| {
            names.iter().any(|n| {
                n == needle
                    || n.ends_with(&format!(" {}", needle))
                    || n.ends_with(&format!("'{}'", needle))
            })
        };
        assert!(
            names.contains("tonne"),
            "expected 'tonne' symbol in {:?}",
            names
        );
        assert!(has_name("metre"), "expected 'metre' symbol in {:?}", names);
        assert!(
            has_name("kilogram"),
            "expected 'kilogram' symbol in {:?}",
            names
        );
        assert!(
            names.contains("metric ton"),
            "expected alias 'metric ton' symbol in {:?}",
            names
        );
        assert!(
            names.contains("arcmin"),
            "expected alias 'arcmin' symbol in {:?}",
            names
        );
        assert!(
            names.contains("arcsec"),
            "expected alias 'arcsec' symbol in {:?}",
            names
        );
        assert!(
            symbols.len() >= 7,
            "expected at least package + attributes + aliases, got {} symbols: {:?}",
            symbols.len(),
            names
        );
    }
}
