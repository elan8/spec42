//! Raw KerML library declarations (`KermlSemanticDecl`/`KermlFeatureDecl`/`ExtendedLibraryDecl`):
//! opaque BNF-tagged text captured verbatim from KerML standard-library sources that don't have
//! a dedicated SysML grammar production.

use std::collections::HashMap;

use sysml_v2_parser::ast::{ExtendedLibraryDecl, KermlFeatureDecl, KermlSemanticDecl};
use sysml_v2_parser::Node;
use url::Url;

use super::modeled_kerml_name::{extract_kerml_feature_names_from_text, extract_modeled_decl_name};
use super::{add_node_and_recurse, qualified_name_for_node, unit_type_promotion};
use crate::semantic::ast_util::span_to_range;
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::{ElementKind, NodeId};

fn semantic_metadata_metaclass_role(display_name: &str, text: &str) -> Option<&'static str> {
    if display_name == "SemanticMetadata" || text.contains("SemanticMetadata") {
        Some("SemanticMetadata")
    } else {
        None
    }
}

pub(super) struct KermlLibraryNodeInput<'a> {
    pub(super) uri: &'a Url,
    pub(super) container_prefix: Option<&'a str>,
    pub(super) parent_id: &'a NodeId,
    pub(super) display_name: String,
    pub(super) bnf_production: &'a str,
    pub(super) text: &'a str,
    pub(super) span: &'a sysml_v2_parser::Span,
}

fn add_kerml_library_decl_node(g: &mut SemanticGraph, input: KermlLibraryNodeInput<'_>) {
    let KermlLibraryNodeInput {
        uri,
        container_prefix,
        parent_id,
        display_name,
        bnf_production,
        text,
        span,
    } = input;
    if bnf_production.eq_ignore_ascii_case("attribute") {
        if let Some(parsed) = unit_type_promotion::try_parse_unit_attribute_def(text) {
            unit_type_promotion::materialize_unit_attribute_def_from_kerml(
                g,
                uri,
                container_prefix,
                parent_id,
                &parsed,
                &span_to_range(span),
            );
            return;
        }
    }
    let metaclass_role = semantic_metadata_metaclass_role(&display_name, text);
    let element_kind = if metaclass_role.is_some() {
        "metadata def"
    } else {
        "kermlDecl"
    };
    let qualified = qualified_name_for_node(g, uri, container_prefix, &display_name, element_kind);
    let mut attrs = HashMap::new();
    attrs.insert(
        "bnfProduction".to_string(),
        serde_json::json!(bnf_production),
    );
    attrs.insert("text".to_string(), serde_json::json!(text));
    if let Some(role) = metaclass_role {
        attrs.insert("metaclassRole".to_string(), serde_json::json!(role));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        element_kind,
        display_name.clone(),
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
    if metaclass_role == Some("SemanticMetadata") {
        let node_id = NodeId::new(uri, &qualified);
        for feature_name in extract_kerml_feature_names_from_text(text) {
            let feature_qualified = qualified_name_for_node(
                g,
                uri,
                Some(node_id.qualified_name.as_str()),
                &feature_name,
                "attribute def",
            );
            add_node_and_recurse(
                g,
                uri,
                &feature_qualified,
                "attribute def",
                feature_name,
                span_to_range(span),
                HashMap::new(),
                Some(&node_id),
            );
        }
    }
}

pub(super) fn add_kerml_library_feature_node(
    g: &mut SemanticGraph,
    input: KermlLibraryNodeInput<'_>,
) {
    let KermlLibraryNodeInput {
        uri,
        container_prefix,
        parent_id,
        display_name,
        bnf_production,
        text,
        span,
    } = input;
    if let Some(parent) = g.get_node(parent_id) {
        if parent.element_kind == ElementKind::MetadataDef
            && parent
                .attributes
                .get("metaclassRole")
                .and_then(|value| value.as_str())
                == Some("SemanticMetadata")
        {
            let qualified = qualified_name_for_node(
                g,
                uri,
                Some(parent_id.qualified_name.as_str()),
                &display_name,
                "attribute def",
            );
            let mut attrs = HashMap::new();
            attrs.insert(
                "bnfProduction".to_string(),
                serde_json::json!(bnf_production),
            );
            attrs.insert("text".to_string(), serde_json::json!(text));
            add_node_and_recurse(
                g,
                uri,
                &qualified,
                "attribute def",
                display_name,
                span_to_range(span),
                attrs,
                Some(parent_id),
            );
            return;
        }
    }
    let qualified = qualified_name_for_node(g, uri, container_prefix, &display_name, "kermlDecl");
    let mut attrs = HashMap::new();
    attrs.insert(
        "bnfProduction".to_string(),
        serde_json::json!(bnf_production),
    );
    attrs.insert("text".to_string(), serde_json::json!(text));
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "kermlDecl",
        display_name,
        span_to_range(span),
        attrs,
        Some(parent_id),
    );
}

pub(super) fn build_kerml_semantic_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    k: &Node<KermlSemanticDecl>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let kv = &k.value;
    let promoted = kv.bnf_production.eq_ignore_ascii_case("attribute")
        && unit_type_promotion::try_parse_unit_attribute_def(&kv.text)
            .map(|parsed| {
                unit_type_promotion::materialize_unit_attribute_def_from_kerml(
                    g,
                    uri,
                    container_prefix,
                    pid,
                    &parsed,
                    &span_to_range(&k.span),
                );
            })
            .is_some();
    if !promoted {
        let display_name =
            extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_kermlSemantic");
        add_kerml_library_decl_node(
            g,
            KermlLibraryNodeInput {
                uri,
                container_prefix,
                parent_id: pid,
                display_name,
                bnf_production: &kv.bnf_production,
                text: &kv.text,
                span: &k.span,
            },
        );
    }
}

pub(super) fn build_kerml_feature_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    k: &Node<KermlFeatureDecl>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let kv = &k.value;
    let display_name = extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_kermlFeature");
    add_kerml_library_feature_node(
        g,
        KermlLibraryNodeInput {
            uri,
            container_prefix,
            parent_id: pid,
            display_name,
            bnf_production: &kv.bnf_production,
            text: &kv.text,
            span: &k.span,
        },
    );
}

pub(super) fn build_extended_library_decl(
    g: &mut SemanticGraph,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    k: &Node<ExtendedLibraryDecl>,
) {
    let Some(pid) = parent_id else {
        return;
    };
    let kv = &k.value;
    let display_name = extract_modeled_decl_name(&kv.bnf_production, &kv.text, "_extendedLibrary");
    add_kerml_library_decl_node(
        g,
        KermlLibraryNodeInput {
            uri,
            container_prefix,
            parent_id: pid,
            display_name,
            bnf_production: &kv.bnf_production,
            text: &kv.text,
            span: &k.span,
        },
    );
}
