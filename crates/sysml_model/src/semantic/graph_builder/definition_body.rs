//! Generic `DefinitionBody` walker for flow def, occurrence def, and flow usage bodies.

use sysml_v2_parser::ast::{DefinitionBody, DefinitionBodyElement};
use url::Url;

use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;

use super::occurrence_body::build_from_occurrence_body_element;

pub(super) fn build_from_definition_body(
    body: &DefinitionBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let DefinitionBody::Brace { elements } = body else {
        return;
    };
    for element in elements {
        match &element.value {
            DefinitionBodyElement::OccurrenceMember(member) => {
                build_from_occurrence_body_element(member, uri, container_prefix, parent_id, g);
            }
            DefinitionBodyElement::Doc(_)
            | DefinitionBodyElement::Error(_)
            | DefinitionBodyElement::Other(_) => {}
        }
    }
}
