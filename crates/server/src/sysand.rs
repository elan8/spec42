use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SysandStatus {
    pub installed: bool,
    pub executable_path: Option<String>,
    pub version: Option<String>,
    pub project_root: Option<String>,
    pub manifest_present: bool,
    pub lock_present: bool,
    pub dependency_roots: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn detect_sysand_status() -> SysandStatus {
    let executable = find_sysand_executable();
    let version = executable
        .as_ref()
        .and_then(|path| read_sysand_version(path.as_path()));
    let project_root = discover_project_root();
    detect_sysand_status_from(project_root.as_deref(), executable, version)
}

pub fn dependency_roots_from_status(status: &SysandStatus) -> Vec<PathBuf> {
    status
        .dependency_roots
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>()
}

fn detect_sysand_status_from(
    project_root: Option<&Path>,
    executable: Option<PathBuf>,
    version: Option<String>,
) -> SysandStatus {
    let mut warnings = Vec::new();
    if executable.is_none() {
        warnings.push(
            "Sysand executable was not found on PATH; package dependencies are optional and were not resolved.".to_string(),
        );
    }

    let manifest_present = project_root
        .map(|root| manifest_path(root).is_some())
        .unwrap_or(false);
    let lock_present = project_root
        .map(|root| lock_path(root).is_some())
        .unwrap_or(false);
    let dependency_roots = project_root
        .map(discover_dependency_roots)
        .unwrap_or_default();

    if project_root.is_none() {
        warnings.push(
            "No Sysand project manifest was found from the current workspace upward.".to_string(),
        );
    } else if !manifest_present {
        warnings
            .push("Sysand project root was detected, but no manifest file was found.".to_string());
    }

    let missing = dependency_roots
        .iter()
        .filter(|path| !path.is_dir())
        .map(|path| path.display().to_string())
        .collect::<Vec<_>>();
    if !missing.is_empty() {
        warnings.push(format!(
            "Sysand dependency roots are listed but missing: {}",
            missing.join(", ")
        ));
    }

    SysandStatus {
        installed: executable.is_some(),
        executable_path: executable.map(|path| path.display().to_string()),
        version,
        project_root: project_root.map(|path| path.display().to_string()),
        manifest_present,
        lock_present,
        dependency_roots: dependency_roots
            .into_iter()
            .map(|path| path.display().to_string())
            .collect(),
        warnings,
    }
}

fn discover_project_root() -> Option<PathBuf> {
    if let Ok(raw) = std::env::var("SPEC42_SYSAND_PROJECT_ROOT") {
        let trimmed = raw.trim();
        if !trimmed.is_empty() {
            return Some(canonicalize_lossy(Path::new(trimmed)));
        }
    }
    let cwd = std::env::current_dir().ok()?;
    discover_project_root_from(cwd.as_path())
}

fn discover_project_root_from(start: &Path) -> Option<PathBuf> {
    for ancestor in start.ancestors() {
        if manifest_path(ancestor).is_some() || lock_path(ancestor).is_some() {
            return Some(canonicalize_lossy(ancestor));
        }
    }
    None
}

fn manifest_path(root: &Path) -> Option<PathBuf> {
    ["sysand.toml", "Sysand.toml", "kerml.toml", "Kerml.toml"]
        .iter()
        .map(|name| root.join(name))
        .find(|path| path.is_file())
}

fn lock_path(root: &Path) -> Option<PathBuf> {
    ["sysand.lock", "Sysand.lock", "kerml.lock", "Kerml.lock"]
        .iter()
        .map(|name| root.join(name))
        .find(|path| path.is_file())
}

fn discover_dependency_roots(root: &Path) -> Vec<PathBuf> {
    let mut roots = Vec::new();
    for candidate in [
        root.join(".sysand").join("packages"),
        root.join(".sysand").join("deps"),
        root.join("sysand").join("packages"),
        root.join("sysand").join("deps"),
        root.join(".kerml").join("packages"),
    ] {
        roots.push(candidate);
    }
    if let Some(manifest) = manifest_path(root) {
        roots.extend(parse_dependency_roots_from_toml(root, manifest.as_path()));
    }
    let mut seen = BTreeSet::new();
    roots
        .into_iter()
        .map(|path| canonicalize_lossy(path.as_path()))
        .filter(|path| seen.insert(path.display().to_string()))
        .collect()
}

fn parse_dependency_roots_from_toml(root: &Path, manifest: &Path) -> Vec<PathBuf> {
    let Ok(raw) = std::fs::read_to_string(manifest) else {
        return Vec::new();
    };
    let Ok(value) = raw.parse::<toml::Value>() else {
        return Vec::new();
    };
    let mut out = Vec::new();
    collect_path_values(&value, &mut out);
    out.into_iter()
        .filter(|raw| {
            let lower = raw.to_ascii_lowercase();
            lower.contains("sysml")
                || lower.contains("kerml")
                || lower.contains("package")
                || lower.contains("library")
        })
        .map(|raw| {
            let path = PathBuf::from(raw);
            if path.is_absolute() {
                path
            } else {
                root.join(path)
            }
        })
        .collect()
}

fn collect_path_values(value: &toml::Value, out: &mut Vec<String>) {
    match value {
        toml::Value::String(value) if value.contains('/') || value.contains('\\') => {
            out.push(value.clone());
        }
        toml::Value::Array(values) => {
            for value in values {
                collect_path_values(value, out);
            }
        }
        toml::Value::Table(values) => {
            for value in values.values() {
                collect_path_values(value, out);
            }
        }
        _ => {}
    }
}

fn find_sysand_executable() -> Option<PathBuf> {
    let path_value = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_value) {
        for name in executable_names() {
            let candidate = dir.join(name);
            if candidate.is_file() {
                return Some(candidate);
            }
        }
    }
    None
}

fn executable_names() -> &'static [&'static str] {
    if cfg!(windows) {
        &["sysand.exe", "sysand.cmd", "sysand.bat", "sysand"]
    } else {
        &["sysand"]
    }
}

fn read_sysand_version(executable: &Path) -> Option<String> {
    let output = Command::new(executable).arg("--version").output().ok()?;
    if !output.status.success() {
        return None;
    }
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if stdout.is_empty() {
        None
    } else {
        Some(stdout)
    }
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn absent_sysand_reports_warning_without_failure() {
        let status = detect_sysand_status_from(None, None, None);
        assert!(!status.installed);
        assert!(status.project_root.is_none());
        assert!(status
            .warnings
            .iter()
            .any(|warning| warning.contains("not found")));
    }

    #[test]
    fn manifest_project_reports_dependency_roots() {
        let temp = tempfile::tempdir().expect("temp dir");
        let root = temp.path();
        std::fs::write(
            root.join("sysand.toml"),
            r#"
                [dependencies]
                local = "libraries/sysml"
            "#,
        )
        .expect("write manifest");
        let package_root = root.join(".sysand").join("packages");
        std::fs::create_dir_all(&package_root).expect("create package root");

        let status = detect_sysand_status_from(
            Some(root),
            Some(PathBuf::from("sysand")),
            Some("sysand 0.test".to_string()),
        );

        assert!(status.installed);
        assert!(status.manifest_present);
        assert!(status
            .dependency_roots
            .iter()
            .any(|path| path.ends_with(".sysand\\packages") || path.ends_with(".sysand/packages")));
        assert!(status
            .dependency_roots
            .iter()
            .any(|path| path.ends_with("libraries\\sysml") || path.ends_with("libraries/sysml")));
    }

    #[test]
    fn manifest_without_executable_reports_project_and_install_warning() {
        let temp = tempfile::tempdir().expect("temp dir");
        std::fs::write(
            temp.path().join("sysand.toml"),
            "[package]\nname = \"demo\"",
        )
        .expect("write manifest");

        let status = detect_sysand_status_from(Some(temp.path()), None, None);

        assert!(!status.installed);
        assert!(status.manifest_present);
        assert!(status.project_root.is_some());
        assert!(status
            .warnings
            .iter()
            .any(|warning| warning.contains("not found")));
    }

    #[test]
    fn missing_dependency_roots_are_reported_as_warnings() {
        let temp = tempfile::tempdir().expect("temp dir");
        std::fs::write(
            temp.path().join("sysand.toml"),
            r#"
                [dependencies]
                missing = "missing/library"
            "#,
        )
        .expect("write manifest");

        let status = detect_sysand_status_from(
            Some(temp.path()),
            Some(PathBuf::from("sysand")),
            Some("sysand 0.test".to_string()),
        );

        assert!(status
            .dependency_roots
            .iter()
            .any(|path| path.ends_with("missing\\library") || path.ends_with("missing/library")));
        assert!(status
            .warnings
            .iter()
            .any(|warning| warning.contains("listed but missing")));
    }

    #[test]
    fn lock_file_presence_is_reported() {
        let temp = tempfile::tempdir().expect("temp dir");
        std::fs::write(
            temp.path().join("sysand.toml"),
            "[package]\nname = \"demo\"",
        )
        .expect("write manifest");
        std::fs::write(temp.path().join("sysand.lock"), "# lock").expect("write lock");

        let status = detect_sysand_status_from(
            Some(temp.path()),
            Some(PathBuf::from("sysand")),
            Some("sysand 0.test".to_string()),
        );

        assert!(status.manifest_present);
        assert!(status.lock_present);
    }

    #[test]
    fn discovers_project_root_from_manifest_ancestor() {
        let temp = tempfile::tempdir().expect("temp dir");
        let nested = temp.path().join("a").join("b");
        std::fs::create_dir_all(&nested).expect("create nested");
        std::fs::write(
            temp.path().join("Sysand.toml"),
            "[package]\nname = \"demo\"",
        )
        .expect("write manifest");

        let found = discover_project_root_from(nested.as_path()).expect("project root");
        assert_eq!(found, canonicalize_lossy(temp.path()));
    }
}
