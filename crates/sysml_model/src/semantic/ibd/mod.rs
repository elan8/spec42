//! Internal Block Diagram extraction from the semantic graph.

mod connectors;
mod dto;
mod extract_impl;
mod instance_paths;
mod merge;

pub(crate) use connectors::enrich_connector_endpoint_refs;
pub use connectors::finalize_merged_ibd_connectors;
pub use dto::*;
pub use extract_impl::{build_ibd_for_uri, qualified_name_to_dot};
pub(crate) use extract_impl::{resolve_owner_part_qn_for_endpoint, resolve_port_id_for_endpoint};
pub(crate) use instance_paths::infer_def_instance_scope_mappings_for_ibd;
pub use instance_paths::normalize_ibd_to_instance_paths;
pub use merge::{merge_ibd_payloads, merge_ibd_payloads_for_workspace_finalize};
