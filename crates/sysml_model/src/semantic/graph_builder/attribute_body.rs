//! Shared walker for `AttributeBody` members (metadata, item def, individual def).

use std::collections::HashMap;

use sysml_v2_parser::ast::{AttributeBody, AttributeBodyElement};
use url::Url;

use super::{
    add_node_and_recurse, attach_declared_name, attach_declared_subsetting_family, expressions,
    qualified_name_for_node, unit_metadata, usage_builders,
};
use crate::semantic::ast_util::{span_to_range, typing_targets};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{NodeId, RelationshipKind};
use crate::semantic::relationships::add_typing_edge_if_exists;

/// Attaches any `doc` comments written inside a nested attribute def/usage's own
/// braces (e.g. a unit-catalog definition's `symbol`/`conversionFactor` block) to
/// that attribute's node. `build_from_attribute_body` only walks sibling elements,
/// so this body is otherwise never visited for graph materialization.
fn attach_nested_doc_comments(g: &mut SemanticGraph, node_id: &NodeId, body: &AttributeBody) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };
    for node in elements {
        if let AttributeBodyElement::Doc(doc) = &node.value {
            super::attach_doc_comment(g, node_id, &doc.value.text);
        }
    }
}

pub(super) fn build_from_attribute_body(
    body: &AttributeBody,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: &NodeId,
    g: &mut SemanticGraph,
) {
    let AttributeBody::Brace { elements } = body else {
        return;
    };

    for node in elements {
        match &node.value {
            AttributeBodyElement::AttributeDef(attribute) => {
                let value = &attribute.value;
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, &value.name, "attribute def");
                let attrs = HashMap::new();
                g.register_declared_membership_facts(
                    NodeId::new(uri, &qualified),
                    crate::semantic::ast_util::declared_membership_facts(&value.membership),
                );
                if let Some(short_name) = unit_metadata::attribute_def_short_name(value) {
                    g.register_declared_short_name(NodeId::new(uri, &qualified), short_name);
                }
                let targets = typing_targets(value.typing.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute def",
                    value.name.clone(),
                    span_to_range(&attribute.span),
                    attrs,
                    Some(parent_id),
                );
                let node_id = NodeId::new(uri, &qualified);
                if let Some(node) = g.get_node_mut(&node_id) {
                    node.declared_facts.unit = unit_metadata::attribute_def_unit_facts(value);
                }
                if let Some(expr_node) = &value.value {
                    let rendered =
                        expressions::expression_to_debug_string(&expr_node.value.expression);
                    if let Some(node) = g.get_node_mut(&node_id) {
                        node.expression_text.value = Some(rendered.clone());
                        node.expression_text.default_value = Some(rendered);
                    }
                }
                for target in targets.iter().copied() {
                    add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
                }
                attach_nested_doc_comments(g, &node_id, &value.body);
            }
            AttributeBodyElement::AttributeUsage(attribute) => {
                let value = &attribute.value;
                let name = super::effective_usage_name(&value.name, value.redefines.as_deref());
                let qualified =
                    qualified_name_for_node(g, uri, container_prefix, name, "attribute");
                let attrs = HashMap::new();
                g.register_declared_membership_facts(
                    NodeId::new(uri, &qualified),
                    crate::semantic::ast_util::declared_membership_facts(&value.membership),
                );
                let targets = typing_targets(value.typing.as_deref());
                add_node_and_recurse(
                    g,
                    uri,
                    &qualified,
                    "attribute",
                    name.to_string(),
                    span_to_range(&attribute.span),
                    attrs,
                    Some(parent_id),
                );
                let node_id = NodeId::new(uri, &qualified);
                if let Some(expr_node) = &value.value {
                    if let Some(node) = g.get_node_mut(&node_id) {
                        node.expression_text.value = Some(expressions::expression_to_debug_string(
                            &expr_node.value.expression,
                        ));
                    }
                }
                attach_declared_name(g, &node_id, &value.name);
                attach_declared_subsetting_family(
                    g,
                    &node_id,
                    value.subsets.as_deref(),
                    value.redefines.as_deref(),
                    value.references.as_deref(),
                    value.crosses.as_deref(),
                );
                // The KerML entailment that a `:>>` redefinition of a semantic-metadata
                // restriction feature (`annotatedElement`/`baseType`) is also a subsetting of
                // that feature (`Redefinition` specializes `Subsetting`,
                // `org.omg.sysml/model/kerml.ecore:1471` in the OMG pilot) is published as an
                // *implied* graph edge by `relationships::link_subsetting_family_edges_for_node`,
                // not recorded here as a second declared fact -- `DeclaredRelationshipTarget` has
                // no provenance field, so writing it here would make an entailed relationship
                // look authored (`planning/UNIFY_CACHE_PROGRESS.md` B9, `AGENTS.md` "One semantic
                // system").
                if let Some(node) = g.get_node_mut(&node_id) {
                    node.declared_facts.unit = unit_metadata::attribute_usage_unit_facts(value);
                }
                for target in targets.iter().copied() {
                    add_typing_edge_if_exists(g, uri, &qualified, target, container_prefix);
                }
                attach_nested_doc_comments(g, &node_id, &value.body);
            }
            AttributeBodyElement::Doc(doc) => {
                super::attach_doc_comment(g, parent_id, &doc.value.text);
            }
            // §6 G27: this body is shared with `item def`/`item` usage bodies, which legally
            // own occurrence members.
            AttributeBodyElement::OccurrenceUsage(occ_node) => {
                usage_builders::materialize_occurrence_usage(
                    occ_node,
                    uri,
                    container_prefix,
                    Some(parent_id),
                    g,
                );
            }
            // Also shared with `item def`/`item` usage bodies, which legally own connector
            // members (`connect a to b;`) and metadata tags (see the FMEA library example in
            // `14c-Language Extensions.sysml`).
            AttributeBodyElement::Connect(c) => {
                expressions::add_expression_edge_if_both_exist(
                    g,
                    uri,
                    container_prefix,
                    crate::semantic::ast_util::connection_end_expression(&c.from),
                    crate::semantic::ast_util::connection_end_expression(&c.to),
                    RelationshipKind::Connection,
                );
            }
            AttributeBodyElement::MetadataKeywordUsage(mk_node) => {
                super::metadata_keyword::add_metadata_keyword_node(
                    g,
                    uri,
                    parent_id,
                    &mk_node.value,
                    &mk_node.span,
                );
            }
            // Also shared with `item def`/`item` usage bodies: `ref`/`ref part` members
            // (validation `15_11`/`15_19`/`17a`/`17b`). Same materialization interface_def.rs/
            // action.rs already use for `ref` in their own body contexts.
            AttributeBodyElement::RefDecl(ref_node) => {
                super::ref_decl::materialize_ref_decl(
                    g,
                    uri,
                    container_prefix,
                    parent_id,
                    ref_node,
                    super::ref_decl::RefDeclOptions::default(),
                );
            }
            // Nested `part` usage inside an item/attribute body (validation `3e`/`14c`).
            AttributeBodyElement::PartUsage(part) => {
                usage_builders::materialize_part_usage(
                    part,
                    uri,
                    container_prefix,
                    Some(parent_id),
                    g,
                );
            }
            // `assert constraint` here mirrors `part_def.rs`'s/`action.rs`'s existing
            // `AssertConstraint(_) => {}` precedent -- no dedicated graph projection yet for any
            // body kind except `occurrence_body.rs`'s.
            AttributeBodyElement::AssertConstraint(_) => {}
            AttributeBodyElement::Error(_) | AttributeBodyElement::Other(_) => {}
        }
    }
}
