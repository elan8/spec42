use std::sync::Arc;

#[tokio::main]
async fn main() {
    let mut config = lsp_server::default_server_config();
    let stdlib = std::env::var_os("SPEC42_LSP_TEST_STDLIB").map(std::path::PathBuf::from);
    let no_stdlib = std::env::var_os("SPEC42_LSP_TEST_NO_STDLIB").is_some();
    if stdlib.is_some() || no_stdlib {
        let catalog =
            library_catalog::resolve_library_catalog(&library_catalog::HostLibraryRequest {
                cache_dir: stdlib
                    .as_ref()
                    .map(|root| root.join(".cache"))
                    .unwrap_or_else(std::env::temp_dir),
                no_stdlib,
                stdlib_path_override: stdlib,
                kpar_library_path_overrides: Default::default(),
                project_library_paths: Default::default(),
                disabled_kpar_libraries: Default::default(),
                library_paths: Vec::new(),
                standard_library: Default::default(),
                use_embedded_stdlib: false,
                use_embedded_kpar_libraries: false,
                config_stdlib_path: None,
                config_no_stdlib: false,
                extra_library_paths: Vec::new(),
            })
            .expect("test standard-library catalog");
        config = config
            .with_default_library_paths(catalog.package_roots.clone())
            .with_standard_library_paths(catalog.stdlib.roots.clone())
            .with_project_library_catalog(catalog);
    }
    let config = Arc::new(config);
    lsp_server::run_lsp(config, "spec42-core-test").await;
}
