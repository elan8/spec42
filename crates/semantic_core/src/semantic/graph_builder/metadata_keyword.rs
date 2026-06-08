//! User-defined metadata keyword usages (`#keyword`) on the semantic graph.

use std::collections::HashMap;

use sysml_v2_parser::ast::MetadataKeywordUsage;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn add_metadata_keyword_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    mk: &MetadataKeywordUsage,
    span: &sysml_v2_parser::Span,
) {
    let name = format!("_{}", mk.keyword);
    let qualified = qualified_name_for_node(
        g,
        uri,
        Some(parent_id.qualified_name.as_str()),
        &name,
        "metadata keyword",
    );
    let mut attrs = HashMap::new();
    attrs.insert("keyword".to_string(), serde_json::json!(&mk.keyword));
    if let Some(ref type_name) = mk.type_name {
        attrs.insert("keywordType".to_string(), serde_json::json!(type_name));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "metadata keyword",
        mk.keyword.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
}
