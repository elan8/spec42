//! Build script: embed SysML standard library and domain libraries for `include_bytes!`.
//!
//! Override inputs:
//! - `SPEC42_STDLIB_KPAR_DIR`: directory of OMG `.kpar` archives for the pinned stdlib tag.
//! - `SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP`: path to a `.kpar` bundle.
//! - `SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR`: pack a KPAR on the fly from a checkout.

use std::fs::{self, File};
use std::io::{Cursor, Write};
use std::path::{Path, PathBuf};
use std::process;

use kpar::pack::{build_kpar, PackOptions};
use kpar::schema::Project;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use zip::read::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};

const STDLIB_KPAR_EMBED_PREFIX: &str = "bundled-sysml-kpar/";

#[derive(Debug, Deserialize)]
struct LibraryBundleConfig {
    version: String,
    repo: String,
    #[serde(rename = "contentPath")]
    content_path: String,
    #[serde(default = "default_kpar_format")]
    format: String,
    #[serde(default)]
    artifact: Option<String>,
}

fn default_kpar_format() -> String {
    "kpar".to_string()
}

type StandardLibraryConfig = LibraryBundleConfig;
type DomainLibrariesConfig = LibraryBundleConfig;

fn main() {
    embed_stdlib();
    embed_domain_libraries();
}

fn embed_stdlib() {
    let config = load_stdlib_config();
    println!("cargo:rustc-env=SPEC42_STDLIB_VERSION={}", config.version);
    println!("cargo:rustc-env=SPEC42_STDLIB_REPO={}", config.repo);
    println!(
        "cargo:rustc-env=SPEC42_STDLIB_CONTENT_PATH={}",
        config.content_path
    );
    println!("cargo:rustc-env=SPEC42_STDLIB_FORMAT={}", config.format);
    println!("cargo:rerun-if-env-changed=SPEC42_STDLIB_KPAR_DIR");
    println!("cargo:rerun-if-changed=build.rs");

    let rerun_path = format!("../../.cache/sysml-stdlib-kpar-{}", config.version);
    println!("cargo:rerun-if-changed={rerun_path}");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_zip = Path::new(&out_dir).join("sysml.library.embedded.zip");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_STDLIB").is_ok();
    if !embed_enabled {
        fs::write(&out_zip, []).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to write empty stub zip: {e}");
            process::exit(1);
        });
        return;
    }

    let Some(kpar_dir) = resolve_stdlib_kpar_dir(&config.version) else {
        if out_zip.exists() && embedded_stdlib_archive_is_usable(&out_zip) {
            eprintln!(
                "workspace build: reusing cached embedded stdlib archive at {}",
                out_zip.display()
            );
            return;
        }
        if let Some(cached_embedded_zip) = find_cached_embedded_zip(&out_zip) {
            fs::copy(&cached_embedded_zip, &out_zip).unwrap_or_else(|e| {
                eprintln!(
                    "workspace build: failed to reuse cached embedded stdlib archive {}: {e}",
                    cached_embedded_zip.display()
                );
                process::exit(1);
            });
            eprintln!(
                "workspace build: reused cached embedded stdlib archive from {}",
                cached_embedded_zip.display()
            );
            return;
        }

        eprintln!(
            "workspace build: embedded stdlib requires local KPAR archives for {}.",
            config.version
        );
        eprintln!(
            "workspace build: set SPEC42_STDLIB_KPAR_DIR or place .kpar files at .cache/sysml-stdlib-kpar-{}/.",
            config.version
        );
        eprintln!("workspace build: run scripts/fetch-stdlib-bundle.sh");
        process::exit(1);
    };

    embed_stdlib_from_kpar_dir(&kpar_dir, &out_zip).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to embed standard library KPAR: {e}");
        process::exit(1);
    });

    let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));
}

fn embed_domain_libraries() {
    let config = load_domain_libraries_config();
    println!(
        "cargo:rustc-env=SPEC42_DOMAIN_LIBRARIES_VERSION={}",
        config.version
    );
    println!(
        "cargo:rustc-env=SPEC42_DOMAIN_LIBRARIES_REPO={}",
        config.repo
    );
    println!(
        "cargo:rustc-env=SPEC42_DOMAIN_LIBRARIES_CONTENT_PATH={}",
        config.content_path
    );
    println!(
        "cargo:rustc-env=SPEC42_DOMAIN_LIBRARIES_FORMAT={}",
        config.format
    );
    println!("cargo:rerun-if-env-changed=SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP");
    println!("cargo:rerun-if-env-changed=SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR");

    let cache_name = domain_cache_filename(&config);
    let rerun_path = format!("../../.cache/{cache_name}");
    println!("cargo:rerun-if-changed={rerun_path}");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_kpar = Path::new(&out_dir).join("domain-libraries.embedded.kpar");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_DOMAIN_LIBRARIES").is_ok();
    if !embed_enabled {
        fs::write(&out_kpar, []).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to write empty domain libraries stub: {e}");
            process::exit(1);
        });
        return;
    }

    if out_kpar.exists() && domain_embedded_kpar_is_usable(&out_kpar) {
        eprintln!(
            "workspace build: reusing cached embedded domain libraries KPAR at {}",
            out_kpar.display()
        );
        return;
    }

    if let Some(source_dir) = resolve_domain_libraries_source_dir() {
        pack_domain_kpar_from_dir(&source_dir, &config, &out_kpar).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to pack domain libraries from directory: {e}");
            process::exit(1);
        });
        return;
    }

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let sibling = manifest_dir.join("../../../sysml-domain-libraries");
    if sibling.is_dir() {
        pack_domain_kpar_from_dir(&sibling, &config, &out_kpar).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to pack domain libraries from sibling checkout: {e}");
            process::exit(1);
        });
        eprintln!(
            "workspace build: packed domain libraries KPAR from sibling checkout {}",
            sibling.display()
        );
        return;
    }

    let Some(local_bundle) = resolve_domain_libraries_bundle(&cache_name) else {
        if let Some(cached) = find_cached_domain_embedded_kpar(&out_kpar) {
            fs::copy(&cached, &out_kpar).unwrap_or_else(|e| {
                eprintln!(
                    "workspace build: failed to reuse cached embedded domain KPAR {}: {e}",
                    cached.display()
                );
                process::exit(1);
            });
            eprintln!(
                "workspace build: reused cached embedded domain KPAR from {}",
                cached.display()
            );
            return;
        }

        eprintln!(
            "workspace build: embedded domain libraries require a local KPAR bundle for {}.",
            config.version
        );
        eprintln!(
            "workspace build: set SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP, SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR, or place a bundle at .cache/{cache_name}."
        );
        eprintln!("workspace build: run scripts/fetch-domain-libraries-bundle.sh");
        process::exit(1);
    };

    if local_bundle.extension().is_none_or(|ext| ext != "kpar") {
        eprintln!(
            "workspace build: expected a .kpar bundle at {}",
            local_bundle.display()
        );
        process::exit(1);
    }

    fs::copy(&local_bundle, &out_kpar).unwrap_or_else(|e| {
        eprintln!(
            "workspace build: failed to copy domain KPAR {}: {e}",
            local_bundle.display()
        );
        process::exit(1);
    });
}

fn domain_cache_filename(config: &DomainLibrariesConfig) -> String {
    config
        .artifact
        .clone()
        .unwrap_or_else(|| format!("elan8-domain-libraries-{}.kpar", config.version))
}

fn pack_domain_kpar_from_dir(
    source_dir: &Path,
    config: &DomainLibrariesConfig,
    out_kpar: &Path,
) -> Result<(), String> {
    let project = Project {
        name: "elan8-domain-libraries".to_string(),
        version: config.version.clone(),
        description: Some("Elan8 SysML v2 domain libraries".to_string()),
        license: Some("MIT".to_string()),
        publisher: Some("elan8".to_string()),
        maintainer: vec![],
        website: None,
        topic: vec![],
        usage: vec![],
    };
    let options = PackOptions::domain_libraries_defaults(project, source_dir);
    build_kpar(&options, out_kpar).map_err(|e| e.to_string())
}

fn load_stdlib_config() -> StandardLibraryConfig {
    load_config("../../config/standard-library.json")
}

fn load_domain_libraries_config() -> DomainLibrariesConfig {
    load_config("../../config/domain-libraries.json")
}

fn load_config(relative: &str) -> LibraryBundleConfig {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join(relative);
    println!("cargo:rerun-if-changed={}", config_path.display());
    let raw = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!(
            "workspace build: failed to read {}: {e}",
            config_path.display()
        );
        process::exit(1);
    });
    serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!(
            "workspace build: failed to parse {}: {e}",
            config_path.display()
        );
        process::exit(1);
    })
}

fn embed_cache_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .join("../../.cache")
}

fn resolve_stdlib_kpar_dir(version: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_STDLIB_KPAR_DIR") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            if stdlib_kpar_dir_is_usable(&candidate) {
                return Some(candidate);
            }
        }
    }
    let cached = embed_cache_dir().join(format!("sysml-stdlib-kpar-{version}"));
    stdlib_kpar_dir_is_usable(&cached).then_some(cached)
}

fn stdlib_kpar_dir_is_usable(path: &Path) -> bool {
    if !path.is_dir() {
        return false;
    }
    fs::read_dir(path)
        .ok()
        .into_iter()
        .flatten()
        .flatten()
        .any(|entry| entry.path().extension().is_some_and(|ext| ext == "kpar"))
}

fn resolve_domain_libraries_bundle(cache_name: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    let cached = embed_cache_dir().join(cache_name);
    cached.is_file().then_some(cached)
}

fn resolve_domain_libraries_source_dir() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            let candidate = PathBuf::from(trimmed);
            if candidate.is_dir() {
                return Some(candidate);
            }
        }
    }
    None
}

fn find_cached_embedded_zip(out_zip: &Path) -> Option<PathBuf> {
    let build_root = out_zip.parent()?.parent()?.parent()?;
    for entry in fs::read_dir(build_root).ok()?.flatten() {
        let candidate = entry.path().join("out/sysml.library.embedded.zip");
        if candidate != out_zip
            && candidate.is_file()
            && embedded_stdlib_archive_is_usable(&candidate)
        {
            return Some(candidate);
        }
    }
    None
}

fn find_cached_domain_embedded_kpar(out_kpar: &Path) -> Option<PathBuf> {
    let build_root = out_kpar.parent()?.parent()?.parent()?;
    for entry in fs::read_dir(build_root).ok()?.flatten() {
        let candidate = entry.path().join("out/domain-libraries.embedded.kpar");
        if candidate != out_kpar
            && candidate.is_file()
            && domain_embedded_kpar_is_usable(&candidate)
        {
            return Some(candidate);
        }
    }
    None
}

fn embedded_stdlib_archive_is_usable(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let Ok(mut archive) = ZipArchive::new(Cursor::new(bytes)) else {
        return false;
    };
    for i in 0..archive.len() {
        let Ok(entry) = archive.by_index(i) else {
            return false;
        };
        let name = entry.name();
        if name.starts_with(STDLIB_KPAR_EMBED_PREFIX)
            && name.ends_with(".kpar")
            && !name.ends_with('/')
        {
            return true;
        }
    }
    false
}

fn domain_embedded_kpar_is_usable(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    !bytes.is_empty() && kpar::is_kpar_archive(&bytes)
}

fn embed_stdlib_from_kpar_dir(kpar_dir: &Path, out_path: &Path) -> Result<(), String> {
    let mut kpar_files: Vec<PathBuf> = fs::read_dir(kpar_dir)
        .map_err(|e| format!("read {}: {e}", kpar_dir.display()))?
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "kpar"))
        .collect();
    kpar_files.sort();
    if kpar_files.is_empty() {
        return Err(format!("no .kpar files found in {}", kpar_dir.display()));
    }

    let out_file =
        File::create(out_path).map_err(|e| format!("create {}: {e}", out_path.display()))?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();
    for path in kpar_files {
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| format!("invalid kpar file name {}", path.display()))?;
        let out_name = format!("{STDLIB_KPAR_EMBED_PREFIX}{file_name}");
        writer
            .start_file(&out_name, options)
            .map_err(|e| format!("start_file {out_name}: {e}"))?;
        let bytes = fs::read(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
        writer
            .write_all(&bytes)
            .map_err(|e| format!("write {out_name}: {e}"))?;
    }
    writer.finish().map_err(|e| format!("finish zip: {e}"))?;
    Ok(())
}
