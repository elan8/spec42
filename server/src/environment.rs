use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::Cli;
use crate::stdlib::{
    legacy_vscode_stdlib_path, load_managed_metadata, managed_status, project_dirs,
    standard_library_paths_from_data_dir, StandardLibraryConfig, StandardLibraryPaths,
    StandardLibraryStatus,
};

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConfigFile {
    pub library_paths: Option<Vec<String>>,
    pub stdlib_path: Option<String>,
    pub no_stdlib: Option<bool>,
    pub standard_library_version: Option<String>,
    pub standard_library_repo: Option<String>,
    pub standard_library_content_path: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResolvedEnvironment {
    pub config_file_used: Option<PathBuf>,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub library_paths: Vec<PathBuf>,
    pub stdlib_path: Option<PathBuf>,
    pub stdlib_source: Option<String>,
    pub used_legacy_vscode_fallback: bool,
    pub standard_library: StandardLibraryConfig,
    pub standard_library_paths: StandardLibraryPaths,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorReport {
    pub version: String,
    pub mode: String,
    pub config_file_used: Option<String>,
    pub config_dir: String,
    pub data_dir: String,
    pub resolved_stdlib_path: Option<String>,
    pub stdlib_source: Option<String>,
    pub stdlib_source_kind: String,
    pub used_legacy_vscode_fallback: bool,
    pub standard_library_status: StandardLibraryStatus,
    pub library_paths: Vec<DoctorPathStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorPathStatus {
    pub path: String,
    pub exists: bool,
}

pub fn resolve_environment(cli: &Cli) -> Result<ResolvedEnvironment, String> {
    let project_dirs = project_dirs()?;
    resolve_environment_with_dirs(
        cli,
        project_dirs.config_dir().to_path_buf(),
        project_dirs.data_local_dir().to_path_buf(),
    )
}

fn resolve_environment_with_dirs(
    cli: &Cli,
    config_dir: PathBuf,
    data_dir: PathBuf,
) -> Result<ResolvedEnvironment, String> {
    let project_dirs = project_dirs()?;
    let _ = project_dirs;
    let standard_library_paths = standard_library_paths_from_data_dir(data_dir.clone());

    let explicit_config_path = cli
        .config_path
        .as_ref()
        .map(|path| canonicalize_lossy(path.as_path()));
    let default_config_path = config_dir.join("config.toml");
    let config_file_used = explicit_config_path
        .clone()
        .filter(|path| path.is_file())
        .or_else(|| {
            default_config_path
                .is_file()
                .then_some(default_config_path.clone())
        });

    let explicit_config = explicit_config_path
        .as_ref()
        .map(|path| load_config_file(path.as_path()))
        .transpose()?
        .unwrap_or_default();
    let default_config = if explicit_config_path.is_none() {
        load_config_file_if_present(&default_config_path)?
    } else {
        ConfigFile::default()
    };

    let standard_library = resolve_standard_library_config(cli, &explicit_config, &default_config);
    let stdlib_resolution = resolve_stdlib_path(
        cli,
        &explicit_config,
        &default_config,
        &standard_library,
        &standard_library_paths,
    )?;

    let library_paths = resolve_library_paths(
        cli,
        &explicit_config,
        &default_config,
        stdlib_resolution.path.as_ref(),
    );

    Ok(ResolvedEnvironment {
        config_file_used,
        config_dir,
        data_dir,
        library_paths,
        stdlib_path: stdlib_resolution.path,
        stdlib_source: stdlib_resolution.source,
        used_legacy_vscode_fallback: stdlib_resolution.used_legacy_vscode_fallback,
        standard_library,
        standard_library_paths,
    })
}

pub fn build_doctor_report(
    mode: &str,
    environment: &ResolvedEnvironment,
) -> Result<DoctorReport, String> {
    let mut status = managed_status(
        &environment.standard_library_paths,
        &environment.standard_library,
    )?;
    if status.install_path.is_none() {
        status.install_path = environment
            .stdlib_path
            .as_ref()
            .map(|path| path.display().to_string());
    }
    if status.source.is_none() {
        status.source = environment.stdlib_source.clone();
    }
    if let Some(stdlib_path) = &environment.stdlib_path {
        status.is_installed = stdlib_path.is_dir();
    }
    Ok(DoctorReport {
        version: env!("CARGO_PKG_VERSION").to_string(),
        mode: mode.to_string(),
        config_file_used: environment
            .config_file_used
            .as_ref()
            .map(|path| path.display().to_string()),
        config_dir: environment.config_dir.display().to_string(),
        data_dir: environment.data_dir.display().to_string(),
        resolved_stdlib_path: environment
            .stdlib_path
            .as_ref()
            .map(|path| path.display().to_string()),
        stdlib_source: environment.stdlib_source.clone(),
        stdlib_source_kind: if environment.stdlib_source.as_deref() == Some("managed") {
            "canonical-managed".to_string()
        } else if environment.used_legacy_vscode_fallback {
            "compatibility-fallback".to_string()
        } else if environment.stdlib_source.as_deref() == Some("disabled") {
            "disabled".to_string()
        } else {
            "none".to_string()
        },
        used_legacy_vscode_fallback: environment.used_legacy_vscode_fallback,
        standard_library_status: status,
        library_paths: environment
            .library_paths
            .iter()
            .map(|path| DoctorPathStatus {
                path: path.display().to_string(),
                exists: path.is_dir(),
            })
            .collect(),
    })
}

fn resolve_standard_library_config(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
) -> StandardLibraryConfig {
    let mut config = StandardLibraryConfig::default();
    config.version = explicit_config
        .standard_library_version
        .clone()
        .or_else(|| default_config.standard_library_version.clone())
        .unwrap_or(config.version);
    config.repo = explicit_config
        .standard_library_repo
        .clone()
        .or_else(|| default_config.standard_library_repo.clone())
        .unwrap_or(config.repo);
    config.content_path = explicit_config
        .standard_library_content_path
        .clone()
        .or_else(|| default_config.standard_library_content_path.clone())
        .unwrap_or(config.content_path);
    if let Ok(version) = std::env::var("SPEC42_STDLIB_VERSION") {
        if !version.trim().is_empty() {
            config.version = version;
        }
    }
    if let Ok(repo) = std::env::var("SPEC42_STDLIB_REPO") {
        if !repo.trim().is_empty() {
            config.repo = repo;
        }
    }
    if let Ok(content_path) = std::env::var("SPEC42_STDLIB_CONTENT_PATH") {
        if !content_path.trim().is_empty() {
            config.content_path = content_path;
        }
    }
    if let Some(path) = &cli.stdlib_path {
        let _ = path;
    }
    config
}

fn resolve_library_paths(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
    stdlib_path: Option<&PathBuf>,
) -> Vec<PathBuf> {
    let mut paths = if !cli.library_paths.is_empty() {
        cli.library_paths
            .iter()
            .map(|path| canonicalize_lossy(path.as_path()))
            .collect::<Vec<_>>()
    } else if let Some(value) = std::env::var_os("SPEC42_LIBRARY_PATHS") {
        split_paths(&value)
    } else if let Some(paths) = &explicit_config.library_paths {
        paths
            .iter()
            .map(PathBuf::from)
            .map(|path| canonicalize_lossy(path.as_path()))
            .collect()
    } else if let Some(paths) = &default_config.library_paths {
        paths
            .iter()
            .map(PathBuf::from)
            .map(|path| canonicalize_lossy(path.as_path()))
            .collect()
    } else {
        Vec::new()
    };

    if let Some(stdlib_path) = stdlib_path {
        paths.push(stdlib_path.clone());
    }

    let mut deduped = BTreeSet::new();
    paths
        .into_iter()
        .filter(|path| deduped.insert(path.display().to_string()))
        .collect()
}

struct StdlibResolution {
    path: Option<PathBuf>,
    source: Option<String>,
    used_legacy_vscode_fallback: bool,
}

fn resolve_stdlib_path(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
    standard_library: &StandardLibraryConfig,
    standard_library_paths: &StandardLibraryPaths,
) -> Result<StdlibResolution, String> {
    let no_stdlib = cli.no_stdlib
        || std::env::var("SPEC42_NO_STDLIB")
            .map(|value| matches!(value.trim(), "1" | "true" | "yes" | "on"))
            .unwrap_or(false)
        || explicit_config.no_stdlib.unwrap_or(false)
        || default_config.no_stdlib.unwrap_or(false);
    if no_stdlib {
        return Ok(StdlibResolution {
            path: None,
            source: Some("disabled".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(path) = cli.stdlib_path.as_ref() {
        return Ok(StdlibResolution {
            path: Some(canonicalize_lossy(path)),
            source: Some("flag".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(value) = std::env::var_os("SPEC42_STDLIB_PATH") {
        return Ok(StdlibResolution {
            path: Some(canonicalize_lossy(&PathBuf::from(value))),
            source: Some("env".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(path) = explicit_config.stdlib_path.as_ref() {
        return Ok(StdlibResolution {
            path: Some(canonicalize_lossy(&PathBuf::from(path))),
            source: Some("config".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }
    if let Some(path) = default_config.stdlib_path.as_ref() {
        return Ok(StdlibResolution {
            path: Some(canonicalize_lossy(&PathBuf::from(path))),
            source: Some("user-config".to_string()),
            used_legacy_vscode_fallback: false,
        });
    }

    if let Some(metadata) = load_managed_metadata(standard_library_paths)? {
        let managed_path = PathBuf::from(metadata.install_path);
        if managed_path.is_dir() {
            return Ok(StdlibResolution {
                path: Some(managed_path),
                source: Some("managed".to_string()),
                used_legacy_vscode_fallback: false,
            });
        }
    }

    if let Some(path) = legacy_vscode_stdlib_path(standard_library) {
        return Ok(StdlibResolution {
            path: Some(path),
            source: Some("legacy-vscode".to_string()),
            used_legacy_vscode_fallback: true,
        });
    }

    Ok(StdlibResolution {
        path: None,
        source: None,
        used_legacy_vscode_fallback: false,
    })
}

fn load_config_file_if_present(path: &Path) -> Result<ConfigFile, String> {
    if !path.is_file() {
        return Ok(ConfigFile::default());
    }
    load_config_file(path)
}

fn load_config_file(path: &Path) -> Result<ConfigFile, String> {
    let raw = fs::read_to_string(path)
        .map_err(|err| format!("Failed to read {}: {err}", path.display()))?;
    toml::from_str(&raw).map_err(|err| format!("Failed to parse {}: {err}", path.display()))
}

fn split_paths(value: &std::ffi::OsStr) -> Vec<PathBuf> {
    std::env::split_paths(value)
        .map(|path| canonicalize_lossy(path.as_path()))
        .collect::<Vec<_>>()
}

fn canonicalize_lossy(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::{save_managed_metadata, standard_library_paths_from_data_dir};

    #[test]
    fn explicit_library_paths_take_precedence() {
        let cli = Cli {
            config_path: None,
            library_paths: vec![PathBuf::from("C:/models/lib")],
            stdlib_path: None,
            no_stdlib: false,
            command: None,
        };
        let paths =
            resolve_library_paths(&cli, &ConfigFile::default(), &ConfigFile::default(), None);
        assert_eq!(paths, vec![PathBuf::from("C:/models/lib")]);
    }

    #[test]
    fn explicit_no_stdlib_disables_resolution() {
        let cli = Cli {
            config_path: None,
            library_paths: Vec::new(),
            stdlib_path: None,
            no_stdlib: true,
            command: None,
        };
        let resolution = resolve_stdlib_path(
            &cli,
            &ConfigFile::default(),
            &ConfigFile::default(),
            &StandardLibraryConfig::default(),
            &standard_library_paths_from_data_dir(std::env::temp_dir().join("spec42-stdlib-test")),
        )
        .expect("resolve stdlib");
        assert!(resolution.path.is_none());
        assert_eq!(resolution.source.as_deref(), Some("disabled"));
    }

    #[test]
    fn resolve_environment_prefers_managed_stdlib_and_includes_it_in_library_paths() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");
        let paths = standard_library_paths_from_data_dir(data_dir.clone());
        let install_path = paths
            .managed_root
            .join("versions")
            .join("2026-02")
            .join("sysml.library");
        std::fs::create_dir_all(&install_path).expect("create install path");
        std::fs::write(
            install_path.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("write stdlib file");
        save_managed_metadata(
            &paths,
            &crate::stdlib::StandardLibraryMetadata {
                installed_version: "2026-02".to_string(),
                install_path: install_path.display().to_string(),
                installed_at: "0".to_string(),
                repo: "Systems-Modeling/SysML-v2-Release".to_string(),
                content_path: "sysml.library".to_string(),
            },
        )
        .expect("save metadata");

        let cli = Cli {
            config_path: None,
            library_paths: Vec::new(),
            stdlib_path: None,
            no_stdlib: false,
            command: None,
        };
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");
        assert_eq!(environment.stdlib_source.as_deref(), Some("managed"));
        assert!(environment
            .library_paths
            .iter()
            .any(|path| path == &install_path));
        let doctor = build_doctor_report("doctor", &environment).expect("doctor");
        assert_eq!(doctor.stdlib_source_kind, "canonical-managed");
        assert!(doctor.standard_library_status.is_canonical_managed);
    }
}
