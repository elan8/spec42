//! Shared host workspace snapshot loading for CLI, HTTP, and MCP surfaces.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use lsp_server::{BuiltWorkspaceInput, SemanticValidationReport, ValidationRequest};
use workspace::{
    HostContext, HostFilesystemProvider, HostWorkspaceSnapshot, Spec42Engine, WorkspaceLoadRequest,
};

use crate::cli::{CheckArgs, Cli};
use crate::environment::{build_engine, ResolvedEnvironment};

pub fn built_workspace_input_from_snapshot(
    snapshot: &HostWorkspaceSnapshot,
) -> BuiltWorkspaceInput {
    BuiltWorkspaceInput {
        semantic_graph: snapshot.semantic_graph().clone(),
        parsed_documents: snapshot.parsed_documents().to_vec(),
        library_urls: snapshot.library_urls().to_vec(),
        workspace_root: Some(snapshot.workspace_root().to_path_buf()),
    }
}

pub fn load_snapshot_for_check(
    cli: &Cli,
    args: &CheckArgs,
) -> Result<Arc<HostWorkspaceSnapshot>, String> {
    let engine = build_engine(cli)?;
    load_snapshot_with_engine(
        &engine,
        &args.path,
        args.workspace_root.as_deref(),
        args.strict_diagnostics,
    )
}

pub fn load_snapshot_for_paths(
    cli: &Cli,
    path: &Path,
    workspace_root: Option<&Path>,
    strict_diagnostics: bool,
) -> Result<Arc<HostWorkspaceSnapshot>, String> {
    let engine = build_engine(cli)?;
    load_snapshot_with_engine(&engine, path, workspace_root, strict_diagnostics)
}

pub fn load_snapshot_with_engine(
    engine: &Spec42Engine,
    path: &Path,
    workspace_root: Option<&Path>,
    strict_diagnostics: bool,
) -> Result<Arc<HostWorkspaceSnapshot>, String> {
    let provider = HostFilesystemProvider::from_paths(
        path,
        workspace_root,
        engine.package_roots(),
    );
    let request = WorkspaceLoadRequest::single_target(path.to_path_buf())
        .with_workspace_root(workspace_root.map(Path::to_path_buf))
        .with_strict_diagnostics(strict_diagnostics);
    engine
        .load_workspace(provider, request, HostContext::default())
        .map_err(|error| error.to_string())
}

pub fn semantic_report_from_snapshot(
    snapshot: &HostWorkspaceSnapshot,
    _environment: &ResolvedEnvironment,
    request: ValidationRequest,
) -> Result<SemanticValidationReport, String> {
    let config = Arc::new(lsp_server::default_server_config());
    let built = built_workspace_input_from_snapshot(snapshot);
    lsp_server::semantic_report_from_built_workspace(&config, &built, request)
        .map_err(|error| error.to_string())
}

pub fn load_snapshot_for_validation_request(
    cli: &Cli,
    environment: &ResolvedEnvironment,
    path: PathBuf,
    workspace_root: Option<PathBuf>,
    strict_diagnostics: bool,
) -> Result<Arc<HostWorkspaceSnapshot>, String> {
    let _ = environment;
    load_snapshot_for_paths(cli, &path, workspace_root.as_deref(), strict_diagnostics)
}
