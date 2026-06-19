//! Visualization pipeline helpers (payload shaping, scoping, artifacts).

pub mod payload;
pub mod scope;

pub use payload::{
    finalize_activity_diagrams_for_response, finalize_sequence_diagrams_for_response,
    finalize_state_machines_for_response, warn_if_behavior_payload_missing,
};
pub use scope::{ibd_uri_closure_for_exposed_ids, IbdBuildScope};
