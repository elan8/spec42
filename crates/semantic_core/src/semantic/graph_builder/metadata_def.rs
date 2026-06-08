use sysml_v2_parser::ast::AttributeBody;
use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;

use super::attribute_body::build_from_attribute_body;

pub(super) fn build_from_metadata_attribute_body(
    body: &AttributeBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    build_from_attribute_body(body, uri, container_prefix, parent_id, g);
}
