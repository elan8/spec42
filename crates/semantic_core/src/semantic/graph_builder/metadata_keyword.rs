//! User-defined metadata keyword usages (`#keyword`) on the semantic graph.

use std::collections::HashMap;

use sysml_v2_parser::ast::MetadataKeywordUsage;
use url::Url;

use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;
use crate::semantic::relationships::{
    add_typing_edge_if_exists, wire_metadata_annotated_elements,
};

use super::attribute_body::build_from_attribute_body;
use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn add_metadata_keyword_node(
    g: &mut SemanticGraph,
    uri: &Url,
    parent_id: &NodeId,
    mk: &MetadataKeywordUsage,
    span: &sysml_v2_parser::Span,
) {
    let container_prefix = Some(parent_id.qualified_name.as_str());
    let name = format!("_{}", mk.keyword);
    let qualified = qualified_name_for_node(
        g,
        uri,
        container_prefix,
        &name,
        "metadata keyword",
    );
    let mut attrs = HashMap::new();
    attrs.insert("keyword".to_string(), serde_json::json!(&mk.keyword));
    if let Some(ref type_name) = mk.type_name {
        attrs.insert("keywordType".to_string(), serde_json::json!(type_name));
    }
    if !mk.about_targets.is_empty() {
        attrs.insert(
            "aboutTargets".to_string(),
            serde_json::json!(&mk.about_targets),
        );
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
    if let Some(ref type_name) = mk.type_name {
        add_typing_edge_if_exists(g, uri, &qualified, type_name, container_prefix);
    }
    build_from_attribute_body(&mk.body, uri, Some(&qualified), &NodeId::new(uri, &qualified), g);
    let metadata_id = NodeId::new(uri, &qualified);
    wire_metadata_annotated_elements(g, uri, &metadata_id, parent_id, &mk.about_targets);
}
