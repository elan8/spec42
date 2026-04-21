use std::collections::{BTreeMap, BTreeSet};

use super::types::{
    BlockKind, DeploymentUnit, GenerationModel, LaunchInclude, ManifestDependency, ModelBlock,
    ParameterProfile, RosParameter, TraceEntry,
};

pub(crate) fn extract_generation_model(blocks: &[ModelBlock]) -> GenerationModel {
    let part_bases = blocks
        .iter()
        .filter(|block| block.kind == BlockKind::PartDef)
        .map(|block| (block.name.clone(), block.base.clone()))
        .collect::<BTreeMap<_, _>>();

    let mut package_name = String::new();
    let mut launch_files = BTreeSet::new();
    let mut launch_includes = Vec::new();
    let mut deployments = Vec::new();
    let mut parameter_profiles = Vec::new();
    let mut parameters = Vec::new();
    let mut dependencies = Vec::new();
    let mut artifacts = BTreeSet::new();
    let mut traces = Vec::new();
    let mut message_types = BTreeSet::new();
    let mut service_types = BTreeSet::new();
    let mut action_types = BTreeSet::new();

    for block in blocks {
        if block.kind == BlockKind::PartDef && specializes_part_def(block, "RosPackage", &part_bases) {
            if package_name.is_empty() {
                package_name = attr(block, "name").unwrap_or_default();
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosLaunchDescription", &part_bases)
        {
            if let Some(name) = attr(block, "name") {
                launch_files.insert(launch_file_name(&name));
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosLaunchInclude", &part_bases)
        {
            if let Some(include_ref) = attr(block, "includeRef") {
                launch_includes.push(LaunchInclude {
                    include_ref,
                    condition_expr: attr(block, "conditionExpr"),
                });
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosDeploymentUnit", &part_bases)
        {
            if let (Some(name), Some(package_ref), Some(executable_name)) = (
                attr(block, "name"),
                attr(block, "packageRef"),
                attr(block, "executableName"),
            ) {
                deployments.push(DeploymentUnit {
                    name,
                    package_ref,
                    executable_name,
                });
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosParameterProfile", &part_bases)
        {
            if let Some(file_ref) = attr(block, "parameterFileRef") {
                parameter_profiles.push(ParameterProfile {
                    file_ref,
                    owner_node_ref: attr(block, "ownerNodeRef"),
                });
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosParameter", &part_bases)
        {
            if let Some(name) = attr(block, "name") {
                parameters.push(RosParameter {
                    owner_node_ref: attr(block, "ownerNodeRef"),
                    namespace_ref: attr(block, "namespaceRef"),
                    name,
                    parameter_type: attr(block, "parameterType"),
                    value: parameter_value(block),
                });
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosManifestDependency", &part_bases)
        {
            if let Some(dependency_package_ref) = attr(block, "dependencyPackageRef") {
                dependencies.push(ManifestDependency {
                    owner_package_ref: attr(block, "ownerPackageRef"),
                    dependency_kind: attr(block, "dependencyKind")
                        .unwrap_or_else(|| "exec".to_string()),
                    dependency_package_ref,
                });
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosGenerationArtifact", &part_bases)
        {
            if let Some(path) = attr(block, "artifactPath") {
                artifacts.insert(path);
            }
        }
        if block.kind == BlockKind::PartDef
            && specializes_part_def(block, "RosTraceabilityEntry", &part_bases)
        {
            if let (Some(model_element_id), Some(artifact_path)) =
                (attr(block, "modelElementId"), attr(block, "artifactPath"))
            {
                traces.push(TraceEntry {
                    model_element_id,
                    artifact_path,
                });
            }
        }
        if block.kind == BlockKind::ItemDef && block.base.as_deref() == Some("RosMessageType") {
            if let Some(name) = attr(block, "name") {
                message_types.insert(name);
            }
        }
        if block.kind == BlockKind::ItemDef && block.base.as_deref() == Some("RosServiceType") {
            if let Some(name) = attr(block, "name") {
                service_types.insert(name);
            }
        }
        if block.kind == BlockKind::ItemDef && block.base.as_deref() == Some("RosActionType") {
            if let Some(name) = attr(block, "name") {
                action_types.insert(name);
            }
        }
    }

    traces.sort_by(|a, b| {
        (&a.model_element_id, &a.artifact_path).cmp(&(&b.model_element_id, &b.artifact_path))
    });

    GenerationModel {
        package_name,
        launch_files: launch_files.into_iter().collect(),
        launch_includes,
        deployments,
        parameter_profiles,
        parameters,
        dependencies,
        artifacts: artifacts.into_iter().collect(),
        traces,
        message_types: message_types.into_iter().collect(),
        service_types: service_types.into_iter().collect(),
        action_types: action_types.into_iter().collect(),
    }
}

fn specializes_part_def(
    block: &ModelBlock,
    target_base: &str,
    part_bases: &BTreeMap<String, Option<String>>,
) -> bool {
    let mut current = block.base.clone();
    while let Some(base) = current {
        if base == target_base {
            return true;
        }
        current = part_bases.get(&base).cloned().flatten();
    }
    false
}

fn attr(block: &ModelBlock, name: &str) -> Option<String> {
    block.attrs.get(name).cloned()
}

fn parameter_value(block: &ModelBlock) -> String {
    for key in ["defaultInt", "defaultFloat", "defaultBool", "defaultText"] {
        if let Some(value) = attr(block, key) {
            return value;
        }
    }
    "0".to_string()
}

fn launch_file_name(name: &str) -> String {
    if name.ends_with(".launch.py") {
        name.to_string()
    } else {
        format!("{name}.launch.py")
    }
}
