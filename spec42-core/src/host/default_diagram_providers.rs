//! Default server configuration without built-in diagram renderers.
//!
//! Diagram providers remain pluggable via `Spec42Config`, but no renderer ships in-tree.

pub fn default_config() -> crate::Spec42Config {
    crate::Spec42Config::new()
        .with_check_provider(std::sync::Arc::new(crate::DefaultSemanticChecks))
}
