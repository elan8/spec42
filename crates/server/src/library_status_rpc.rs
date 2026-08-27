//! LSP custom method `sysml/libraryStatus` — live stdlib + managed KPAR library status.

use std::sync::Arc;

use library_catalog::library::managed::{managed_status as kpar_managed_status, KparLibraryStatus};
use library_catalog::library::stdlib::{
    managed_status as stdlib_managed_status, StandardLibraryConfig, StandardLibraryPaths,
    StandardLibraryStatus,
};
use library_catalog::KparLibraryComponent;
use lsp_server::{CustomRpcContext, CustomRpcProvider};
use serde::Serialize;

const METHOD: &str = "sysml/libraryStatus";

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryStatusResponse {
    pub stdlib: StdlibLibraryStatusDto,
    pub kpar_libraries: Vec<KparLibraryStatusDto>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StdlibLibraryStatusDto {
    pub pinned_version: String,
    pub installed_version: Option<String>,
    pub format: String,
    pub available: bool,
    pub resolved_path: Option<String>,
    pub source_kind: String,
    pub version_matches: bool,
    pub status_message: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct KparLibraryStatusDto {
    pub id: String,
    pub display_name: String,
    pub pinned_version: String,
    pub installed_version: Option<String>,
    pub format: String,
    pub available: bool,
    pub resolved_path: Option<String>,
    pub source_kind: String,
    pub version_matches: bool,
    pub is_installed: bool,
    pub status_message: Option<String>,
}

#[derive(Debug)]
pub struct LibraryStatusRpcProvider {
    stdlib_config: StandardLibraryConfig,
    stdlib_paths: StandardLibraryPaths,
    stdlib_source: Option<String>,
    stdlib_path: Option<std::path::PathBuf>,
    kpar_libraries: Vec<KparLibraryComponent>,
}

impl LibraryStatusRpcProvider {
    pub fn new(
        stdlib_config: StandardLibraryConfig,
        stdlib_paths: StandardLibraryPaths,
        stdlib_source: Option<String>,
        stdlib_path: Option<std::path::PathBuf>,
        kpar_libraries: Vec<KparLibraryComponent>,
    ) -> Self {
        Self {
            stdlib_config,
            stdlib_paths,
            stdlib_source,
            stdlib_path,
            kpar_libraries,
        }
    }

    pub fn build_response(&self) -> Result<LibraryStatusResponse, String> {
        let mut stdlib_status = stdlib_managed_status(&self.stdlib_paths, &self.stdlib_config)?;
        if stdlib_status.install_path.is_none() {
            stdlib_status.install_path = self
                .stdlib_path
                .as_ref()
                .map(|path| path.display().to_string());
        }
        if stdlib_status.source.is_none() {
            stdlib_status.source = self.stdlib_source.clone();
        }

        let kpar_libraries = self
            .kpar_libraries
            .iter()
            .map(build_kpar_dto)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(LibraryStatusResponse {
            stdlib: stdlib_dto(&stdlib_status, self.stdlib_config.format.clone()),
            kpar_libraries,
        })
    }
}

impl CustomRpcProvider for LibraryStatusRpcProvider {
    fn custom_method_names(&self) -> Vec<String> {
        vec![METHOD.to_string()]
    }

    fn try_handle(
        &self,
        method: &str,
        _params: serde_json::Value,
        _context: CustomRpcContext<'_>,
    ) -> tower_lsp::jsonrpc::Result<Option<serde_json::Value>> {
        if method != METHOD {
            return Ok(None);
        }
        let response = self.build_response().map_err(|message| {
            let mut error = tower_lsp::jsonrpc::Error::internal_error();
            error.message = message.into();
            error
        })?;
        let value = serde_json::to_value(response).map_err(|error| {
            let mut err = tower_lsp::jsonrpc::Error::internal_error();
            err.message = format!("serialize {METHOD}: {error}").into();
            err
        })?;
        Ok(Some(value))
    }
}

pub fn library_status_rpc_provider(
    stdlib_config: StandardLibraryConfig,
    stdlib_paths: StandardLibraryPaths,
    stdlib_source: Option<String>,
    stdlib_path: Option<std::path::PathBuf>,
    kpar_libraries: Vec<KparLibraryComponent>,
) -> Arc<dyn CustomRpcProvider> {
    Arc::new(LibraryStatusRpcProvider::new(
        stdlib_config,
        stdlib_paths,
        stdlib_source,
        stdlib_path,
        kpar_libraries,
    ))
}

fn source_kind(source: Option<&str>) -> String {
    match source {
        Some("bundled") => "bundled".to_string(),
        Some("managed") => "canonical-managed".to_string(),
        Some("flag") | Some("env") => "override".to_string(),
        Some("custom") => "custom".to_string(),
        Some("disabled") => "disabled".to_string(),
        _ => "none".to_string(),
    }
}

fn stdlib_dto(status: &StandardLibraryStatus, format: String) -> StdlibLibraryStatusDto {
    StdlibLibraryStatusDto {
        pinned_version: status.pinned_version.clone(),
        installed_version: status.installed_version.clone(),
        format,
        available: status.is_installed || status.install_path.is_some(),
        resolved_path: status.install_path.clone(),
        source_kind: source_kind(status.source.as_deref()),
        version_matches: status.version_matches,
        status_message: status.status_message.clone(),
    }
}

fn build_kpar_dto(component: &KparLibraryComponent) -> Result<KparLibraryStatusDto, String> {
    let mut status: KparLibraryStatus = kpar_managed_status(&component.paths, &component.config)?;
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

    let source = component.source.as_deref().or(status.source.as_deref());
    Ok(KparLibraryStatusDto {
        id: component.id.clone(),
        display_name: component.display_name.clone(),
        pinned_version: status.pinned_version,
        installed_version: status.installed_version,
        format: component.config.format.clone(),
        available: status.is_installed || component.path.is_some(),
        resolved_path: component
            .path
            .as_ref()
            .map(|path| path.display().to_string())
            .or(status.install_path),
        source_kind: source_kind(source),
        version_matches: status.version_matches,
        is_installed: status.is_installed,
        status_message: status.status_message,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use library_catalog::library::managed::{
        kpar_library_paths_from_data_dir, KparLibraryConfig, EMBEDDED_KPAR_LIBRARY_REPO,
    };

    #[test]
    fn provider_advertises_library_status_method() {
        let provider = LibraryStatusRpcProvider::new(
            StandardLibraryConfig::default(),
            StandardLibraryPaths {
                managed_root: std::path::PathBuf::from("/tmp/stdlib"),
                metadata_path: std::path::PathBuf::from("/tmp/stdlib/metadata.toml"),
            },
            Some("bundled".to_string()),
            None,
            Vec::new(),
        );
        assert_eq!(provider.custom_method_names(), vec![METHOD.to_string()]);
    }

    #[test]
    fn kpar_dto_marks_bundled_path_available() {
        let config = KparLibraryConfig {
            id: "method".to_string(),
            display_name: "Method libraries".to_string(),
            version: "0.2.0".to_string(),
            repo: EMBEDDED_KPAR_LIBRARY_REPO.to_string(),
            content_path: String::new(),
            format: "kpar".to_string(),
            artifact: Some("elan8-method-libraries-0.2.0.kpar".to_string()),
            resource: None,
        };
        let paths = kpar_library_paths_from_data_dir(std::path::Path::new("/tmp/data"), "method");
        let component = KparLibraryComponent {
            id: "method".to_string(),
            display_name: "Method libraries".to_string(),
            path: Some(std::path::PathBuf::from(
                "/tmp/data/kpar-libraries/method/versions/0.2.0",
            )),
            source: Some("bundled".to_string()),
            config,
            paths,
        };
        let dto = build_kpar_dto(&component).expect("dto");
        assert_eq!(dto.id, "method");
        assert_eq!(dto.pinned_version, "0.2.0");
        assert_eq!(dto.source_kind, "bundled");
        assert!(dto.available);
        assert_eq!(dto.format, "kpar");
    }
}
