mod args;
mod error;
mod handlers;
mod paths;
mod types;

pub use args::ApiServeArgs;
pub use error::{ApiError, ApiResult};
pub use types::*;

use std::net::IpAddr;
use std::path::PathBuf;
use std::sync::Arc;

use axum::routing::{get, post};
use axum::Router;
use kernel::Spec42Config;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::api::handlers::{
    diagnostic_codes, diagrams_export, doctor, element_by_name, elements, explain_diagnostic,
    health, meta, model_projection, model_summary, openapi_json, ready, validate,
};
use crate::cli::Cli;
use crate::environment::{resolve_environment, ResolvedEnvironment};

pub struct ApiServerState {
    pub cli: Cli,
    pub workspace_root: PathBuf,
    pub environment: ResolvedEnvironment,
    pub config: Arc<Spec42Config>,
}

pub fn router(state: Arc<ApiServerState>) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/ready", get(ready))
        .route("/openapi.json", get(openapi_json))
        .route("/v1/meta", get(meta))
        .route("/v1/doctor", get(doctor))
        .route("/v1/validate", post(validate))
        .route("/v1/model/summary", post(model_summary))
        .route("/v1/model/projection", post(model_projection))
        .route("/v1/elements", get(elements))
        .route("/v1/elements/{qualified_name}", get(element_by_name))
        .route("/v1/diagnostics/codes", get(diagnostic_codes))
        .route("/v1/diagnostics/explain/{code}", get(explain_diagnostic))
        .route("/v1/diagrams/export", post(diagrams_export))
        .with_state(state)
}

pub async fn run_api_serve(cli: Cli, args: ApiServeArgs) -> Result<(), String> {
    validate_bind_address(&args)?;

    let workspace_root = args
        .workspace_root
        .canonicalize()
        .map_err(|err| {
            format!(
                "Workspace root does not exist or is not accessible: {} ({err})",
                args.workspace_root.display()
            )
        })?;
    if !workspace_root.is_dir() {
        return Err(format!(
            "Workspace root is not a directory: {}",
            workspace_root.display()
        ));
    }

    let environment = resolve_environment(&cli)?;
    let config = Arc::new(
        kernel::default_server_config().with_default_library_paths(environment.library_paths.clone()),
    );

    let mut router = router(Arc::new(ApiServerState {
        cli,
        workspace_root: workspace_root.clone(),
        environment,
        config,
    }));
    router = router.layer(TraceLayer::new_for_http());
    if !args.cors_origins.is_empty() {
        let origins: Vec<_> = args
            .cors_origins
            .iter()
            .filter_map(|origin| origin.parse().ok())
            .collect();
        let cors = CorsLayer::new()
            .allow_methods(Any)
            .allow_headers(Any)
            .allow_origin(AllowOrigin::list(origins));
        router = router.layer(cors);
    }

    let listener = tokio::net::TcpListener::bind(args.bind)
        .await
        .map_err(|err| format!("Failed to bind {}: {err}", args.bind))?;

    eprintln!(
        "spec42 api serving workspace {} at http://{}",
        workspace_root.display(),
        args.bind
    );

    axum::serve(listener, router)
        .await
        .map_err(|err| format!("HTTP server failed: {err}"))
}

fn validate_bind_address(args: &ApiServeArgs) -> Result<(), String> {
    if args.allow_remote {
        return Ok(());
    }
    if is_loopback(args.bind.ip()) {
        return Ok(());
    }
    Err(format!(
        "Refusing to bind to non-loopback address {} without --allow-remote",
        args.bind
    ))
}

fn is_loopback(ip: IpAddr) -> bool {
    match ip {
        IpAddr::V4(v4) => v4.is_loopback(),
        IpAddr::V6(v6) => v6.is_loopback(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bind_validation_rejects_public_without_flag() {
        let args = ApiServeArgs {
            workspace_root: PathBuf::from("."),
            bind: "0.0.0.0:3842".parse().expect("addr"),
            allow_remote: false,
            cors_origins: Vec::new(),
        };
        assert!(validate_bind_address(&args).is_err());
    }

    #[test]
    fn bind_validation_allows_loopback() {
        let args = ApiServeArgs {
            workspace_root: PathBuf::from("."),
            bind: "127.0.0.1:3842".parse().expect("addr"),
            allow_remote: false,
            cors_origins: Vec::new(),
        };
        assert!(validate_bind_address(&args).is_ok());
    }
}
