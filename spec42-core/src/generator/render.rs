use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use walkdir::WalkDir;

use super::types::{DeploymentUnit, LaunchInclude, ManifestDependency, ParameterProfile, RosParameter, TraceEntry};

pub(crate) fn generate_base_project_files(
    package_dir: &Path,
    package_name: &str,
    dependencies: &[ManifestDependency],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    let deps = dependency_entries(dependencies, package_name);
    let package_xml = render_package_xml(package_name, &deps);
    let setup_py = render_setup_py(package_name);
    let setup_cfg = r#"[develop]
script_dir=$base/lib/{name}
[install]
install_scripts=$base/lib/{name}
"#
    .replace("{name}", package_name);

    write_generated_file(package_dir, "package.xml", &package_xml, dry_run, generated_files)?;
    write_generated_file(package_dir, "setup.py", &setup_py, dry_run, generated_files)?;
    write_generated_file(package_dir, "setup.cfg", &setup_cfg, dry_run, generated_files)?;
    write_generated_file(
        package_dir,
        &format!("resource/{package_name}"),
        "",
        dry_run,
        generated_files,
    )?;
    write_generated_file(
        package_dir,
        &format!("{package_name}/__init__.py"),
        "",
        dry_run,
        generated_files,
    )?;
    Ok(())
}

pub(crate) fn generate_launch_files(
    package_dir: &Path,
    launch_files: &[String],
    launch_includes: &[LaunchInclude],
    deployments: &[DeploymentUnit],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    for file_name in launch_files {
        write_generated_file(
            package_dir,
            &format!("launch/{file_name}"),
            &render_launch_file(launch_includes, deployments),
            dry_run,
            generated_files,
        )?;
    }
    Ok(())
}

pub(crate) fn generate_parameter_files(
    package_dir: &Path,
    profiles: &[ParameterProfile],
    parameters: &[RosParameter],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    let mut grouped: BTreeMap<String, Vec<&RosParameter>> = BTreeMap::new();
    for profile in profiles {
        let mut selected = parameters
            .iter()
            .filter(|param| {
                profile
                    .owner_node_ref
                    .as_deref()
                    .map(|owner| param.owner_node_ref.as_deref() == Some(owner))
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();
        selected.sort_by(|a, b| a.name.cmp(&b.name));
        grouped.insert(profile.file_ref.clone(), selected);
    }

    for (file_ref, entries) in grouped {
        let namespace_hint = entries
            .first()
            .and_then(|entry| entry.namespace_ref.as_deref())
            .unwrap_or("/**");
        let mut yaml_lines = vec![format!("{namespace_hint}:"), "  ros__parameters:".to_string()];
        for param in entries {
            yaml_lines.push(format!("    {}: {}", param.name, yaml_scalar(param)));
        }
        if yaml_lines.len() == 2 {
            yaml_lines.push("    generated: true".to_string());
        }
        write_generated_file(
            package_dir,
            &normalize_relative_path(&file_ref),
            &yaml_lines.join("\n"),
            dry_run,
            generated_files,
        )?;
    }
    Ok(())
}

pub(crate) fn generate_interface_files(
    package_dir: &Path,
    message_types: &[String],
    service_types: &[String],
    action_types: &[String],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    for item in message_types {
        if let Some((pkg, ty)) = split_ros_type(item) {
            if pkg.starts_with("inspection") {
                write_generated_file(
                    package_dir,
                    &format!("interfaces/msg/{ty}.msg"),
                    &interface_content_from_stem(ty, "msg"),
                    dry_run,
                    generated_files,
                )?;
            }
        }
    }
    for item in service_types {
        if let Some((pkg, ty)) = split_ros_type(item) {
            if pkg.starts_with("inspection") {
                write_generated_file(
                    package_dir,
                    &format!("interfaces/srv/{ty}.srv"),
                    &interface_content_from_stem(ty, "srv"),
                    dry_run,
                    generated_files,
                )?;
            }
        }
    }
    for item in action_types {
        if let Some((pkg, ty)) = split_ros_type(item) {
            if pkg.starts_with("inspection") {
                write_generated_file(
                    package_dir,
                    &format!("interfaces/action/{ty}.action"),
                    &interface_content_from_stem(ty, "action"),
                    dry_run,
                    generated_files,
                )?;
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn materialize_artifact_files(
    package_dir: &Path,
    package_name: &str,
    launch_includes: &[LaunchInclude],
    deployments: &[DeploymentUnit],
    artifacts: &[String],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
    warnings: &mut Vec<String>,
) -> Result<(), String> {
    for artifact in artifacts {
        let relative = normalize_artifact_path(artifact, package_name);
        if generated_files.contains(&relative) {
            continue;
        }
        let content = artifact_content(&relative, package_name, launch_includes, deployments);
        if content.starts_with("# Generated placeholder") {
            warnings.push(format!("Created placeholder for modeled artifact `{relative}`."));
        }
        write_generated_file(
            package_dir,
            &relative,
            &content,
            dry_run,
            generated_files,
        )?;
    }
    Ok(())
}

pub(crate) fn write_traceability(
    package_dir: &Path,
    traces: &[TraceEntry],
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<Option<PathBuf>, String> {
    let traceability_path = PathBuf::from("traceability.json");
    let content = serde_json::to_string_pretty(traces)
        .map_err(|err| format!("Failed to serialize traceability output: {err}"))?;
    write_generated_file(
        package_dir,
        traceability_path.to_str().unwrap_or("traceability.json"),
        &content,
        dry_run,
        generated_files,
    )?;
    Ok(Some(package_dir.join(traceability_path)))
}

pub(crate) fn canonicalize_existing(path: &Path) -> Result<PathBuf, String> {
    path.canonicalize()
        .map_err(|err| format!("Failed to resolve {}: {err}", path.display()))
}

pub(crate) fn normalize_output_root(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return path
            .canonicalize()
            .map_err(|err| format!("Failed to resolve {}: {err}", path.display()));
    }
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)
                .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
        }
    }
    Ok(path.to_path_buf())
}

pub(crate) fn collect_sysml_files(input: &Path) -> Result<Vec<PathBuf>, String> {
    let mut files = BTreeSet::new();
    if input.is_file() {
        if is_sysml_file(input) {
            files.insert(input.to_path_buf());
        }
        return Ok(files.into_iter().collect());
    }
    for entry in WalkDir::new(input).into_iter().filter_map(Result::ok) {
        if entry.file_type().is_file() && is_sysml_file(entry.path()) {
            files.insert(entry.path().to_path_buf());
        }
    }
    Ok(files.into_iter().collect())
}

fn dependency_entries(
    dependencies: &[ManifestDependency],
    package_name: &str,
) -> BTreeSet<(String, String)> {
    dependencies
        .iter()
        .filter(|dep| {
            dep.owner_package_ref
                .as_deref()
                .map(|owner| owner == package_name)
                .unwrap_or(true)
        })
        .map(|dep| (dep.dependency_kind.clone(), dep.dependency_package_ref.clone()))
        .collect()
}

fn render_package_xml(package_name: &str, dependencies: &BTreeSet<(String, String)>) -> String {
    let mut lines = vec![
        r#"<?xml version="1.0"?>"#.to_string(),
        r#"<package format="3">"#.to_string(),
        format!("  <name>{package_name}</name>"),
        "  <version>0.0.1</version>".to_string(),
        "  <description>Generated by spec42 generate ros2.</description>".to_string(),
        "  <maintainer email=\"todo@example.com\">spec42</maintainer>".to_string(),
        "  <license>Apache-2.0</license>".to_string(),
        "  <buildtool_depend>ament_python</buildtool_depend>".to_string(),
    ];
    for (kind, dep) in dependencies {
        let tag = match kind.as_str() {
            "build" => "build_depend",
            "test" => "test_depend",
            _ => "exec_depend",
        };
        lines.push(format!("  <{tag}>{dep}</{tag}>"));
    }
    lines.push("  <test_depend>ament_lint_auto</test_depend>".to_string());
    lines.push("  <test_depend>ament_lint_common</test_depend>".to_string());
    lines.push("  <export>".to_string());
    lines.push("    <build_type>ament_python</build_type>".to_string());
    lines.push("  </export>".to_string());
    lines.push("</package>".to_string());
    lines.join("\n")
}

fn render_setup_py(package_name: &str) -> String {
    format!(
        r#"from setuptools import find_packages, setup

package_name = "{package_name}"

setup(
    name=package_name,
    version="0.0.1",
    packages=find_packages(exclude=["test"]),
    data_files=[
        ("share/ament_index/resource_index/packages", ["resource/" + package_name]),
        ("share/" + package_name, ["package.xml"]),
    ],
    install_requires=["setuptools"],
    zip_safe=True,
    maintainer="spec42",
    maintainer_email="todo@example.com",
    description="Generated by spec42 generate ros2",
    license="Apache-2.0",
    tests_require=["pytest"],
    entry_points={{"console_scripts": []}},
)
"#
    )
}

fn yaml_scalar(param: &RosParameter) -> String {
    let value = param.value.trim();
    match param.parameter_type.as_deref() {
        Some("int") | Some("integer") | Some("float") | Some("double") | Some("bool") => {
            value.to_string()
        }
        _ => format!("\"{}\"", value.replace('"', "\\\"")),
    }
}

fn render_launch_file(launch_includes: &[LaunchInclude], deployments: &[DeploymentUnit]) -> String {
    let mut lines = vec![
        "from launch import LaunchDescription".to_string(),
        "from launch.actions import DeclareLaunchArgument, IncludeLaunchDescription".to_string(),
        "from launch.conditions import IfCondition, UnlessCondition".to_string(),
        "from launch.launch_description_sources import PythonLaunchDescriptionSource".to_string(),
        "from launch.substitutions import LaunchConfiguration, PathJoinSubstitution".to_string(),
        "from launch_ros.actions import Node".to_string(),
        "from launch_ros.substitutions import FindPackageShare".to_string(),
        "".to_string(),
        "def generate_launch_description():".to_string(),
    ];

    let mut condition_vars = BTreeSet::new();
    for include in launch_includes {
        if let Some((var, _negated)) = parse_condition_variable(include.condition_expr.as_deref()) {
            condition_vars.insert(var);
        }
    }
    for var in condition_vars {
        lines.push(format!(
            "    {var}_arg = DeclareLaunchArgument(\"{var}\", default_value=\"false\")"
        ));
    }

    lines.push("    actions = []".to_string());
    if !launch_includes.is_empty() || !deployments.is_empty() {
        lines.push(String::new());
    }

    for include in launch_includes {
        if let Some((pkg, launch_file)) = split_include_ref(&include.include_ref) {
            let include_source = format!(
                "PythonLaunchDescriptionSource([PathJoinSubstitution([FindPackageShare(\"{pkg}\"), \"launch\", \"{launch_file}\"])])"
            );
            if let Some(condition_expr) = render_condition_expr(include.condition_expr.as_deref()) {
                lines.push(format!(
                    "    actions.append(IncludeLaunchDescription({include_source}, condition={condition_expr}))"
                ));
            } else {
                lines.push(format!(
                    "    actions.append(IncludeLaunchDescription({include_source}))"
                ));
            }
        }
    }

    let mut sorted_deployments = deployments.to_vec();
    sorted_deployments.sort_by(|a, b| a.name.cmp(&b.name));
    for deployment in sorted_deployments {
        lines.push(format!(
            "    actions.append(Node(package=\"{}\", executable=\"{}\", name=\"{}\", output=\"screen\"))",
            deployment.package_ref, deployment.executable_name, deployment.name
        ));
    }

    lines.push("    return LaunchDescription(actions)".to_string());
    lines.join("\n")
}

fn split_include_ref(include_ref: &str) -> Option<(&str, &str)> {
    let (pkg, rest) = include_ref.split_once('/')?;
    let launch_file = rest.trim_start_matches("launch/");
    Some((pkg, launch_file))
}

fn parse_condition_variable(expr: Option<&str>) -> Option<(String, bool)> {
    let expr = expr?.trim();
    if expr.eq_ignore_ascii_case("true") || expr.is_empty() {
        return None;
    }
    if let Some(stripped) = expr.strip_prefix("not ") {
        return Some((stripped.trim().to_string(), true));
    }
    Some((expr.to_string(), false))
}

fn render_condition_expr(expr: Option<&str>) -> Option<String> {
    let (var, negated) = parse_condition_variable(expr)?;
    if negated {
        Some(format!("UnlessCondition(LaunchConfiguration(\"{var}\"))"))
    } else {
        Some(format!("IfCondition(LaunchConfiguration(\"{var}\"))"))
    }
}

fn artifact_content(
    relative: &str,
    package_name: &str,
    launch_includes: &[LaunchInclude],
    deployments: &[DeploymentUnit],
) -> String {
    let normalized = normalize_relative_path(relative);
    if normalized.ends_with(".launch.py") {
        return render_launch_file(launch_includes, deployments);
    }
    if normalized.ends_with(".yaml") {
        return "/**:\n  ros__parameters:\n    generated_from_artifact: true\n".to_string();
    }
    if normalized.ends_with(".msg") {
        let stem = Path::new(&normalized)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Message");
        return interface_content_from_stem(stem, "msg");
    }
    if normalized.ends_with(".srv") {
        let stem = Path::new(&normalized)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Service");
        return interface_content_from_stem(stem, "srv");
    }
    if normalized.ends_with(".action") {
        let stem = Path::new(&normalized)
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("Action");
        return interface_content_from_stem(stem, "action");
    }
    if normalized.ends_with("CMakeLists.txt") {
        return format!(
            "cmake_minimum_required(VERSION 3.8)\nproject({package_name})\n\n# ament_python package scaffold; CMake not required for core build.\n"
        );
    }
    if normalized.ends_with("permissions.xml") || normalized.ends_with(".policy.xml") {
        return "<?xml version=\"1.0\"?>\n<policy version=\"0.2.0\">\n  <profiles />\n</policy>\n"
            .to_string();
    }
    "# Generated placeholder from RosGenerationArtifact\n".to_string()
}

fn interface_content_from_stem(stem: &str, kind: &str) -> String {
    match (stem, kind) {
        ("MissionTarget", "msg") => {
            "float64 latitude\nfloat64 longitude\nfloat64 altitude\nstring frame_id\n".to_string()
        }
        ("LaserScan", "msg") => {
            "float32 angle_min\nfloat32 angle_max\nfloat32 angle_increment\nfloat32 range_min\nfloat32 range_max\nfloat32[] ranges\n".to_string()
        }
        ("SetRuntimeMode", "srv") => {
            "string mode\n---\nbool accepted\nstring message\n".to_string()
        }
        ("NavigateToPoint", "action") => {
            "float64 latitude\nfloat64 longitude\n---\nbool success\nstring result_message\n---\nfloat32 progress_pct\n".to_string()
        }
        (_, "msg") => "string data\n".to_string(),
        (_, "srv") => "string request\n---\nstring response\n".to_string(),
        (_, "action") => "string goal\n---\nstring result\n---\nstring feedback\n".to_string(),
        _ => "# Generated placeholder\n".to_string(),
    }
}

fn write_generated_file(
    package_dir: &Path,
    relative: &str,
    content: &str,
    dry_run: bool,
    generated_files: &mut BTreeSet<String>,
) -> Result<(), String> {
    let relative = normalize_relative_path(relative);
    generated_files.insert(relative.clone());
    if dry_run {
        return Ok(());
    }
    let target = package_dir.join(&relative);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)
            .map_err(|err| format!("Failed to create {}: {err}", parent.display()))?;
    }
    fs::write(&target, content).map_err(|err| format!("Failed to write {}: {err}", target.display()))
}

fn split_ros_type(name: &str) -> Option<(&str, &str)> {
    name.split_once('/')
}

fn normalize_relative_path(value: &str) -> String {
    value
        .replace('\\', "/")
        .trim_start_matches("./")
        .trim_start_matches('/')
        .to_string()
}

fn normalize_artifact_path(value: &str, package_name: &str) -> String {
    let normalized = normalize_relative_path(value);
    let package_prefix = format!("{package_name}/");
    if normalized.starts_with(&package_prefix) {
        normalized.trim_start_matches(&package_prefix).to_string()
    } else {
        normalized
    }
}

fn is_sysml_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            ext == "sysml" || ext == "kerml"
        })
        .unwrap_or(false)
}
