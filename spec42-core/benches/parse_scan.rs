use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use tower_lsp::lsp_types::Url;

use spec42_core::bench::{
    build_document_graphs, extract_symbols_from_workspace, link_cross_document_relationships,
    merge_document_graphs, parse_scanned_entries, scan_sysml_files, startup_index_scanned_entries,
};
use spec42_core::common::util as spec42_util;

fn env_path(name: &str) -> Option<PathBuf> {
    std::env::var_os(name)
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
}

fn url_from_path(path: &PathBuf) -> Option<Url> {
    Url::from_file_path(path).ok()
}

fn existing_roots(paths: &[PathBuf]) -> Vec<PathBuf> {
    paths.iter().filter(|path| path.exists()).cloned().collect()
}

fn roots_to_urls(roots: &[PathBuf]) -> Option<Vec<Url>> {
    roots.iter().map(url_from_path).collect()
}

fn parallel_parse_enabled(entry_count: usize) -> bool {
    let enabled = spec42_util::env_flag_enabled("SPEC42_PARALLEL_STARTUP_PARSE", true);
    let min_files = spec42_util::env_usize("SPEC42_PARALLEL_STARTUP_PARSE_MIN_FILES", 10);
    enabled && entry_count >= min_files
}

fn bench_root_set(c: &mut Criterion, group_name: &str, roots: Vec<PathBuf>) {
    let mut group = c.benchmark_group(group_name);
    let roots = existing_roots(&roots);
    if roots.is_empty() {
        eprintln!("[bench] skipped {}: no existing roots", group_name);
        group.finish();
        return;
    }

    let Some(root_urls) = roots_to_urls(&roots) else {
        eprintln!(
            "[bench] skipped {}: one or more roots are not valid file URLs: {:?}",
            group_name, roots
        );
        group.finish();
        return;
    };

    let entries = scan_sysml_files(root_urls.clone());
    if entries.is_empty() {
        eprintln!(
            "[bench] skipped {}: no .sysml/.kerml files under {:?}",
            group_name, roots
        );
        group.finish();
        return;
    }

    let should_parallel_parse = parallel_parse_enabled(entries.len());
    let bytes_total: u64 = entries
        .iter()
        .map(|(_, content)| content.len() as u64)
        .sum();
    group.throughput(Throughput::Bytes(bytes_total));

    let label = roots
        .iter()
        .map(|root| root.display().to_string())
        .collect::<Vec<_>>()
        .join(" + ");

    group.bench_function(BenchmarkId::new("scan_only", &label), |b| {
        b.iter(|| {
            let scanned = scan_sysml_files(root_urls.clone());
            criterion::black_box(scanned.len());
        })
    });

    group.bench_function(BenchmarkId::new("parse_only", entries.len()), |b| {
        b.iter(|| {
            let parsed = parse_scanned_entries(entries.clone(), should_parallel_parse);
            criterion::black_box(parsed);
        })
    });

    group.bench_function(
        BenchmarkId::new("startup_index_total", entries.len()),
        |b| {
            b.iter(|| {
                let indexed = startup_index_scanned_entries(entries.clone(), should_parallel_parse);
                criterion::black_box(indexed);
            })
        },
    );

    group.bench_function(BenchmarkId::new("graph_build_total", entries.len()), |b| {
        b.iter(|| {
            let nodes = build_document_graphs(entries.clone(), should_parallel_parse);
            criterion::black_box(nodes);
        })
    });

    group.bench_function(BenchmarkId::new("graph_merge_total", entries.len()), |b| {
        b.iter(|| {
            let merged = merge_document_graphs(entries.clone(), should_parallel_parse);
            criterion::black_box(merged);
        })
    });

    group.bench_function(
        BenchmarkId::new("cross_document_link_total", entries.len()),
        |b| {
            b.iter(|| {
                let linked =
                    link_cross_document_relationships(entries.clone(), should_parallel_parse);
                criterion::black_box(linked);
            })
        },
    );

    group.bench_function(
        BenchmarkId::new("symbol_extract_total", entries.len()),
        |b| {
            b.iter(|| {
                let symbols =
                    extract_symbols_from_workspace(entries.clone(), should_parallel_parse);
                criterion::black_box(symbols);
            })
        },
    );

    group.bench_function(BenchmarkId::new("scan_parse_total", &label), |b| {
        b.iter(|| {
            let scanned = scan_sysml_files(root_urls.clone());
            let parsed = parse_scanned_entries(scanned, should_parallel_parse);
            criterion::black_box(parsed);
        })
    });

    group.finish();
}

fn stdlib_root_from_release_root(release_root: PathBuf) -> PathBuf {
    let sysml_root = release_root.join("sysml");
    let preferred = sysml_root.join("src");
    if preferred.exists() {
        preferred
    } else {
        sysml_root
    }
}

fn parse_scan_benches(c: &mut Criterion) {
    let workspace_root = env_path("SPEC42_BENCH_DRONE_ROOT")
        .unwrap_or_else(|| PathBuf::from(r"C:\Git\sysml-examples\drone\sysml"));
    bench_root_set(c, "workspace_scan", vec![workspace_root.clone()]);

    let stdlib_release_root = env_path("SYSML_V2_RELEASE_DIR")
        .unwrap_or_else(|| PathBuf::from(r"C:\Git\SysML-v2-Release-2026-01"));
    let stdlib_root = stdlib_root_from_release_root(stdlib_release_root);
    bench_root_set(c, "sysml_stdlib_scan", vec![stdlib_root.clone()]);

    bench_root_set(
        c,
        "workspace_plus_stdlib_scan",
        vec![workspace_root, stdlib_root],
    );
}

criterion_group!(benches, parse_scan_benches);
criterion_main!(benches);
