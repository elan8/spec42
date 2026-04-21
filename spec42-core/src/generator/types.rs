use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::Serialize;

use crate::validation::ValidationSummary;

#[derive(Debug, Clone)]
pub struct Ros2GenerationRequest {
    pub input: PathBuf,
    pub output: PathBuf,
    pub package_name: Option<String>,
    pub workspace_root: Option<PathBuf>,
    pub library_paths: Vec<PathBuf>,
    pub force: bool,
    pub dry_run: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Ros2GenerationReport {
    pub package_name: String,
    pub package_dir: String,
    pub generated_files: Vec<String>,
    pub traceability_path: Option<String>,
    pub validation_summary: ValidationSummary,
    pub warnings: Vec<String>,
    pub dry_run: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct ModelBlock {
    pub(crate) kind: BlockKind,
    pub(crate) name: String,
    pub(crate) base: Option<String>,
    pub(crate) attrs: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BlockKind {
    PartDef,
    ItemDef,
}

#[derive(Debug, Clone)]
pub(crate) struct GenerationModel {
    pub(crate) package_name: String,
    pub(crate) launch_files: Vec<String>,
    pub(crate) launch_includes: Vec<LaunchInclude>,
    pub(crate) deployments: Vec<DeploymentUnit>,
    pub(crate) parameter_profiles: Vec<ParameterProfile>,
    pub(crate) parameters: Vec<RosParameter>,
    pub(crate) dependencies: Vec<ManifestDependency>,
    pub(crate) artifacts: Vec<String>,
    pub(crate) traces: Vec<TraceEntry>,
    pub(crate) message_types: Vec<String>,
    pub(crate) service_types: Vec<String>,
    pub(crate) action_types: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct ParameterProfile {
    pub(crate) file_ref: String,
    pub(crate) owner_node_ref: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct LaunchInclude {
    pub(crate) include_ref: String,
    pub(crate) condition_expr: Option<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct DeploymentUnit {
    pub(crate) name: String,
    pub(crate) package_ref: String,
    pub(crate) executable_name: String,
}

#[derive(Debug, Clone)]
pub(crate) struct RosParameter {
    pub(crate) owner_node_ref: Option<String>,
    pub(crate) namespace_ref: Option<String>,
    pub(crate) name: String,
    pub(crate) parameter_type: Option<String>,
    pub(crate) value: String,
}

#[derive(Debug, Clone)]
pub(crate) struct ManifestDependency {
    pub(crate) owner_package_ref: Option<String>,
    pub(crate) dependency_kind: String,
    pub(crate) dependency_package_ref: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct TraceEntry {
    pub(crate) model_element_id: String,
    pub(crate) artifact_path: String,
}
