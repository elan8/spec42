//! Re-export sysml/model parsing and empty response from spec42-core.
//! The actual response is built by spec42_core::build_sysml_model_response with config.diagram_providers.

pub use spec42_core::sysml_model::{empty_model_response, parse_sysml_model_params};
