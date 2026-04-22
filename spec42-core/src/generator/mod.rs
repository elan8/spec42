mod extract;
mod parse;
mod render;
mod types;

use std::collections::BTreeSet;
use std::sync::Arc;

use crate::host::config::Spec42Config;
use crate::validation::{validate_paths, ValidationRequest};

pub use types::{Ros2GenerationReport, Ros2GenerationRequest};

pub fn generate_ros2_project(request: Ros2GenerationRequest) -> Result<Ros2GenerationReport, String> {
    let normalized_input = render::canonicalize_existing(&request.input)?;
    let output_root = render::normalize_output_root(&request.output)?;

    let config = Arc::new(Spec42Config::default());
    let validation = validate_paths(
        &config,
        ValidationRequest {
            targets: vec![normalized_input.clone()],
            workspace_root: request.workspace_root.clone(),
            library_paths: request.library_paths.clone(),
            parallel_enabled: true,
        },
    )?;
    if validation.summary.error_count > 0 {
        return Err(format!(
            "Generation preflight failed with {} error(s) and {} warning(s).",
            validation.summary.error_count, validation.summary.warning_count
        ));
    }

    let source_paths = render::collect_sysml_files(&normalized_input)?;
    let model_blocks = parse::parse_model_blocks(&source_paths)?;
    let mut model = extract::extract_generation_model(&model_blocks);
    if let Some(package_override) = request.package_name.as_ref() {
        model.package_name = package_override.clone();
    }
    if model.package_name.is_empty() {
        model.package_name = "spec42_ros2_project".to_string();
    }

    let package_dir = output_root.join(&model.package_name);
    if package_dir.exists() && !request.force && !request.dry_run {
        return Err(format!(
            "Output directory already exists: {} (use --force to overwrite).",
            package_dir.display()
        ));
    }
    if package_dir.exists() && request.force && !request.dry_run {
        std::fs::remove_dir_all(&package_dir)
            .map_err(|err| format!("Failed to clear {}: {err}", package_dir.display()))?;
    }

    let mut generated_files = BTreeSet::new();
    let mut warnings = Vec::new();

    render::generate_base_project_files(
        &package_dir,
        &model.package_name,
        &model.dependencies,
        request.dry_run,
        &mut generated_files,
    )?;
    render::generate_launch_files(
        &package_dir,
        &model.launch_files,
        &model.launch_includes,
        &model.deployments,
        request.dry_run,
        &mut generated_files,
    )?;
    render::generate_parameter_files(
        &package_dir,
        &model.parameter_profiles,
        &model.parameters,
        request.dry_run,
        &mut generated_files,
    )?;
    render::generate_interface_files(
        &package_dir,
        &model.message_types,
        &model.service_types,
        &model.action_types,
        request.dry_run,
        &mut generated_files,
    )?;
    render::materialize_artifact_files(
        &package_dir,
        &model.package_name,
        &model.launch_includes,
        &model.deployments,
        &model.artifacts,
        request.dry_run,
        &mut generated_files,
        &mut warnings,
    )?;
    let traceability_path =
        render::write_traceability(&package_dir, &model.traces, request.dry_run, &mut generated_files)?;

    Ok(Ros2GenerationReport {
        package_name: model.package_name,
        package_dir: package_dir.display().to_string(),
        generated_files: generated_files.into_iter().collect(),
        traceability_path: traceability_path.map(|path| path.display().to_string()),
        validation_summary: validation.summary,
        warnings,
        dry_run: request.dry_run,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn generate_ros2_project_dry_run_for_rover_fixture() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = root
            .join("../domain-libraries/business/robotics/examples/inspection-rover/inspection-rover.sysml");
        let output = tempdir().expect("tempdir");
        let report = generate_ros2_project(Ros2GenerationRequest {
            input: fixture,
            output: output.path().to_path_buf(),
            package_name: Some("inspection_rover_bringup".to_string()),
            workspace_root: Some(root.join("..")),
            library_paths: vec![root
                .join("../domain-libraries/business/robotics")
                .canonicalize()
                .unwrap()],
            force: false,
            dry_run: true,
        })
        .expect("generation should succeed");
        assert_eq!(report.package_name, "inspection_rover_bringup");
        assert!(report.generated_files.iter().any(|path| path == "package.xml"));
        assert!(
            report
                .generated_files
                .iter()
                .any(|path| path == "launch/inspection_rover.launch.py")
        );
        assert!(
            report
                .generated_files
                .iter()
                .any(|path| path == "traceability.json")
        );
    }

    #[test]
    fn generate_ros2_project_writes_scaffold_files() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let fixture = root
            .join("../domain-libraries/business/robotics/examples/inspection-rover/inspection-rover.sysml");
        let output = tempdir().expect("tempdir");
        let report = generate_ros2_project(Ros2GenerationRequest {
            input: fixture,
            output: output.path().to_path_buf(),
            package_name: Some("inspection_rover_bringup".to_string()),
            workspace_root: Some(root.join("..")),
            library_paths: vec![root
                .join("../domain-libraries/business/robotics")
                .canonicalize()
                .unwrap()],
            force: false,
            dry_run: false,
        })
        .expect("generation should succeed");
        let package_dir = PathBuf::from(&report.package_dir);
        assert!(package_dir.join("package.xml").exists());
        assert!(
            package_dir
                .join("launch/inspection_rover.launch.py")
                .exists()
        );
        let package_xml = fs::read_to_string(package_dir.join("package.xml")).expect("package.xml");
        let launch_file = fs::read_to_string(package_dir.join("launch/inspection_rover.launch.py"))
            .expect("launch file");
        let service_file = fs::read_to_string(package_dir.join("interfaces/srv/SetRuntimeMode.srv"))
            .expect("service file");
        let action_file =
            fs::read_to_string(package_dir.join("interfaces/action/NavigateToPoint.action"))
                .expect("action file");
        assert!(package_xml.contains("<build_type>ament_python</build_type>"));
        assert!(launch_file.contains("IncludeLaunchDescription("));
        assert!(launch_file.contains("Node("));
        assert!(!service_file.contains("# TODO"));
        assert!(!action_file.contains("# TODO"));
        assert!(package_dir.join("traceability.json").exists());
    }
}
