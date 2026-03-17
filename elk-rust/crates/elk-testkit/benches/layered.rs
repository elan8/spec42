use criterion::{Criterion, criterion_group, criterion_main};
use elk_core::{LayoutEngine, LayoutOptions};
use elk_layered::LayeredLayoutEngine;
use elk_testkit::{
    canonical_dag, compound_graph, deep_dag_graph, label_heavy_graph, long_edge_graph,
    parallel_edges_graph,
};

fn bench_graph(c: &mut Criterion, name: &str, build: impl Fn() -> elk_core::Graph) {
    c.bench_function(name, |b| {
        b.iter(|| {
            let mut graph = build();
            LayeredLayoutEngine::new()
                .layout(&mut graph, &LayoutOptions::default())
                .expect("layout should succeed");
        });
    });
}

fn layered_benchmark(c: &mut Criterion) {
    bench_graph(c, "layered_canonical_dag", canonical_dag);
    bench_graph(c, "layered_deep_dag", deep_dag_graph);
    bench_graph(c, "layered_label_heavy", label_heavy_graph);
    bench_graph(c, "layered_parallel_edges", parallel_edges_graph);
    bench_graph(c, "layered_compound", compound_graph);
    bench_graph(c, "layered_long_edge", long_edge_graph);
}

criterion_group!(benches, layered_benchmark);
criterion_main!(benches);
