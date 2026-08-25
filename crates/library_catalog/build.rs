//! Build script: embed OMG stdlib and config-driven Elan8 KPAR libraries.

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
struct StdlibConfig {
    version: String,
    repo: String,
    #[serde(rename = "contentPath")]
    content_path: String,
    #[serde(default = "default_kpar_format")]
    format: String,
    projects: Vec<StdlibProjectConfig>,
}

#[derive(Debug, Deserialize)]
struct StdlibProjectConfig {
    archive: String,
    name: String,
    version: String,
    resources: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct PackConfig {
    kind: String,
    #[serde(rename = "siblingRelative")]
    sibling_relative: String,
    #[serde(rename = "archivePrefix", default)]
    archive_prefix: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
struct KparLibraryFile {
    #[serde(default)]
    id: Option<String>,
    #[serde(rename = "displayName")]
    display_name: String,
    version: String,
    repo: String,
    #[serde(rename = "contentPath", default)]
    content_path: String,
    #[serde(default = "default_kpar_format")]
    format: String,
    #[serde(default)]
    artifact: Option<String>,
    #[serde(default)]
    resource: Option<String>,
    pack: PackConfig,
}

fn default_kpar_format() -> String {
    "kpar".to_string()
}

fn main() {
    embed_stdlib();
    embed_kpar_libraries();
}

fn embed_stdlib() {
    let config = load_stdlib_config();
    write_stdlib_project_registry(&config);
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

    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let out_zip = out_dir.join("sysml.library.embedded.zip");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_STDLIB").is_ok();
    if !embed_enabled {
        write_empty_stub(&out_zip, "stdlib stub");
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
            copy_file(
                &cached_embedded_zip,
                &out_zip,
                "cached embedded stdlib archive",
            );
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

    validate_stdlib_projects(&config, &kpar_dir).unwrap_or_else(|e| {
        eprintln!("workspace build: standard-library project config drift: {e}");
        process::exit(1);
    });
    embed_stdlib_from_kpar_dir(&kpar_dir, &out_zip).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to embed standard library KPAR: {e}");
        process::exit(1);
    });

    let _embedded_digest = format!("{:x}", Sha256::digest(fs::read(&out_zip).unwrap()));
}

fn validate_stdlib_projects(config: &StdlibConfig, directory: &Path) -> Result<(), String> {
    let declared: std::collections::BTreeSet<_> = config
        .projects
        .iter()
        .map(|project| project.archive.as_str())
        .collect();
    let actual: std::collections::BTreeSet<_> = fs::read_dir(directory)
        .map_err(|error| format!("read {}: {error}", directory.display()))?
        .flatten()
        .filter_map(|entry| {
            let path = entry.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("kpar"))
                .then(|| entry.file_name().to_string_lossy().into_owned())
        })
        .collect();
    let actual_refs: std::collections::BTreeSet<_> = actual.iter().map(String::as_str).collect();
    if declared != actual_refs {
        return Err(format!(
            "declared archives {declared:?}, actual archives {actual_refs:?}"
        ));
    }
    for declared in &config.projects {
        let archive = kpar::open_kpar_path(&directory.join(&declared.archive))
            .map_err(|error| format!("{}: {error}", declared.archive))?;
        if archive.project.name != declared.name || archive.project.version != declared.version {
            return Err(format!(
                "{} declares {} {}, archive contains {} {}",
                declared.archive,
                declared.name,
                declared.version,
                archive.project.name,
                archive.project.version
            ));
        }
    }
    Ok(())
}

fn write_stdlib_project_registry(config: &StdlibConfig) {
    let out = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"))
        .join("stdlib_projects_registry.rs");
    let json = serde_json::to_string(
        &config
            .projects
            .iter()
            .map(|project| {
                serde_json::json!({"archive": project.archive, "name": project.name,
            "version": project.version, "resources": project.resources})
            })
            .collect::<Vec<_>>(),
    )
    .expect("serialize stdlib projects");
    fs::write(
        out,
        format!("pub const STDLIB_PROJECTS_JSON: &str = r#\"{json}\"#;\n"),
    )
    .expect("write stdlib project registry");
}

fn embed_kpar_libraries() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").expect("OUT_DIR"));
    let registry_path = out_dir.join("kpar_libraries_registry.rs");
    let libraries_dir = config_libraries_dir();
    println!("cargo:rerun-if-changed={}", libraries_dir.display());
    println!("cargo:rerun-if-changed=build.rs");

    let embed_enabled = std::env::var("CARGO_FEATURE_EMBED_KPAR_LIBRARIES").is_ok();
    let mut entries = load_library_configs(&libraries_dir);
    entries.sort_by(|a, b| a.id.cmp(&b.id));

    if !embed_enabled {
        for entry in &entries {
            write_empty_stub(
                &out_dir.join(format!("{}.embedded.kpar", entry.id)),
                &format!("{} stub", entry.id),
            );
        }
        write_registry(&registry_path, &[], false);
        return;
    }

    let mut embedded = Vec::new();
    for entry in &entries {
        let out_kpar = out_dir.join(format!("{}.embedded.kpar", entry.id));
        println!(
            "cargo:rerun-if-env-changed=SPEC42_KPAR_LIBRARY_SOURCE_DIR_{}",
            entry.id.to_ascii_uppercase().replace('-', "_")
        );
        println!(
            "cargo:rerun-if-env-changed=SPEC42_KPAR_LIBRARY_BUNDLE_{}",
            entry.id.to_ascii_uppercase().replace('-', "_")
        );

        pack_or_copy_library(entry, &out_kpar);
        embedded.push(entry.clone());
    }
    write_registry(&registry_path, &embedded, true);
}

#[derive(Clone)]
struct LibraryEntry {
    id: String,
    display_name: String,
    version: String,
    repo: String,
    content_path: String,
    format: String,
    artifact: Option<String>,
    resource: Option<String>,
    pack: PackConfig,
    config_path: PathBuf,
}

fn load_library_configs(libraries_dir: &Path) -> Vec<LibraryEntry> {
    if !libraries_dir.is_dir() {
        eprintln!(
            "workspace build: missing KPAR libraries config directory {}",
            libraries_dir.display()
        );
        process::exit(1);
    }
    let mut entries = Vec::new();
    let mut files: Vec<PathBuf> = fs::read_dir(libraries_dir)
        .unwrap_or_else(|e| {
            eprintln!(
                "workspace build: failed to read {}: {e}",
                libraries_dir.display()
            );
            process::exit(1);
        })
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| {
            path.extension()
                .and_then(|ext| ext.to_str())
                .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
        })
        .collect();
    files.sort();
    for path in files {
        println!("cargo:rerun-if-changed={}", path.display());
        let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to read {}: {e}", path.display());
            process::exit(1);
        });
        let parsed: KparLibraryFile = serde_json::from_str(&raw).unwrap_or_else(|e| {
            eprintln!("workspace build: failed to parse {}: {e}", path.display());
            process::exit(1);
        });
        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("library")
            .to_string();
        let id = parsed.id.unwrap_or(stem);
        if id.is_empty() {
            eprintln!("workspace build: empty library id in {}", path.display());
            process::exit(1);
        }
        entries.push(LibraryEntry {
            id,
            display_name: parsed.display_name,
            version: parsed.version,
            repo: parsed.repo,
            content_path: parsed.content_path,
            format: parsed.format,
            artifact: parsed.artifact,
            resource: parsed.resource,
            pack: parsed.pack,
            config_path: path,
        });
    }
    entries
}

fn pack_or_copy_library(entry: &LibraryEntry, out_kpar: &Path) {
    let env_source = format!(
        "SPEC42_KPAR_LIBRARY_SOURCE_DIR_{}",
        entry.id.to_ascii_uppercase().replace('-', "_")
    );
    if let Some(source_dir) = resolve_source_dir_env(&env_source) {
        pack_library(entry, &source_dir, out_kpar);
        eprintln!(
            "workspace build: packed {} from {}",
            entry.id,
            source_dir.display()
        );
        return;
    }

    let sibling = repo_root().join(&entry.pack.sibling_relative);
    if sibling.is_dir() {
        println!("cargo:rerun-if-changed={}", sibling.display());
        pack_library(entry, &sibling, out_kpar);
        eprintln!(
            "workspace build: packed {} from sibling {}",
            entry.id,
            sibling.display()
        );
        return;
    }

    let cache_name = entry
        .artifact
        .clone()
        .unwrap_or_else(|| format!("elan8-{}-libraries-{}.kpar", entry.id, entry.version));
    let env_bundle = format!(
        "SPEC42_KPAR_LIBRARY_BUNDLE_{}",
        entry.id.to_ascii_uppercase().replace('-', "_")
    );

    if out_kpar.exists() && embedded_kpar_is_usable(out_kpar) {
        eprintln!(
            "workspace build: reusing cached embedded {} KPAR at {}",
            entry.id,
            out_kpar.display()
        );
        return;
    }

    if let Some(bundle) = resolve_bundle(&env_bundle, &cache_name) {
        if bundle.extension().is_none_or(|ext| ext != "kpar") {
            eprintln!(
                "workspace build: expected a .kpar bundle at {}",
                bundle.display()
            );
            process::exit(1);
        }
        copy_file(&bundle, out_kpar, &format!("{} KPAR", entry.id));
        return;
    }

    if let Some(cached) = find_cached_embedded_kpar(out_kpar, out_kpar.file_name().unwrap()) {
        copy_file(
            &cached,
            out_kpar,
            &format!("cached embedded {} KPAR", entry.id),
        );
        eprintln!(
            "workspace build: reused cached embedded {} KPAR from {}",
            entry.id,
            cached.display()
        );
        return;
    }

    eprintln!(
        "workspace build: embedded {} requires a local KPAR for {}.",
        entry.id, entry.version
    );
    eprintln!(
        "workspace build: set {env_source}, place sibling {}, or run scripts/fetch-kpar-libraries-bundle.sh {}",
        entry.pack.sibling_relative, entry.id
    );
    let _ = entry.config_path;
    process::exit(1);
}

fn pack_library(entry: &LibraryEntry, source_dir: &Path, out_kpar: &Path) {
    let project = Project {
        name: format!("elan8-{}-libraries", entry.id),
        version: entry.version.clone(),
        description: Some(entry.display_name.clone()),
        license: Some("MIT".to_string()),
        publisher: Some("elan8".to_string()),
        maintainer: vec![],
        website: None,
        topic: vec![],
        usage: vec![],
    };
    let options = match entry.pack.kind.as_str() {
        "domain-roots" => PackOptions::domain_libraries_defaults(project, source_dir),
        "named-prefix" => {
            let prefix = entry
                .pack
                .archive_prefix
                .clone()
                .unwrap_or_else(|| entry.id.clone());
            PackOptions {
                project,
                source_roots: Vec::new(),
                named_source_roots: vec![(prefix, source_dir.to_path_buf())],
                excludes: kpar::pack::default_domain_excludes(),
                timestamp: kpar::pack::ArchiveTimestamp::default(),
                compression: kpar::pack::ArchiveCompression::default(),
            }
        }
        other => {
            eprintln!(
                "workspace build: unknown pack.kind '{other}' for library {}",
                entry.id
            );
            process::exit(1);
        }
    };
    build_kpar(&options, out_kpar).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to pack {}: {e}", entry.id);
        process::exit(1);
    });
}

fn write_registry(path: &Path, entries: &[LibraryEntry], include_archives: bool) {
    let mut out = String::new();
    out.push_str("#[derive(Debug)]\n");
    out.push_str("pub struct EmbeddedKparLibrary {\n");
    out.push_str("    pub id: &'static str,\n");
    out.push_str("    pub display_name: &'static str,\n");
    out.push_str("    pub version: &'static str,\n");
    out.push_str("    pub repo: &'static str,\n");
    out.push_str("    pub content_path: &'static str,\n");
    out.push_str("    pub format: &'static str,\n");
    out.push_str("    pub artifact: Option<&'static str>,\n");
    out.push_str("    pub resource: Option<&'static str>,\n");
    out.push_str("    pub archive: &'static [u8],\n");
    out.push_str("}\n\n");
    out.push_str("pub static EMBEDDED_KPAR_LIBRARIES: &[EmbeddedKparLibrary] = &[\n");
    if include_archives {
        for entry in entries {
            let artifact = match &entry.artifact {
                Some(value) => format!("Some(\"{}\")", escape_rust_str(value)),
                None => "None".to_string(),
            };
            let resource = match &entry.resource {
                Some(value) => format!("Some(\"{}\")", escape_rust_str(value)),
                None => "None".to_string(),
            };
            out.push_str("    EmbeddedKparLibrary {\n");
            out.push_str(&format!(
                "        id: \"{}\",\n",
                escape_rust_str(&entry.id)
            ));
            out.push_str(&format!(
                "        display_name: \"{}\",\n",
                escape_rust_str(&entry.display_name)
            ));
            out.push_str(&format!(
                "        version: \"{}\",\n",
                escape_rust_str(&entry.version)
            ));
            out.push_str(&format!(
                "        repo: \"{}\",\n",
                escape_rust_str(&entry.repo)
            ));
            out.push_str(&format!(
                "        content_path: \"{}\",\n",
                escape_rust_str(&entry.content_path)
            ));
            out.push_str(&format!(
                "        format: \"{}\",\n",
                escape_rust_str(&entry.format)
            ));
            out.push_str(&format!("        artifact: {artifact},\n"));
            out.push_str(&format!("        resource: {resource},\n"));
            out.push_str(&format!(
                "        archive: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{}.embedded.kpar\")),\n",
                entry.id
            ));
            out.push_str("    },\n");
        }
    }
    out.push_str("];\n");
    fs::write(path, out).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to write {}: {e}", path.display());
        process::exit(1);
    });
}

fn escape_rust_str(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

fn load_stdlib_config() -> StdlibConfig {
    let path = config_dir().join("standard-library.json");
    println!("cargo:rerun-if-changed={}", path.display());
    let raw = fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to read {}: {e}", path.display());
        process::exit(1);
    });
    serde_json::from_str(&raw).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to parse {}: {e}", path.display());
        process::exit(1);
    })
}

fn config_dir() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"))
        .join("../../config")
}

fn config_libraries_dir() -> PathBuf {
    config_dir().join("libraries")
}

fn repo_root() -> PathBuf {
    PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR")).join("../..")
}

fn embed_cache_dir() -> PathBuf {
    repo_root().join(".cache")
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

fn resolve_bundle(env_name: &str, cache_name: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var(env_name) {
        let trimmed = path.trim();
        if !trimmed.is_empty() {
            return Some(PathBuf::from(trimmed));
        }
    }
    let cached = embed_cache_dir().join(cache_name);
    cached.is_file().then_some(cached)
}

fn resolve_source_dir_env(env_name: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var(env_name) {
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

fn find_cached_embedded_kpar(out_kpar: &Path, file_name: &std::ffi::OsStr) -> Option<PathBuf> {
    let build_root = out_kpar.parent()?.parent()?.parent()?;
    for entry in fs::read_dir(build_root).ok()?.flatten() {
        let candidate = entry.path().join("out").join(file_name);
        if candidate != out_kpar && candidate.is_file() && embedded_kpar_is_usable(&candidate) {
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
            continue;
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

fn embedded_kpar_is_usable(path: &Path) -> bool {
    let Ok(bytes) = fs::read(path) else {
        return false;
    };
    !bytes.is_empty() && kpar::is_kpar_archive(&bytes)
}

fn write_empty_stub(path: &Path, label: &str) {
    fs::write(path, []).unwrap_or_else(|e| {
        eprintln!("workspace build: failed to write empty {label}: {e}");
        process::exit(1);
    });
}

fn copy_file(from: &Path, to: &Path, label: &str) {
    fs::copy(from, to).unwrap_or_else(|e| {
        eprintln!(
            "workspace build: failed to copy {label} {}: {e}",
            from.display()
        );
        process::exit(1);
    });
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
