use std::path::{Path, PathBuf};

use generator_host::{GeneratorRuntime, RuntimeOptions};
use spec42_generator_protocol::COMPATIBILITY_TOKEN;

fn generator() -> Vec<u8> {
    wat::parse_str(format!(
        r#"(module
          (import "spec42" "query" (func $query (param i32 i32 i32 i32 i32) (result i64)))
          (import "spec42" "diagnostic" (func $diagnostic (param i32 i32 i32 i32 i32)))
          (memory (export "memory") 1)
          (data (i32.const 1024) "\00\00")
          (func (export "spec42_abi_version") (result i64) (i64.const {COMPATIBILITY_TOKEN}))
          (func (export "spec42_alloc") (param i32) (result i32) (i32.const 2048))
          (func (export "spec42_generate") (param i32 i32) (result i64)
            (i64.const 8589935616)))"#
    ))
    .expect("valid guest")
}

fn cache_artifacts(root: &Path) -> Vec<PathBuf> {
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        for entry in std::fs::read_dir(directory).expect("cache directory") {
            let path = entry.expect("cache entry").path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().and_then(|value| value.to_str()) != Some("stats")
                && !path.to_string_lossy().contains(".wip-")
            {
                files.push(path);
            }
        }
    }
    files
}

fn runtime(cache: &Path) -> GeneratorRuntime {
    GeneratorRuntime::with_options_and_cache_directory(
        RuntimeOptions {
            fuel_metering: false,
            compilation_cache: true,
        },
        cache,
    )
    .expect("runtime")
}

#[test]
fn compiled_module_cache_has_cold_warm_parity_and_rejects_corruption() {
    let cache = tempfile::tempdir().expect("cache");
    let module = generator();

    let cold = runtime(cache.path());
    cold.prepare(&module).expect("cold preparation");
    assert_eq!(cold.compilation_cache_hits(), 0);
    assert_eq!(cold.compilation_cache_misses(), 1);
    drop(cold);

    let warm = runtime(cache.path());
    warm.prepare(&module).expect("warm preparation");
    assert_eq!(warm.compilation_cache_hits(), 1);
    assert_eq!(warm.compilation_cache_misses(), 0);

    let artifacts = cache_artifacts(cache.path());
    assert_eq!(artifacts.len(), 1, "one compiled module cache artifact");
    std::fs::write(&artifacts[0], b"corrupt compiled module").expect("corrupt cache fixture");

    let recovered = runtime(cache.path());
    recovered
        .prepare(&module)
        .expect("corruption falls back to canonical compilation");
    assert_eq!(recovered.compilation_cache_hits(), 0);
    assert_eq!(recovered.compilation_cache_misses(), 1);
}

#[test]
fn unavailable_cache_is_explicit_and_falls_back_to_compilation() {
    let runtime = GeneratorRuntime::with_options_and_cache_directory(
        RuntimeOptions {
            fuel_metering: false,
            compilation_cache: true,
        },
        Path::new("relative-cache-path-is-invalid"),
    )
    .expect("cache failure must not disable generation");
    assert!(!runtime.compilation_cache_enabled());
    assert!(runtime.compilation_cache_error().is_some());
    runtime
        .prepare(&generator())
        .expect("canonical uncached preparation");
}
