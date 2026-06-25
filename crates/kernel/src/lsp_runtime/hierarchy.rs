use crate::common::text_span::to_lsp_range;
use crate::semantic::SemanticNode;
use tower_lsp::lsp_types::{
    CallHierarchyItem, Moniker, MonikerKind, SymbolKind, TypeHierarchyItem, UniquenessLevel,
};

pub(crate) fn moniker_for_node(node: &SemanticNode) -> Moniker {
    Moniker {
        scheme: "spec42".to_string(),
        identifier: format!("{}#{}", node.id.uri, node.id.qualified_name),
        unique: UniquenessLevel::Scheme,
        kind: Some(MonikerKind::Export),
    }
}

pub(crate) fn type_hierarchy_item_for_node(node: &SemanticNode) -> TypeHierarchyItem {
    TypeHierarchyItem {
        name: node.name.clone(),
        kind: SymbolKind::CLASS,
        tags: None,
        detail: Some(node.element_kind.as_str().to_string()),
        uri: node.id.uri.clone(),
        range: to_lsp_range(node.range),
        selection_range: to_lsp_range(node.range),
        data: None,
    }
}

pub(crate) fn call_hierarchy_item_for_node(node: &SemanticNode) -> CallHierarchyItem {
    CallHierarchyItem {
        name: node.name.clone(),
        kind: SymbolKind::FUNCTION,
        tags: None,
        detail: Some(node.element_kind.as_str().to_string()),
        uri: node.id.uri.clone(),
        range: to_lsp_range(node.range),
        selection_range: to_lsp_range(node.range),
        data: None,
    }
}
