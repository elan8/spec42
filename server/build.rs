//! Build script: download SysML v2 Release tag zip, repack only `sysml.library/` into a minimal
//! archive for `include_bytes!` (see `stdlib.rs`).
//!
//! Override inputs:
//! - `SPEC42_STDLIB_BUNDLE_ZIP`: path to a local copy of the **full** GitHub release zip (skips download).
//! - `CARGO_FEATURE_EMBED_STDLIB`: unset when `embed-stdlib` feature is disabled — writes an empty file.

use std::fs::{self, File};
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::process;

use sha2::{Digest, Sha256};
use zip::read::ZipArchive;
use zip::write::{SimpleFileOptions, ZipWriter};

const DEFAULT_TAG: &str = "2026-02";
const DOWNLOAD_URL: &str =
    "https://codeload.github.com/Systems-Modeling/SysML-v2-Release/zip/refs/tags/2026-02";
/// Single top-level folder prefix inside the embedded zip (must match `extract_archive_subset` in stdlib.rs).
const EMBED_ROOT: &str = "bundled-sysml-release";

fn main() {
    println!("cargo:rerun-if-env-changed=SPEC42_STDLIB_BUNDLE_ZIP");
    println!("cargo:rerun-if-changed=build.rs");

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

    let full_zip_bytes: Vec<u8> = if let Ok(path) = std::env::var("SPEC42_STDLIB_BUNDLE_ZIP") {
        let p = PathBuf::from(path.trim());
        fs::read(&p).unwrap_or_else(|e| {
            eprintln!(
                "spec42 build: failed to read SPEC42_STDLIB_BUNDLE_ZIP {}: {e}",
                p.display()
            );
            process::exit(1);
        })
    } else {
        eprintln!("spec42 build: downloading SysML v2 Release {DEFAULT_TAG} for embedded stdlib...");
        {
            let mut reader = ureq::get(DOWNLOAD_URL)
                .set("User-Agent", "spec42-build")
                .call()
                .unwrap_or_else(|e| {
                    eprintln!("spec42 build: failed to download release zip: {e}");
                    eprintln!(
                        "spec42 build: set SPEC42_STDLIB_BUNDLE_ZIP to a local zip to build offline."
                    );
                    process::exit(1);
                })
                .into_reader();
            let mut out = Vec::new();
            std::io::Read::read_to_end(&mut reader, &mut out).unwrap_or_else(|e| {
                eprintln!("spec42 build: failed to read release zip body: {e}");
                process::exit(1);
            });
            out
        }
    };

    let _digest = format!("{:x}", Sha256::digest(&full_zip_bytes));

    repack_sysml_library(&full_zip_bytes, &out_zip).unwrap_or_else(|e| {
        eprintln!("spec42 build: failed to repack sysml.library: {e}");
        process::exit(1);
    });

    let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));
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
    let out_file = File::create(out_path).map_err(|e| format!("create {}: {e}", out_path.display()))?;
    let mut writer = ZipWriter::new(out_file);
    let options = SimpleFileOptions::default();

    let mut count = 0usize;
    for i in 0..archive.len() {
        let mut entry = archive.by_index(i).map_err(|e| format!("zip entry {i}: {e}"))?;
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
