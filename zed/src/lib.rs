use std::fs;

use zed::settings::LspSettings;
use zed::{
    current_platform, download_file, github_release_by_tag_name, make_file_executable,
    set_language_server_installation_status, Architecture, Command, DownloadedFileType,
    GithubReleaseAsset, GithubReleaseOptions, LanguageServerId, LanguageServerInstallationStatus,
    Os, Result, Worktree,
};
use zed_extension_api as zed;

const GITHUB_REPO: &str = "elan8/spec42";
const SERVER_NAME: &str = "spec42";

struct Spec42Extension;

struct PlatformRelease {
    package_dir: &'static str,
    archive_ext: &'static str,
    binary_name: &'static str,
    downloaded_file_type: DownloadedFileType,
}

impl Spec42Extension {
    fn lsp_settings(&self, worktree: &Worktree) -> LspSettings {
        LspSettings::for_worktree(SERVER_NAME, worktree).unwrap_or_default()
    }

    fn release_tag(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    fn release_version(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }

    fn release_for_platform(&self) -> Result<PlatformRelease> {
        match current_platform() {
            (Os::Linux, Architecture::X8664) => Ok(PlatformRelease {
                package_dir: "linux-x64",
                archive_ext: "tar.gz",
                binary_name: SERVER_NAME,
                downloaded_file_type: DownloadedFileType::GzipTar,
            }),
            (Os::Mac, Architecture::X8664) => Ok(PlatformRelease {
                package_dir: "darwin-x64",
                archive_ext: "tar.gz",
                binary_name: SERVER_NAME,
                downloaded_file_type: DownloadedFileType::GzipTar,
            }),
            (Os::Mac, Architecture::Aarch64) => Ok(PlatformRelease {
                package_dir: "darwin-arm64",
                archive_ext: "tar.gz",
                binary_name: SERVER_NAME,
                downloaded_file_type: DownloadedFileType::GzipTar,
            }),
            (Os::Windows, Architecture::X8664) => Ok(PlatformRelease {
                package_dir: "win32-x64",
                archive_ext: "zip",
                binary_name: "spec42.exe",
                downloaded_file_type: DownloadedFileType::Zip,
            }),
            (os, arch) => Err(format!(
                "spec42 does not publish a bundled server for platform {os:?}/{arch:?}"
            )),
        }
    }

    fn cached_binary_path(&self, release: &PlatformRelease) -> String {
        format!(
            "spec42-{}-{}/{}",
            self.release_version(),
            release.package_dir,
            release.binary_name
        )
    }

    fn release_asset_name(&self, release: &PlatformRelease) -> String {
        format!(
            "spec42-{}-{}.{}",
            self.release_version(),
            release.package_dir,
            release.archive_ext
        )
    }

    fn find_release_asset<'a>(
        &self,
        assets: &'a [GithubReleaseAsset],
        asset_name: &str,
    ) -> Result<&'a GithubReleaseAsset> {
        assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("release asset '{asset_name}' not found in {GITHUB_REPO}"))
    }

    fn ensure_downloaded_binary(
        &self,
        language_server_id: &LanguageServerId,
    ) -> Result<String> {
        let release = self.release_for_platform()?;
        let binary_path = self.cached_binary_path(&release);

        if fs::metadata(&binary_path)
            .map(|metadata| metadata.is_file())
            .unwrap_or(false)
        {
            if !matches!(current_platform(), (Os::Windows, _)) {
                let _ = make_file_executable(&binary_path);
            }
            return Ok(binary_path);
        }

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        let download_result: Result<String> = (|| {
            let tag = self.release_tag();
            let github_release = github_release_by_tag_name(GITHUB_REPO, &tag)
                .or_else(|_| {
                    zed::latest_github_release(
                        GITHUB_REPO,
                        GithubReleaseOptions {
                            require_assets: true,
                            pre_release: false,
                        },
                    )
                })
                .map_err(|err| format!("failed to resolve GitHub release for spec42: {err}"))?;
            let asset_name = self.release_asset_name(&release);
            let asset = self.find_release_asset(&github_release.assets, &asset_name)?;
            let extraction_dir = format!("spec42-{}-{}", self.release_version(), release.package_dir);

            download_file(
                &asset.download_url,
                &extraction_dir,
                release.downloaded_file_type,
            )
            .map_err(|err| format!("failed to download {asset_name}: {err}"))?;

            if !matches!(current_platform(), (Os::Windows, _)) {
                make_file_executable(&binary_path)
                    .map_err(|err| format!("failed to mark {binary_path} executable: {err}"))?;
            }

            Ok(binary_path)
        })();

        match download_result {
            Ok(binary_path) => {
                set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::None,
                );
                Ok(binary_path)
            }
            Err(err) => {
                set_language_server_installation_status(
                    language_server_id,
                    &LanguageServerInstallationStatus::Failed(err.clone()),
                );
                Err(err)
            }
        }
    }
}

impl zed::Extension for Spec42Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        let settings = self.lsp_settings(worktree);
        let binary = settings.binary;

        let command = if let Some(path) = binary.as_ref().and_then(|binary| binary.path.clone()) {
            path
        } else if let Some(path) = worktree.which(SERVER_NAME) {
            path
        } else {
            self.ensure_downloaded_binary(language_server_id)?
        };

        Ok(Command {
            command,
            args: binary
                .as_ref()
                .and_then(|binary| binary.arguments.clone())
                .unwrap_or_default(),
            env: binary
                .and_then(|binary| binary.env)
                .unwrap_or_default()
                .into_iter()
                .collect(),
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(self.lsp_settings(worktree).initialization_options)
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(self.lsp_settings(worktree).settings)
    }
}

zed::register_extension!(Spec42Extension);
