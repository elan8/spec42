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

fn detect_sysand_status_from(
    project_root: Option<&Path>,
    executable: Option<PathBuf>,
    version: Option<String>,
) -> SysandStatus {
    let mut warnings = Vec::new();
    if executable.is_none() {
        warnings.push(
            "Sysand executable was not found on PATH; existing installed dependencies can still be loaded, but package installation must be performed explicitly.".to_string(),
        );
    }

    let manifest_present = project_root
        .map(|root| manifest_path(root).is_some())
        .unwrap_or(false);
    let lock_present = project_root
        .map(|root| lock_path(root).is_some())
        .unwrap_or(false);
    let dependency_roots = project_root
        .map(
            |root| match library_catalog::sysand::installed_manifest_candidates(root) {
                Ok(candidates) => {
                    let mut roots = candidates
                        .into_iter()
                        .flat_map(|candidate| candidate.package_roots)
                        .collect::<Vec<_>>();
                    roots.sort();
                    roots.dedup();
                    roots
                }
                Err(error) => {
                    warnings.push(error);
                    Vec::new()
                }
            },
        )
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
    [library_catalog::PROJECT_FILE]
        .iter()
        .map(|name| root.join(name))
        .find(|path| path.is_file())
}

fn lock_path(root: &Path) -> Option<PathBuf> {
    [library_catalog::sysand::SYSAND_LOCK_FILE]
        .iter()
        .map(|name| root.join(name))
        .find(|path| path.is_file())
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
    fn detects_current_manifest_and_lock_from_a_nested_directory() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join(".project.json"),
            r#"{"name":"demo","version":"1.0.0"}"#,
        )
        .unwrap();
        std::fs::write(temp.path().join("sysand-lock.toml"), "lock_version = '0.6'").unwrap();
        let nested = temp.path().join("model");
        std::fs::create_dir(&nested).unwrap();
        assert_eq!(
            discover_project_root_from(&nested),
            Some(canonicalize_lossy(temp.path()))
        );
        let status = detect_sysand_status_from(Some(temp.path()), None, None);
        assert!(status.manifest_present);
        assert!(status.lock_present);
        assert!(status.dependency_roots.is_empty());
        assert!(!status.warnings.is_empty());
    }
    #[test]
    fn arbitrary_toml_paths_do_not_become_library_roots() {
        let temp = tempfile::tempdir().unwrap();
        std::fs::write(
            temp.path().join("sysand.toml"),
            "[dependencies]\nlocal = 'library/anything'",
        )
        .unwrap();
        assert_eq!(discover_project_root_from(temp.path()), None);
        assert!(detect_sysand_status_from(Some(temp.path()), None, None)
            .dependency_roots
            .is_empty());
    }
}
