//! Default server configuration for the OSS host.

pub fn default_config() -> crate::Spec42Config {
    crate::Spec42Config::new()
        .with_check_provider(std::sync::Arc::new(crate::DefaultSemanticChecks))
}
