//! Internal Block Diagram extraction from the semantic graph.

mod dto;
mod extract_impl;

pub use dto::*;
pub use extract_impl::{
    build_ibd_for_uri, finalize_merged_ibd_connectors, is_port_like, merge_ibd_payloads,
    normalize_ibd_to_instance_paths, qualified_name_to_dot,
};
pub(crate) use extract_impl::{
    enrich_connector_endpoint_refs, infer_def_instance_scope_mappings_for_ibd, is_part_like,
    resolve_owner_part_qn_for_endpoint, resolve_port_id_for_endpoint,
};
