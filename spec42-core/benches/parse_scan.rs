use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tower_lsp::lsp_types::Url;

use spec42_core::bench::{parse_scanned_entries, scan_sysml_files};
use spec42_core::common::util as spec42_util;

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}

fn url_from_path(p: &PathBuf) -> Option<Url> {
    Url::from_file_path(p).ok()
}

fn bench_workspace_scan(
    c: &mut Criterion,
    group_name: &str,
    root: PathBuf,
    parse_only: bool,
) {
    let Some(root_url) = url_from_path(&root) else {
        eprintln!(
            "[bench] skipped {}: root is not a valid file path: {}",
            group_name,
            root.display()
        );
        return;
    };

    let mut group = c.benchmark_group(group_name);

    if !root.exists() {
        eprintln!(
            "[bench] skipped {}: path does not exist: {}",
            group_name,
            root.display()
        );
        group.finish();
        return;
    }

    let parallel_parse_enabled = spec42_util::env_flag_enabled("SPEC42_PARALLEL_STARTUP_PARSE", true);
    let parallel_parse_min_files =
        spec42_util::env_usize("SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES", 10);

    if parse_only {
        let entries = scan_sysml_files(vec![root_url]);
        if entries.is_empty() {
            eprintln!(
                "[bench] skipped {}: no .sysml/.kerml files under {}",
                group_name,
                root.display()
            );
            group.finish();
            return;
        }

        let should_parallel_parse =
            parallel_parse_enabled && entries.len() >= parallel_parse_min_files;
        let bytes_total: u64 = entries.iter().map(|(_, s)| s.len() as u64).sum();
        group.throughput(Throughput::Bytes(bytes_total));
        group.bench_function(BenchmarkId::new("parse_only", entries.len()), |b| {
            b.iter(|| {
                let parsed = parse_scanned_entries(entries.clone(), should_parallel_parse);
                criterion::black_box(parsed);
            })
        });
    } else {
        group.bench_function(BenchmarkId::new("scan_total", root.display().to_string()), |b| {
            b.iter(|| {
                let entries = scan_sysml_files(vec![root_url.clone()]);
                let should_parallel_parse =
                    parallel_parse_enabled && entries.len() >= parallel_parse_min_files;
                let parsed = parse_scanned_entries(entries, should_parallel_parse);
                criterion::black_box(parsed);
            })
        });
    }

    group.finish();
}

fn parse_scan_benches(c: &mut Criterion) {
    // Drone example workspace root (default matches user request; override via env).
    let drone_root = env_path("SPEC42_BENCH_DRONE_ROOT")
        .unwrap_or_else(|| PathBuf::from(r"C:\Git\sysml-examples\drone\sysml"));
    bench_workspace_scan(c, "drone_workspace_scan", drone_root, false);
    let drone_root_parse_only = env_path("SPEC42_BENCH_DRONE_ROOT")
        .unwrap_or_else(|| PathBuf::from(r"C:\Git\sysml-examples\drone\sysml"));
    bench_workspace_scan(c, "drone_workspace_parse_only", drone_root_parse_only, true);

    // SysML v2 release library root (only runs when SYSML_V2_RELEASE_DIR is provided and exists).
    if let Some(release_root) = env_path("SYSML_V2_RELEASE_DIR") {
        // Mirror `spec42-core/src/language/mod.rs` test convention:
        // release_root/<sysml>/<src>/<examples|libraries...>
        // For library indexing, we benchmark the whole `sysml/src` tree when present,
        // otherwise fall back to `sysml` (still valid for user-provided layouts).
        let sysml_root = release_root.join("sysml");
        let preferred = sysml_root.join("src");
        let stdlib_root = if preferred.exists() { preferred } else { sysml_root };
        bench_workspace_scan(c, "sysml_stdlib_scan", stdlib_root.clone(), false);
        bench_workspace_scan(c, "sysml_stdlib_parse_only", stdlib_root, true);
    } else {
        eprintln!("[bench] skipped sysml_stdlib_*: SYSML_V2_RELEASE_DIR is not set");
    }
}

criterion_group!(benches, parse_scan_benches);
criterion_main!(benches);

