use tower_lsp::lsp_types::Url;

use crate::graph::SemanticGraph;

pub(super) fn relationships_from_part_def(
    _pd_node: &sysml_v2_parser::PartDef,
    _uri: &Url,
    _container_prefix: Option<&str>,
    _qualified: &str,
    _g: &mut SemanticGraph,
) {
    // Specializes edge added in build_from_package_body_element for PartDef
}
