//! Build script: repack a local SysML v2 Release zip into a minimal `sysml.library/`
//! archive for `include_bytes!` (see `stdlib.rs`).
//!
//! Override inputs:
//! - `SPEC42_STDLIB_BUNDLE_ZIP`: path to a local copy of the **full** GitHub release zip.
//! - `CARGO_FEATURE_EMBED_STDLIB`: unset when `embed-stdlib` feature is disabled — writes an empty file.

use std::fs::{self, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process;

use serde::Deserialize;
use sha2::{Digest, Sha256};
use zip::read::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};

/// Single top-level folder prefix inside the embedded zip (must match `extract_archive_subset` in stdlib.rs).
const EMBED_ROOT: &str = "bundled-sysml-release";
const DOMAIN_EMBED_ROOT: &str = "bundled-domain-libraries";

#[derive(Debug, Deserialize)]
struct LibraryBundleConfig {
    version: String,
    repo: String,
    #[serde(rename = "contentPath")]
    content_path: String,
}

type StandardLibraryConfig = LibraryBundleConfig;
type DomainLibrariesConfig = LibraryBundleConfig;

fn main() {
    let config = load_stdlib_config();
    println!("cargo:rustc-env=SPEC42_STDLIB_VERSION={}", config.version);
    println!("cargo:rustc-env=SPEC42_STDLIB_REPO={}", config.repo);
    println!(
        "cargo:rustc-env=SPEC42_STDLIB_CONTENT_PATH={}",
        config.content_path
    );
    println!("cargo:rerun-if-env-changed=SPEC42_STDLIB_BUNDLE_ZIP");
    println!("cargo:rerun-if-changed=build.rs");

    let local_cache_relative_path = format!("cache/sysml-v2-release-{}.zip", config.version);
    println!("cargo:rerun-if-changed={local_cache_relative_path}");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_zip = Path::new(&out_dir).join("sysml.library.embedded.zip");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_STDLIB").is_ok();
    if !embed_enabled {
        fs::write(&out_zip, []).unwrap_or_else(|e| {
            eprintln!("spec42 build: failed to write empty stub zip: {e}");
            process::exit(1);
        });
        return;
    }

    let Some(local_zip) = resolve_local_stdlib_zip(&local_cache_relative_path) else {
        if out_zip.exists() {
            if embedded_archive_is_usable(&out_zip) {
                eprintln!(
                    "spec42 build: reusing cached embedded stdlib archive at {}",
                    out_zip.display()
                );
                let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));
                return;
            }
            eprintln!(
                "spec42 build: ignoring unusable cached embedded stdlib archive at {}",
                out_zip.display()
            );
        }
        if let Some(cached_embedded_zip) = find_cached_embedded_zip(&out_zip) {
            fs::copy(&cached_embedded_zip, &out_zip).unwrap_or_else(|e| {
                eprintln!(
                    "spec42 build: failed to reuse cached embedded stdlib archive {}: {e}",
                    cached_embedded_zip.display()
                );
                process::exit(1);
            });
            eprintln!(
                "spec42 build: reused cached embedded stdlib archive from {}",
                cached_embedded_zip.display()
            );
            let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));
            return;
        }

        eprintln!(
            "spec42 build: embedded stdlib requires a local SysML v2 Release {} zip.",
            config.version
        );
        eprintln!(
            "spec42 build: set SPEC42_STDLIB_BUNDLE_ZIP to the full release zip path, or place it at crates/server/{local_cache_relative_path}."
        );
        eprintln!(
            "spec42 build: download URL: https://github.com/{}/archive/refs/tags/{}.zip",
            config.repo, config.version
        );
        eprintln!("spec42 build: for development without embedded stdlib, run `cargo test -p spec42 --no-default-features`.");
        process::exit(1);
    };

    let full_zip_bytes = fs::read(&local_zip).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to read stdlib release zip {}: {e}",
            local_zip.display()
        );
        process::exit(1);
    });

    let _digest = format!("{:x}", Sha256::digest(&full_zip_bytes));

    repack_sysml_library(&full_zip_bytes, &out_zip).unwrap_or_else(|e| {
        eprintln!("spec42 build: failed to repack sysml.library: {e}");
        process::exit(1);
    });

    let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));

    embed_domain_libraries();
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
    println!("cargo:rerun-if-env-changed=SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP");
    println!("cargo:rerun-if-env-changed=SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR");

    let local_cache_relative_path =
        format!("cache/sysml-domain-libraries-{}.zip", config.version);
    println!("cargo:rerun-if-changed={local_cache_relative_path}");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR");
    let out_zip = Path::new(&out_dir).join("domain-libraries.embedded.zip");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_DOMAIN_LIBRARIES").is_ok();
    if !embed_enabled {
        fs::write(&out_zip, []).unwrap_or_else(|e| {
            eprintln!("spec42 build: failed to write empty domain libraries stub zip: {e}");
            process::exit(1);
        });
        return;
    }

    if out_zip.exists() && domain_embedded_archive_is_usable(&out_zip) {
        eprintln!(
            "spec42 build: reusing cached embedded domain libraries archive at {}",
            out_zip.display()
        );
        return;
    }

    if let Some(source_dir) = resolve_domain_libraries_source_dir() {
        repack_domain_libraries_from_dir(&source_dir, &config.content_path, &out_zip).unwrap_or_else(
            |e| {
                eprintln!("spec42 build: failed to repack domain libraries from directory: {e}");
                process::exit(1);
            },
        );
        return;
    }

    let Some(local_zip) = resolve_domain_libraries_bundle_zip(&local_cache_relative_path) else {
        if let Some(cached_embedded_zip) = find_cached_domain_embedded_zip(&out_zip) {
            fs::copy(&cached_embedded_zip, &out_zip).unwrap_or_else(|e| {
                eprintln!(
                    "spec42 build: failed to reuse cached embedded domain libraries archive {}: {e}",
                    cached_embedded_zip.display()
                );
                process::exit(1);
            });
            eprintln!(
                "spec42 build: reused cached embedded domain libraries archive from {}",
                cached_embedded_zip.display()
            );
            return;
        }

        eprintln!(
            "spec42 build: embedded domain libraries require a local bundle or source directory for {}.",
            config.version
        );
        eprintln!(
            "spec42 build: set SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP, SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR, place a bundle at crates/server/{local_cache_relative_path}, or check out ../sysml-domain-libraries next to the repo."
        );
        eprintln!(
            "spec42 build: download via scripts/fetch-domain-libraries-bundle.sh"
        );
        eprintln!(
            "spec42 build: for development without embedded domain libraries, run `cargo test -p spec42 --no-default-features`."
        );
        process::exit(1);
    };

    let full_zip_bytes = fs::read(&local_zip).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to read domain libraries bundle {}: {e}",
            local_zip.display()
        );
        process::exit(1);
    });

    repack_domain_libraries_from_zip(&full_zip_bytes, &config.content_path, &out_zip).unwrap_or_else(
        |e| {
            eprintln!("spec42 build: failed to repack domain libraries: {e}");
            process::exit(1);
        },
    );
}

fn load_stdlib_config() -> StandardLibraryConfig {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../config/standard-library.json");
    println!("cargo:rerun-if-changed={}", config_path.display());
    let raw = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to read {}: {e}",
            config_path.display()
        );
        process::exit(1);
    });
    serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to parse {}: {e}",
            config_path.display()
        );
        process::exit(1);
    })
}

fn resolve_local_stdlib_zip(local_cache_relative_path: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_STDLIB_BUNDLE_ZIP") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let cached = manifest_dir.join(local_cache_relative_path);
    cached.exists().then_some(cached)
}

fn find_cached_embedded_zip(out_zip: &Path) -> Option<PathBuf> {
    let build_root = out_zip.parent()?.parent()?.parent()?;
    let entries = fs::read_dir(build_root).ok()?;
    for entry in entries.flatten() {
        let candidate = entry.path().join("out/sysml.library.embedded.zip");
        if candidate != out_zip && candidate.is_file() && embedded_archive_is_usable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

fn embedded_archive_is_usable(path: &Path) -> bool {
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
        if name.starts_with(&format!("{EMBED_ROOT}/sysml.library/")) && !name.ends_with('/') {
            return true;
        }
    }
    false
}

fn repack_sysml_library(full_zip_bytes: &[u8], out_path: &Path) -> Result<(), String> {
    let cursor = Cursor::new(full_zip_bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("open release zip: {e}"))?;
    if archive.is_empty() {
        return Err("release zip is empty".to_string());
    }

    let root = {
        let first = archive.by_index(0).map_err(|e| format!("read zip: {e}"))?;
        first
            .name()
            .split('/')
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "malformed zip: missing root folder".to_string())?
            .to_string()
    };

    let wanted_prefix = format!("{root}/sysml.library/");
    let out_file =
        File::create(out_path).map_err(|e| format!("create {}: {e}", out_path.display()))?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();

    let mut count = 0usize;
    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("zip entry {i}: {e}"))?;
        let name = entry.name().to_string();
        if !name.starts_with(&wanted_prefix) || name.ends_with('/') {
            continue;
        }
        let relative = name.trim_start_matches(&wanted_prefix);
        if relative.is_empty() {
            continue;
        }
        let out_name = format!("{EMBED_ROOT}/sysml.library/{relative}");
        writer
            .start_file(&out_name, options)
            .map_err(|e| format!("start_file {out_name}: {e}"))?;
        std::io::copy(&mut entry, &mut writer).map_err(|e| format!("copy {out_name}: {e}"))?;
        count += 1;
    }
    writer.finish().map_err(|e| format!("finish zip: {e}"))?;
    if count == 0 {
        return Err(format!(
            "no files found under prefix '{wanted_prefix}' in release zip"
        ));
    }
    let _count = count;
    Ok(())
}

fn load_domain_libraries_config() -> DomainLibrariesConfig {
    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let config_path = manifest_dir.join("../../config/domain-libraries.json");
    println!("cargo:rerun-if-changed={}", config_path.display());
    let raw = fs::read_to_string(&config_path).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to read {}: {e}",
            config_path.display()
        );
        process::exit(1);
    });
    serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!(
            "spec42 build: failed to parse {}: {e}",
            config_path.display()
        );
        process::exit(1);
    })
}

fn resolve_domain_libraries_bundle_zip(local_cache_relative_path: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let cached = manifest_dir.join(local_cache_relative_path);
    cached.exists().then_some(cached)
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

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let git_sibling = manifest_dir.join("../../../sysml-domain-libraries");
    git_sibling.is_dir().then_some(git_sibling)
}

fn find_cached_domain_embedded_zip(out_zip: &Path) -> Option<PathBuf> {
    let build_root = out_zip.parent()?.parent()?.parent()?;
    let entries = fs::read_dir(build_root).ok()?;
    for entry in entries.flatten() {
        let candidate = entry.path().join("out/domain-libraries.embedded.zip");
        if candidate != out_zip
            && candidate.is_file()
            && domain_embedded_archive_is_usable(&candidate)
        {
            return Some(candidate);
        }
    }
    None
}

fn domain_embedded_archive_is_usable(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    let Ok(mut archive) = ZipArchive::new(Cursor::new(bytes)) else {
        return false;
    };
    let prefix = format!("{DOMAIN_EMBED_ROOT}/");
    for i in 0..archive.len() {
        let Ok(entry) = archive.by_index(i) else {
            return false;
        };
        let name = entry.name();
        if name.starts_with(&prefix) && !name.ends_with('/') {
            return true;
        }
    }
    false
}

fn repack_domain_libraries_from_zip(
    full_zip_bytes: &[u8],
    content_path: &str,
    out_path: &Path,
) -> Result<(), String> {
    let cursor = Cursor::new(full_zip_bytes);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("open domain bundle zip: {e}"))?;
    if archive.is_empty() {
        return Err("domain bundle zip is empty".to_string());
    }

    let source_root = {
        let first = archive.by_index(0).map_err(|e| format!("read zip: {e}"))?;
        first
            .name()
            .split('/')
            .next()
            .filter(|s| !s.is_empty())
            .ok_or_else(|| "malformed zip: missing root folder".to_string())?
            .to_string()
    };

    let source_prefix = format!("{source_root}/");
    let out_file =
        File::create(out_path).map_err(|e| format!("create {}: {e}", out_path.display()))?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();
    let out_prefix = format!("{DOMAIN_EMBED_ROOT}/{content_path}/");
    let mut count = 0usize;

    for i in 0..archive.len() {
        let mut entry = archive
            .by_index(i)
            .map_err(|e| format!("zip entry {i}: {e}"))?;
        let name = entry.name().to_string();
        if !name.starts_with(&source_prefix) || name.ends_with('/') {
            continue;
        }
        let relative = name.trim_start_matches(&source_prefix);
        if relative.is_empty() || relative.starts_with(".git/") {
            continue;
        }
        let out_name = format!("{out_prefix}{relative}");
        writer
            .start_file(&out_name, options)
            .map_err(|e| format!("start_file {out_name}: {e}"))?;
        std::io::copy(&mut entry, &mut writer).map_err(|e| format!("copy {out_name}: {e}"))?;
        count += 1;
    }
    writer.finish().map_err(|e| format!("finish zip: {e}"))?;
    if count == 0 {
        return Err(format!(
            "no files found under prefix '{source_prefix}' in domain bundle zip"
        ));
    }
    Ok(())
}

fn repack_domain_libraries_from_dir(
    source_dir: &Path,
    content_path: &str,
    out_path: &Path,
) -> Result<(), String> {
    let out_file =
        File::create(out_path).map_err(|e| format!("create {}: {e}", out_path.display()))?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();
    let out_prefix = format!("{DOMAIN_EMBED_ROOT}/{content_path}/");
    let mut count = 0usize;
    collect_domain_files(source_dir, source_dir, &out_prefix, &mut writer, options, &mut count)?;
    writer.finish().map_err(|e| format!("finish zip: {e}"))?;
    if count == 0 {
        return Err(format!(
            "no files found under source directory {}",
            source_dir.display()
        ));
    }
    Ok(())
}

fn collect_domain_files(
    source_root: &Path,
    current: &Path,
    out_prefix: &str,
    writer: &mut ZipWriter<File>,
    options: SimpleFileOptions,
    count: &mut usize,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|e| format!("read_dir {}: {e}", current.display()))?
    {
        let entry = entry.map_err(|e| format!("dir entry: {e}"))?;
        let path = entry.path();
        if path.file_name().is_some_and(|name| name == ".git") {
            continue;
        }
        if path.is_dir() {
            collect_domain_files(source_root, &path, out_prefix, writer, options, count)?;
            continue;
        }
        if !path.is_file() {
            continue;
        }
        let relative = path
            .strip_prefix(source_root)
            .map_err(|_| "strip_prefix failed".to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let out_name = format!("{out_prefix}{relative}");
        writer
            .start_file(&out_name, options)
            .map_err(|e| format!("start_file {out_name}: {e}"))?;
        let mut file =
            File::open(&path).map_err(|e| format!("open {}: {e}", path.display()))?;
        std::io::copy(&mut file, writer).map_err(|e| format!("copy {out_name}: {e}"))?;
        *count += 1;
    }
    Ok(())
}
