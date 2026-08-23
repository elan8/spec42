use crate::common::text_span::to_lsp_range;
use sysml_query::resolved_slice::ElementInspection;
use tower_lsp::lsp_types::{SymbolKind, TypeHierarchyItem, Url};

/// A type-hierarchy item for one published element.
///
/// `selection_range` is the element's declaration range rather than its name range, because the
/// client sends the item back for `typeHierarchy/supertypes`, and the position it is resolved from
/// must land inside the declaration whether or not the element is named.
pub(crate) fn type_hierarchy_item(
    model: &sysml_query::resolved_slice::PublishedModel,
    inspection: &ElementInspection,
) -> Option<TypeHierarchyItem> {
    let uri = Url::parse(model.document_identity(inspection.location.document)?).ok()?;
    let range = to_lsp_range(inspection.declaration_range);
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
