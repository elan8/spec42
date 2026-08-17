use crate::common::text_span::to_lsp_range;
use crate::semantic::SemanticNode;
use sysml_query::resolved_slice::{ElementInspection, TextRange};
use tower_lsp::lsp_types::{
    CallHierarchyItem, Moniker, MonikerKind, Position, Range, SymbolKind, TypeHierarchyItem,
    UniquenessLevel, Url,
};

pub(crate) fn moniker_for_node(node: &SemanticNode) -> Moniker {
    Moniker {
        scheme: "spec42".to_string(),
        identifier: format!("{}#{}", node.id.uri, node.id.qualified_name),
        unique: UniquenessLevel::Scheme,
        kind: Some(MonikerKind::Export),
    }
}

/// A type-hierarchy item for one published element.
///
/// `selection_range` is the element's declaration range rather than its name range, because the
/// client sends the item back for `typeHierarchy/supertypes`, and the position it is resolved from
/// must land inside the declaration whether or not the element is named.
pub(crate) fn type_hierarchy_item(inspection: &ElementInspection) -> Option<TypeHierarchyItem> {
    let uri = Url::parse(&inspection.location.document).ok()?;
    let range = lsp_range(inspection.declaration_range);
    Some(TypeHierarchyItem {
        name: inspection.name.as_deref().unwrap_or_default().to_string(),
        kind: SymbolKind::CLASS,
        tags: None,
        detail: Some(inspection.kind.as_str().to_string()),
        uri,
        range,
        selection_range: range,
        data: None,
    })
}

fn lsp_range(range: TextRange) -> Range {
    Range {
        start: Position::new(range.start.line, range.start.character),
        end: Position::new(range.end.line, range.end.character),
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
