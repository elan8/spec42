use std::net::SocketAddr;
use std::path::PathBuf;

use clap::Args;

#[derive(Debug, Clone, Args)]
pub struct ApiServeArgs {
    /// Workspace root directory to serve (required).
    #[arg(long = "workspace-root")]
    pub workspace_root: PathBuf,
    /// Socket address to bind (default 127.0.0.1:3842).
    #[arg(long = "bind", default_value = "127.0.0.1:3842")]
    pub bind: SocketAddr,
    /// Allow binding to non-loopback addresses (off by default).
    #[arg(long = "allow-remote", default_value_t = false)]
    pub allow_remote: bool,
    /// Allowed CORS origins (repeatable). Disabled when omitted.
    #[arg(long = "cors-origins")]
    pub cors_origins: Vec<String>,
}
