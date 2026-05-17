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

use sha2::{Digest, Sha256};
use zip::read::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};

const DEFAULT_TAG: &str = "2026-03";
/// Single top-level folder prefix inside the embedded zip (must match `extract_archive_subset` in stdlib.rs).
const EMBED_ROOT: &str = "bundled-sysml-release";
const LOCAL_CACHE_RELATIVE_PATH: &str = "cache/sysml-v2-release-2026-03.zip";

fn main() {
    println!("cargo:rerun-if-env-changed=SPEC42_STDLIB_BUNDLE_ZIP");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={LOCAL_CACHE_RELATIVE_PATH}");

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

    let Some(local_zip) = resolve_local_stdlib_zip() else {
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
            "spec42 build: embedded stdlib requires a local SysML v2 Release {DEFAULT_TAG} zip."
        );
        eprintln!("spec42 build: set SPEC42_STDLIB_BUNDLE_ZIP to the full release zip path, or place it at crates/server/{LOCAL_CACHE_RELATIVE_PATH}.");
        eprintln!("spec42 build: download URL: https://github.com/Systems-Modeling/SysML-v2-Release/archive/refs/tags/{DEFAULT_TAG}.zip");
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
}

fn resolve_local_stdlib_zip() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("SPEC42_STDLIB_BUNDLE_ZIP") {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }

    let manifest_dir =
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let cached = manifest_dir.join(LOCAL_CACHE_RELATIVE_PATH);
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
