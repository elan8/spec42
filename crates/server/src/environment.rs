use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::Cli;
use crate::stdlib::{
    managed_status, project_dirs, StandardLibraryConfig, StandardLibraryPaths,
    StandardLibraryStatus,
};
use crate::sysand::{dependency_roots_from_status, detect_sysand_status, SysandStatus};
use workspace::catalog::{HostLibraryRequest, KparLibraryComponent};
use workspace::library::managed::{managed_status as kpar_managed_status, KparLibraryStatus};
use workspace::{EngineBuilder, Spec42Engine};

#[cfg(test)]
use workspace::catalog::resolve_stdlib_component_for_test;

#[derive(Debug, Clone, Deserialize, Default)]
pub struct ConfigFile {
    pub library_paths: Option<Vec<String>>,
    pub stdlib_path: Option<String>,
    pub no_stdlib: Option<bool>,
    pub standard_library_version: Option<String>,
    pub standard_library_repo: Option<String>,
    pub standard_library_content_path: Option<String>,
    pub disabled_libraries: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct ResolvedEnvironment {
    pub config_file_used: Option<PathBuf>,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub library_paths: Vec<PathBuf>,
    pub stdlib_path: Option<PathBuf>,
    pub stdlib_roots: Vec<PathBuf>,
    pub stdlib_source: Option<String>,
    pub used_legacy_vscode_fallback: bool,
    pub kpar_libraries: Vec<KparLibraryComponent>,
    pub sysand: SysandStatus,
    pub standard_library: StandardLibraryConfig,
    pub standard_library_paths: StandardLibraryPaths,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorKparLibrary {
    pub id: String,
    pub display_name: String,
    pub path: Option<String>,
    pub source: Option<String>,
    pub source_kind: String,
    pub status: KparLibraryStatus,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorReport {
    pub version: String,
    pub mode: String,
    pub config_file_used: Option<String>,
    pub config_dir: String,
    pub data_dir: String,
    pub resolved_stdlib_path: Option<String>,
    pub stdlib_roots: Vec<String>,
    pub stdlib_source: Option<String>,
    pub stdlib_source_kind: String,
    pub used_legacy_vscode_fallback: bool,
    pub sysand: SysandStatus,
    pub standard_library_status: StandardLibraryStatus,
    pub kpar_libraries: Vec<DoctorKparLibrary>,
    pub library_paths: Vec<DoctorPathStatus>,
}

#[derive(Debug, Clone, Serialize)]
pub struct DoctorPathStatus {
    pub path: String,
    pub exists: bool,
}

pub fn resolve_environment(cli: &Cli) -> Result<ResolvedEnvironment, String> {
    let project_dirs = project_dirs()?;
    let config_dir = std::env::var_os("SPEC42_CONFIG_DIR")
        .map(PathBuf::from)
        .map(|path| canonicalize_lossy(path.as_path()))
        .unwrap_or_else(|| project_dirs.config_dir().to_path_buf());
    let data_dir = std::env::var_os("SPEC42_DATA_DIR")
        .map(PathBuf::from)
        .map(|path| canonicalize_lossy(path.as_path()))
        .unwrap_or_else(|| project_dirs.data_local_dir().to_path_buf());
    resolve_environment_with_dirs(cli, config_dir, data_dir)
}

fn resolve_environment_with_dirs(
    cli: &Cli,
    config_dir: PathBuf,
    data_dir: PathBuf,
) -> Result<ResolvedEnvironment, String> {
    let _project_dirs = project_dirs()?;
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

    let sysand = detect_sysand_status();
    let sysand_dependency_roots = dependency_roots_from_status(&sysand);
    let request = build_host_library_request(
        cli,
        &explicit_config,
        &default_config,
        data_dir.clone(),
        &sysand_dependency_roots,
        standard_library.clone(),
    )?;
    let engine = EngineBuilder::from_request(request)
        .build()
        .map_err(|error| error.to_string())?;
    let catalog = engine.library_catalog();

    Ok(ResolvedEnvironment {
        config_file_used,
        config_dir,
        data_dir,
        library_paths: catalog.package_roots.clone(),
        stdlib_path: catalog.stdlib.path.clone(),
        stdlib_roots: catalog.stdlib.roots.clone(),
        stdlib_source: catalog.stdlib.source.clone(),
        used_legacy_vscode_fallback: catalog.stdlib.used_legacy_vscode_fallback,
        kpar_libraries: catalog.kpar_libraries.clone(),
        sysand,
        standard_library,
        standard_library_paths: catalog.standard_library_paths.clone(),
    })
}

/// Build a [`Spec42Engine`] from CLI configuration without duplicating catalog resolution.
pub fn build_engine(cli: &Cli) -> Result<Spec42Engine, String> {
    let project_dirs = project_dirs()?;
    let config_dir = std::env::var_os("SPEC42_CONFIG_DIR")
        .map(PathBuf::from)
        .map(|path| canonicalize_lossy(path.as_path()))
        .unwrap_or_else(|| project_dirs.config_dir().to_path_buf());
    let data_dir = std::env::var_os("SPEC42_DATA_DIR")
        .map(PathBuf::from)
        .map(|path| canonicalize_lossy(path.as_path()))
        .unwrap_or_else(|| project_dirs.data_local_dir().to_path_buf());
    build_engine_with_dirs(cli, config_dir, data_dir)
}

fn build_engine_with_dirs(
    cli: &Cli,
    config_dir: PathBuf,
    data_dir: PathBuf,
) -> Result<Spec42Engine, String> {
    let explicit_config_path = cli
        .config_path
        .as_ref()
        .map(|path| canonicalize_lossy(path.as_path()));
    let default_config_path = config_dir.join("config.toml");
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
    let sysand = detect_sysand_status();
    let sysand_dependency_roots = dependency_roots_from_status(&sysand);
    let request = build_host_library_request(
        cli,
        &explicit_config,
        &default_config,
        data_dir,
        &sysand_dependency_roots,
        standard_library,
    )?;
    EngineBuilder::from_request(request)
        .build()
        .map_err(|error| error.to_string())
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
        if environment.stdlib_source.as_deref() != Some("flag")
            && environment.stdlib_source.as_deref() != Some("env")
            && environment.stdlib_source.as_deref() != Some("config")
            && environment.stdlib_source.as_deref() != Some("user-config")
        {
            status.is_installed = status.is_installed && stdlib_path.is_dir();
        }
    }

    let kpar_libraries = environment
        .kpar_libraries
        .iter()
        .map(build_doctor_kpar_library)
        .collect::<Result<Vec<_>, String>>()?;

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
        stdlib_roots: environment
            .stdlib_roots
            .iter()
            .map(|path| path.display().to_string())
            .collect(),
        stdlib_source: environment.stdlib_source.clone(),
        stdlib_source_kind: if environment.stdlib_source.as_deref() == Some("managed") {
            "canonical-managed".to_string()
        } else if environment.used_legacy_vscode_fallback {
            "compatibility-fallback".to_string()
        } else if environment.stdlib_source.as_deref() == Some("disabled") {
            "disabled".to_string()
        } else if environment.stdlib_source.as_deref() == Some("bundled") {
            "bundled".to_string()
        } else {
            "none".to_string()
        },
        used_legacy_vscode_fallback: environment.used_legacy_vscode_fallback,
        sysand: environment.sysand.clone(),
        standard_library_status: status,
        kpar_libraries,
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

fn kpar_library_source_kind(source: Option<&str>) -> String {
    match source {
        Some("bundled") => "bundled".to_string(),
        Some("managed") => "canonical-managed".to_string(),
        Some("flag") | Some("env") => "override".to_string(),
        Some("custom") => "custom".to_string(),
        Some("disabled") => "disabled".to_string(),
        _ => "none".to_string(),
    }
}

fn build_doctor_kpar_library(
    component: &KparLibraryComponent,
) -> Result<DoctorKparLibrary, String> {
    let mut status = kpar_managed_status(&component.paths, &component.config)?;
    if status.install_path.is_none() {
        status.install_path = component
            .path
            .as_ref()
            .map(|path| path.display().to_string());
    }
    if status.source.is_none() {
        status.source = component.source.clone();
    }
    if let Some(path) = &component.path {
        if !matches!(
            component.source.as_deref(),
            Some("flag") | Some("env") | Some("custom")
        ) {
            status.is_installed = status.is_installed && path.is_dir();
        }
    }

    Ok(DoctorKparLibrary {
        id: component.id.clone(),
        display_name: component.display_name.clone(),
        path: component
            .path
            .as_ref()
            .map(|path| path.display().to_string()),
        source: component.source.clone(),
        source_kind: kpar_library_source_kind(component.source.as_deref()),
        status,
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

fn build_host_library_request(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
    cache_dir: PathBuf,
    sysand_dependency_roots: &[PathBuf],
    standard_library: StandardLibraryConfig,
) -> Result<HostLibraryRequest, String> {
    let library_paths = resolve_explicit_library_paths(cli, explicit_config, default_config);
    let config_stdlib_path = explicit_config
        .stdlib_path
        .as_ref()
        .or(default_config.stdlib_path.as_ref())
        .map(|path| canonicalize_lossy(Path::new(path)));
    let config_no_stdlib =
        explicit_config.no_stdlib.unwrap_or(false) || default_config.no_stdlib.unwrap_or(false);

    Ok(HostLibraryRequest {
        cache_dir,
        no_stdlib: cli.no_stdlib,
        stdlib_path_override: cli.stdlib_path.clone(),
        kpar_library_path_overrides: parse_kpar_library_path_overrides(&cli.kpar_library_paths)?,
        disabled_kpar_libraries: resolve_disabled_kpar_libraries(
            cli,
            explicit_config,
            default_config,
        ),
        library_paths,
        standard_library,
        use_embedded_stdlib: cfg!(feature = "embed-stdlib"),
        use_embedded_kpar_libraries: cfg!(feature = "embed-kpar-libraries"),
        config_stdlib_path,
        config_no_stdlib,
        extra_library_paths: sysand_dependency_roots.to_vec(),
    })
}

fn resolve_disabled_kpar_libraries(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
) -> BTreeSet<String> {
    let mut disabled: BTreeSet<String> = cli.disabled_kpar_libraries.iter().cloned().collect();
    if let Some(value) = std::env::var_os("SPEC42_DISABLED_KPAR_LIBRARIES") {
        if let Some(value) = value.to_str() {
            disabled.extend(
                value
                    .split(',')
                    .map(str::trim)
                    .filter(|id| !id.is_empty())
                    .map(str::to_string),
            );
        }
    }
    if let Some(ids) = &explicit_config.disabled_libraries {
        disabled.extend(ids.iter().cloned());
    }
    if let Some(ids) = &default_config.disabled_libraries {
        disabled.extend(ids.iter().cloned());
    }
    disabled
}

fn parse_kpar_library_path_overrides(
    entries: &[String],
) -> Result<BTreeMap<String, PathBuf>, String> {
    let mut overrides = BTreeMap::new();
    for entry in entries {
        let (id, path) = entry
            .split_once('=')
            .ok_or_else(|| format!("invalid --kpar-library-path '{entry}' (expected ID=PATH)"))?;
        let id = id.trim();
        let path = path.trim();
        if id.is_empty() || path.is_empty() {
            return Err(format!(
                "invalid --kpar-library-path '{entry}' (empty id or path)"
            ));
        }
        overrides.insert(id.to_string(), canonicalize_lossy(Path::new(path)));
    }
    Ok(overrides)
}

fn resolve_explicit_library_paths(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
) -> Vec<PathBuf> {
    resolve_library_paths(cli, explicit_config, default_config, &[], &[])
}

fn resolve_library_paths(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
    sysand_dependency_roots: &[PathBuf],
    stdlib_roots: &[PathBuf],
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

    paths.extend(sysand_dependency_roots.iter().cloned());
    paths.extend(stdlib_roots.iter().cloned());

    let mut deduped = BTreeSet::new();
    paths
        .into_iter()
        .filter(|path| deduped.insert(path.display().to_string()))
        .collect()
}

#[cfg(test)]
struct StdlibResolution {
    path: Option<PathBuf>,
    source: Option<String>,
    used_legacy_vscode_fallback: bool,
}

#[cfg(test)]
fn resolve_stdlib_path(
    cli: &Cli,
    explicit_config: &ConfigFile,
    default_config: &ConfigFile,
    standard_library: &StandardLibraryConfig,
    standard_library_paths: &StandardLibraryPaths,
) -> Result<StdlibResolution, String> {
    let config_stdlib_path = explicit_config
        .stdlib_path
        .as_ref()
        .or(default_config.stdlib_path.as_ref())
        .map(|path| canonicalize_lossy(Path::new(path)));
    let config_no_stdlib =
        explicit_config.no_stdlib.unwrap_or(false) || default_config.no_stdlib.unwrap_or(false);
    let request = HostLibraryRequest {
        cache_dir: standard_library_paths.managed_root.clone(),
        no_stdlib: cli.no_stdlib,
        stdlib_path_override: cli.stdlib_path.clone(),
        kpar_library_path_overrides: BTreeMap::new(),
        disabled_kpar_libraries: BTreeSet::new(),
        library_paths: Vec::new(),
        standard_library: standard_library.clone(),
        use_embedded_stdlib: cfg!(feature = "embed-stdlib"),
        use_embedded_kpar_libraries: false,
        config_stdlib_path,
        config_no_stdlib,
        extra_library_paths: Vec::new(),
    };
    let component = resolve_stdlib_component_for_test(&request, standard_library_paths)
        .map_err(|error| error.to_string())?;
    Ok(StdlibResolution {
        path: component.path,
        source: component.source,
        used_legacy_vscode_fallback: component.used_legacy_vscode_fallback,
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

/// True when any `.sysml` / `.kerml` file under `path` references common standard-library packages.
pub fn workspace_references_standard_library(path: &Path) -> bool {
    fn file_references_stdlib(path: &Path) -> bool {
        let Ok(content) = fs::read_to_string(path) else {
            return false;
        };
        content.contains("ScalarValues")
            || content.contains("ISQ::")
            || content.contains("ISQ ")
            || content.contains("SI::")
            || content.contains("import ISQ")
            || content.contains("import SI")
    }

    fn walk(dir: &Path, budget: &mut usize) -> bool {
        for entry in walkdir::WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(Result::ok)
        {
            if *budget == 0 {
                break;
            }
            if !entry.file_type().is_file() {
                continue;
            }
            let path = entry.path();
            if !path
                .extension()
                .is_some_and(|ext| ext == "sysml" || ext == "kerml")
            {
                continue;
            }
            *budget = budget.saturating_sub(1);
            if file_references_stdlib(path) {
                return true;
            }
        }
        false
    }

    if path.is_file() {
        return file_references_stdlib(path);
    }
    let mut budget = 256usize;
    walk(path, &mut budget)
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;
    use crate::stdlib::{
        save_managed_metadata, standard_library_paths_from_data_dir, DEFAULT_STDLIB_CONTENT_PATH,
        EMBEDDED_STDLIB_REPO,
    };

    /// Serializes tests that mutate `APPDATA` (global process environment).
    static APPDATA_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn empty_cli() -> Cli {
        Cli {
            config_path: None,
            library_paths: Vec::new(),
            stdlib_path: None,
            kpar_library_paths: Vec::new(),
            disabled_kpar_libraries: Vec::new(),
            no_stdlib: false,
            stdio: false,
            command: None,
        }
    }

    #[test]
    fn explicit_library_paths_take_precedence() {
        let mut cli = empty_cli();
        cli.library_paths = vec![PathBuf::from("C:/models/lib")];
        let paths = resolve_library_paths(
            &cli,
            &ConfigFile::default(),
            &ConfigFile::default(),
            &[],
            &[],
        );
        assert_eq!(paths, vec![PathBuf::from("C:/models/lib")]);
    }

    #[test]
    fn parse_kpar_library_path_overrides_accepts_id_path() {
        let overrides = parse_kpar_library_path_overrides(&["domain=C:/libs/domain".to_string()])
            .expect("parse");
        assert_eq!(
            overrides
                .get("domain")
                .map(|path| path.display().to_string()),
            Some(
                canonicalize_lossy(Path::new("C:/libs/domain"))
                    .display()
                    .to_string()
            )
        );
    }

    #[test]
    fn disabled_kpar_library_is_excluded_from_resolution() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");

        let mut cli = empty_cli();
        cli.no_stdlib = true;
        cli.disabled_kpar_libraries = vec!["domain".to_string()];
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");

        let domain = environment
            .kpar_libraries
            .iter()
            .find(|library| library.id == "domain")
            .expect("domain library");
        assert_eq!(domain.source.as_deref(), Some("disabled"));
        assert!(domain.path.is_none());
        assert!(!environment
            .library_paths
            .iter()
            .any(|path| domain.path.as_ref() == Some(path)));

        let doctor = build_doctor_report("doctor", &environment).expect("doctor");
        let domain_doctor = doctor
            .kpar_libraries
            .iter()
            .find(|library| library.id == "domain")
            .expect("domain doctor entry");
        assert_eq!(domain_doctor.source_kind, "disabled");
    }

    #[test]
    fn unregistered_kpar_library_path_registers_ad_hoc_library() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        let custom_lib_dir = temp.path().join("mylib");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");
        std::fs::create_dir_all(&custom_lib_dir).expect("create custom lib dir");
        std::fs::write(custom_lib_dir.join("Custom.sysml"), "package Custom { }")
            .expect("write custom sysml file");

        let mut cli = empty_cli();
        cli.no_stdlib = true;
        cli.kpar_library_paths = vec![format!("mylib={}", custom_lib_dir.display())];
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");

        let custom = environment
            .kpar_libraries
            .iter()
            .find(|library| library.id == "mylib")
            .expect("ad-hoc mylib library");
        assert_eq!(custom.source.as_deref(), Some("custom"));
        assert_eq!(
            custom.path.as_ref().map(|p| canonicalize_lossy(p)),
            Some(canonicalize_lossy(&custom_lib_dir))
        );
        assert!(environment
            .library_paths
            .iter()
            .any(|path| custom.path.as_ref() == Some(path)));
    }

    #[test]
    fn explicit_no_stdlib_disables_resolution() {
        let mut cli = empty_cli();
        cli.no_stdlib = true;
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
    fn resolve_environment_uses_env_config_and_data_dirs() {
        let _guard = APPDATA_TEST_LOCK.lock().expect("env lock");

        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");

        let old_config_dir = std::env::var_os("SPEC42_CONFIG_DIR");
        let old_data_dir = std::env::var_os("SPEC42_DATA_DIR");
        std::env::set_var("SPEC42_CONFIG_DIR", &config_dir);
        std::env::set_var("SPEC42_DATA_DIR", &data_dir);

        let mut cli = empty_cli();
        cli.no_stdlib = true;
        let environment = resolve_environment(&cli).expect("environment");

        match old_config_dir {
            Some(value) => std::env::set_var("SPEC42_CONFIG_DIR", value),
            None => std::env::remove_var("SPEC42_CONFIG_DIR"),
        }
        match old_data_dir {
            Some(value) => std::env::set_var("SPEC42_DATA_DIR", value),
            None => std::env::remove_var("SPEC42_DATA_DIR"),
        }

        assert_eq!(environment.config_dir, canonicalize_lossy(&config_dir));
        assert_eq!(environment.data_dir, canonicalize_lossy(&data_dir));
        assert_eq!(
            environment.standard_library_paths.managed_root,
            canonicalize_lossy(&data_dir).join("standard-library")
        );
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
            .join(crate::stdlib::DEFAULT_STDLIB_VERSION)
            .join("kpar");
        std::fs::create_dir_all(&install_path).expect("create install path");
        std::fs::write(
            install_path.join("ScalarValues.sysml"),
            "standard library package ScalarValues { attribute def Real; }",
        )
        .expect("write stdlib file");
        save_managed_metadata(
            &paths,
            &crate::stdlib::StandardLibraryMetadata {
                installed_version: crate::stdlib::DEFAULT_STDLIB_VERSION.to_string(),
                install_path: install_path.display().to_string(),
                installed_at: "0".to_string(),
                repo: crate::stdlib::DEFAULT_STDLIB_REPO.to_string(),
                content_path: "kpar".to_string(),
                format: crate::stdlib::DEFAULT_STDLIB_FORMAT.to_string(),
                library_roots: vec![install_path.display().to_string()],
                project_name: None,
            },
        )
        .expect("save metadata");

        let cli = empty_cli();
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

    #[cfg(not(feature = "embed-stdlib"))]
    #[test]
    fn stale_managed_stdlib_metadata_is_not_used_without_embedded_repair() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");
        let paths = standard_library_paths_from_data_dir(data_dir.clone());
        let stale_install_path = paths
            .managed_root
            .join("versions")
            .join("2026-02")
            .join("kpar");
        std::fs::create_dir_all(&stale_install_path).expect("create stale install path");
        save_managed_metadata(
            &paths,
            &crate::stdlib::StandardLibraryMetadata {
                installed_version: "2026-02".to_string(),
                install_path: stale_install_path.display().to_string(),
                installed_at: "0".to_string(),
                repo: EMBEDDED_STDLIB_REPO.to_string(),
                content_path: "kpar".to_string(),
                format: crate::stdlib::DEFAULT_STDLIB_FORMAT.to_string(),
                library_roots: vec![stale_install_path.display().to_string()],
                project_name: None,
            },
        )
        .expect("save metadata");

        let cli = empty_cli();
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");
        assert_ne!(environment.stdlib_path.as_ref(), Some(&stale_install_path));
        assert!(!environment
            .library_paths
            .iter()
            .any(|path| path == &stale_install_path));
        let doctor = build_doctor_report("doctor", &environment).expect("doctor");
        assert!(!doctor.standard_library_status.is_installed);
        assert!(!doctor.standard_library_status.version_matches);
    }

    #[cfg(feature = "embed-stdlib")]
    #[test]
    fn stale_managed_stdlib_metadata_is_repaired_from_embedded_bundle() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");
        let paths = standard_library_paths_from_data_dir(data_dir.clone());
        let stale_install_path = paths
            .managed_root
            .join("versions")
            .join("2026-02")
            .join("kpar");
        std::fs::create_dir_all(&stale_install_path).expect("create stale install path");
        save_managed_metadata(
            &paths,
            &crate::stdlib::StandardLibraryMetadata {
                installed_version: "2026-02".to_string(),
                install_path: stale_install_path.display().to_string(),
                installed_at: "0".to_string(),
                repo: EMBEDDED_STDLIB_REPO.to_string(),
                content_path: "kpar".to_string(),
                format: crate::stdlib::DEFAULT_STDLIB_FORMAT.to_string(),
                library_roots: vec![stale_install_path.display().to_string()],
                project_name: None,
            },
        )
        .expect("save metadata");

        let cli = empty_cli();
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");
        let expected_path = crate::stdlib::managed_install_path(
            &environment.standard_library_paths,
            &environment.standard_library,
        );

        assert_eq!(environment.stdlib_path.as_ref(), Some(&expected_path));
        assert_eq!(environment.stdlib_source.as_deref(), Some("bundled"));
        let doctor = build_doctor_report("doctor", &environment).expect("doctor");
        assert!(doctor.standard_library_status.is_installed);
        assert_eq!(
            doctor.standard_library_status.installed_version.as_deref(),
            Some(crate::stdlib::DEFAULT_STDLIB_VERSION)
        );
    }

    #[cfg(feature = "embed-stdlib")]
    #[test]
    fn embedded_stdlib_precedes_legacy_vscode_path() {
        let _guard = APPDATA_TEST_LOCK.lock().expect("env lock");

        let temp = tempfile::tempdir().expect("temp dir");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&data_dir).expect("create data dir");

        let fake_appdata = temp.path().join("Roaming");
        std::fs::create_dir_all(
            fake_appdata
                .join("Code")
                .join("User")
                .join("globalStorage")
                .join("elan8.spec42")
                .join("standard-library")
                .join(crate::stdlib::DEFAULT_STDLIB_VERSION)
                .join(DEFAULT_STDLIB_CONTENT_PATH),
        )
        .expect("create legacy vscode path");

        let old_appdata = std::env::var_os("APPDATA");
        std::env::set_var("APPDATA", &fake_appdata);

        let paths = standard_library_paths_from_data_dir(data_dir);
        let cli = empty_cli();
        let resolution = resolve_stdlib_path(
            &cli,
            &ConfigFile::default(),
            &ConfigFile::default(),
            &StandardLibraryConfig::default(),
            &paths,
        )
        .expect("resolve stdlib");

        match old_appdata {
            Some(v) => std::env::set_var("APPDATA", v),
            None => std::env::remove_var("APPDATA"),
        }

        assert_eq!(resolution.source.as_deref(), Some("bundled"));
        assert!(!resolution.used_legacy_vscode_fallback);
        assert!(resolution.path.is_some());
    }

    #[cfg(feature = "embed-kpar-libraries")]
    #[test]
    fn resolve_environment_materializes_embedded_kpar_libraries() {
        let temp = tempfile::tempdir().expect("temp dir");
        let config_dir = temp.path().join("config");
        let data_dir = temp.path().join("data");
        std::fs::create_dir_all(&config_dir).expect("create config dir");
        std::fs::create_dir_all(&data_dir).expect("create data dir");

        let mut cli = empty_cli();
        cli.no_stdlib = true;
        let environment =
            resolve_environment_with_dirs(&cli, config_dir, data_dir).expect("environment");

        let domain = environment
            .kpar_libraries
            .iter()
            .find(|library| library.id == "domain")
            .expect("domain library");
        assert_eq!(domain.source.as_deref(), Some("bundled"));
        assert!(domain.path.is_some());
        assert!(environment
            .library_paths
            .iter()
            .any(|path| domain.path.as_ref() == Some(path)));

        let doctor = build_doctor_report("doctor", &environment).expect("doctor");
        let domain_doctor = doctor
            .kpar_libraries
            .iter()
            .find(|library| library.id == "domain")
            .expect("domain doctor entry");
        assert_eq!(domain_doctor.source_kind, "bundled");
        assert!(domain_doctor.status.is_installed);

        let method = doctor
            .kpar_libraries
            .iter()
            .find(|library| library.id == "method")
            .expect("method doctor entry");
        assert_eq!(method.source_kind, "bundled");
        assert!(method.status.is_installed);
    }
}
