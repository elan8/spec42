use std::collections::HashMap;

use sysml_v2_parser::ast::{PackageBody, PackageBodyElement};
use sysml_v2_parser::Node;
use sysml_v2_parser::RootNamespace;
use tower_lsp::lsp_types::Url;

use crate::semantic::ast_util::{identification_name, span_to_range};
use crate::semantic::graph::SemanticGraph;
use crate::semantic::model::NodeId;

use super::{add_node_and_recurse, qualified_name_for_node};

pub(super) fn build_nested_package(
    pkg_node: &Node<sysml_v2_parser::ast::Package>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
    recurse: impl Fn(
        &Node<PackageBodyElement>,
        &Url,
        Option<&str>,
        Option<&NodeId>,
        &RootNamespace,
        &mut SemanticGraph,
    ),
) {
    build_package_like(
        identification_name(&pkg_node.identification),
        &pkg_node.body,
        false,
        uri,
        container_prefix,
        parent_id,
        root,
        g,
        recurse,
        span_to_range(&pkg_node.span),
    );
}

pub(super) fn build_nested_library_package(
    pkg_node: &Node<sysml_v2_parser::ast::LibraryPackage>,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
    recurse: impl Fn(
        &Node<PackageBodyElement>,
        &Url,
        Option<&str>,
        Option<&NodeId>,
        &RootNamespace,
        &mut SemanticGraph,
    ),
) {
    build_package_like(
        identification_name(&pkg_node.identification),
        &pkg_node.body,
        pkg_node.is_standard,
        uri,
        container_prefix,
        parent_id,
        root,
        g,
        recurse,
        span_to_range(&pkg_node.span),
    );
}

#[allow(clippy::too_many_arguments)]
fn build_package_like(
    name: String,
    body: &PackageBody,
    is_standard_library: bool,
    uri: &Url,
    container_prefix: Option<&str>,
    parent_id: Option<&NodeId>,
    root: &RootNamespace,
    g: &mut SemanticGraph,
    recurse: impl Fn(
        &Node<PackageBodyElement>,
        &Url,
        Option<&str>,
        Option<&NodeId>,
        &RootNamespace,
        &mut SemanticGraph,
    ),
    range: tower_lsp::lsp_types::Range,
) {
    let name_display = if name.is_empty() {
        "(top level)"
    } else {
        name.as_str()
    };
    let qualified = qualified_name_for_node(g, uri, container_prefix, name_display, "package");
    let node_id = NodeId::new(uri, &qualified);
    let mut attrs = HashMap::new();
    if is_standard_library {
        attrs.insert("isStandardLibrary".to_string(), serde_json::json!(true));
    }
    add_node_and_recurse(
        g,
        uri,
        &qualified,
        "package",
        name_display.to_string(),
        range,
        attrs,
        parent_id,
    );
    let prefix = if name.is_empty() {
        container_prefix.map(str::to_string)
    } else {
        Some(qualified.clone())
    };
    if let PackageBody::Brace { elements } = body {
        for child in elements {
            recurse(child, uri, prefix.as_deref(), Some(&node_id), root, g);
        }
    }
}
