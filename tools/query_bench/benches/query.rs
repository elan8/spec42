//! The admission benchmark for representation changes.
//!
//! `design.md`: a representation change is admitted with a benchmark showing it neutral-or-better
//! on the bundled standard-library corpus. These are the six cases that number is read from: the
//! cold build, the warm relink one keystroke costs, and the four keystroke-path queries.
//!
//! Run with `cargo bench -p spec42-query-bench`.

use std::hint::black_box;
use std::sync::OnceLock;

use divan::Bencher;
use spec42_query_bench::{
    cold_build, completion_position, navigation_position, warm_relink, Corpus, Fixture,
};
use sysml_query::resolved_slice::{QueryAnswer, SymbolId};

/// Divan's allocation profiler. It reports allocations alongside time for every case here; the
/// per-element normalisation is the `spec42-query-bench-allocations` binary's job.
#[global_allocator]
static ALLOCATOR: divan::AllocProfiler = divan::AllocProfiler::system();

fn main() {
    divan::main();
}

fn corpus() -> &'static Corpus {
    static CORPUS: OnceLock<Corpus> = OnceLock::new();
    CORPUS.get_or_init(|| Corpus::load().expect("load the standard-library corpus"))
}

fn fixture() -> &'static Fixture {
    static FIXTURE: OnceLock<Fixture> = OnceLock::new();
    FIXTURE.get_or_init(|| Fixture::build(corpus()).expect("build the bench fixture"))
}

/// Case 1: a cold host opens a workspace against the bundled standard library.
#[divan::bench(sample_count = 5, sample_size = 1)]
fn cold_build_stdlib(bencher: Bencher) {
    let corpus = corpus();
    bencher.bench(|| {
        let model = cold_build(corpus).expect("cold build");
        black_box(model.publication().completeness());
    });
}

/// Case 2: one keystroke in one user document, against a settled library stratum.
#[divan::bench(sample_count = 20, sample_size = 1)]
fn warm_relink_one_document(bencher: Bencher) {
    let fixture = fixture();
    let mut revision = 0usize;
    bencher.bench_local(|| {
        revision += 1;
        let model = warm_relink(fixture, revision).expect("warm relink");
        black_box(model.publication().completeness());
    });
}

/// Case 3: completion -- the visible members at a point inside a library-typed body.
#[divan::bench]
fn q_visible_members(bencher: Bencher) {
    let fixture = fixture();
    let position = completion_position();
    bencher.bench(|| {
        black_box(fixture.model.completion().visible_members(
            &fixture.workspace_document,
            position,
            None,
        ))
    });
}

/// Case 4a: go-to-definition on a type reference.
#[divan::bench]
fn q_target_at(bencher: Bencher) {
    let fixture = fixture();
    let position = navigation_position();
    bencher.bench(|| {
        black_box(
            fixture
                .model
                .navigation()
                .target_at(&fixture.workspace_document, position),
        )
    });
}

/// Case 4b: find-all-references on the symbol that reference resolves to.
#[divan::bench]
fn q_references(bencher: Bencher) {
    let fixture = fixture();
    let symbol = navigation_symbol(fixture);
    bencher.bench(|| black_box(fixture.model.navigation().references(symbol, true)));
}

/// Case 5: the outline of the largest library document.
#[divan::bench]
fn q_document_symbols(bencher: Bencher) {
    let fixture = fixture();
    bencher.bench(|| {
        black_box(
            fixture
                .model
                .inspection()
                .document_symbols(&fixture.outline_document),
        )
    });
}

/// Case 6: one document's diagnostics. This must stay proportional to what it returns.
#[divan::bench]
fn q_diagnostics_for_document(bencher: Bencher) {
    let fixture = fixture();
    bencher.bench(|| {
        black_box(
            fixture
                .model
                .diagnostics()
                .for_document(&fixture.workspace_document),
        )
    });
}

fn navigation_symbol(fixture: &Fixture) -> SymbolId {
    let outcome = fixture
        .model
        .navigation()
        .target_at(&fixture.workspace_document, navigation_position());
    black_box(outcome.completeness);
    match outcome.answer {
        QueryAnswer::Resolved(target) => target.symbol,
        answer => panic!("the navigation fixture must resolve; got {answer:?}"),
    }
}
