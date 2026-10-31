use crate::common::text_span::to_lsp_range;
use sysml_query::resolved_slice::ElementInspection;
use tower_lsp::lsp_types::{SymbolKind, TypeHierarchyItem, Url};

/// Opaque identity bound to the publication that produced the hierarchy item.
#[derive(serde::Serialize, serde::Deserialize)]
struct ItemIdentity {
    symbol: String,
    publication: String,
}

pub(crate) fn item_symbol(
    model: &sysml_query::resolved_slice::PublishedModel,
    item: &TypeHierarchyItem,
) -> tower_lsp::jsonrpc::Result<sysml_query::resolved_slice::SymbolId> {
    use tower_lsp::jsonrpc::Error;
    let identity: ItemIdentity = serde_json::from_value(
        item.data
            .clone()
            .ok_or_else(|| Error::invalid_params("Missing hierarchy identity"))?,
    )
    .map_err(|_| Error::invalid_params("Invalid hierarchy identity"))?;
    if identity.publication != model.publication().model_digest().to_string() {
        return Err(Error::content_modified());
    }
    model
        .resolve_token(&sysml_query::resolved_slice::SymbolToken::from_encoded(
            identity.symbol,
        ))
        .ok_or_else(|| Error::invalid_params("Unknown hierarchy identity"))
}

/// A type-hierarchy item for one published element.
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
        selection_range: to_lsp_range(inspection.location.range),
        data: Some(
            serde_json::to_value(ItemIdentity {
                symbol: model.symbol_token(inspection.identity)?.into_string(),
                publication: model.publication().model_digest().to_string(),
            })
            .ok()?,
        ),
    })
}
