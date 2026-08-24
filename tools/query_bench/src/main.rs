//! Allocations per element for each benchmark case.
//!
//! Wall time is divan's job (`cargo bench -p spec42-query-bench`). This binary answers the other
//! half of the representation question: how many heap allocations one case performs, and how many
//! that is per element it produced. A counting global allocator is the only way to state that as a
//! measurement rather than an estimate, and it cannot share a binary with divan's own profiler --
//! hence a separate target over the same fixture.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicU64, Ordering};

use spec42_query_bench::{
    cold_build, completion_position, navigation_position, outcome_len, view_outcome_len,
    warm_relink, Corpus, Fixture,
};
use sysml_query::resolved_slice::QueryOutcome;

/// A pass-through allocator that counts allocations and bytes.
///
/// It never changes what is allocated; it only records it, so the numbers describe the same run
/// the timing benchmark measures.
struct CountingAllocator;

static ALLOCATIONS: AtomicU64 = AtomicU64::new(0);
static ALLOCATED_BYTES: AtomicU64 = AtomicU64::new(0);

unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(layout.size() as u64, Ordering::Relaxed);
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        System.dealloc(pointer, layout)
    }

    unsafe fn realloc(&self, pointer: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        ALLOCATED_BYTES.fetch_add(
            new_size.saturating_sub(layout.size()) as u64,
            Ordering::Relaxed,
        );
        System.realloc(pointer, layout, new_size)
    }
}

#[global_allocator]
static ALLOCATOR: CountingAllocator = CountingAllocator;

/// One measured case: what it allocated and how many elements it produced.
struct Measurement {
    case: &'static str,
    allocations: u64,
    bytes: u64,
    elements: usize,
}

impl Measurement {
    fn per_element(&self) -> f64 {
        if self.elements == 0 {
            return 0.0;
        }
        self.allocations as f64 / self.elements as f64
    }
}

/// Runs `case` once and reports what it allocated.
///
/// The counters are read before and after on this thread's clock; the build cases use rayon
/// internally, and `Relaxed` ordering on a global counter still totals every thread's allocations
/// because the measurement is taken after the build has joined.
fn measure(case: &'static str, run: impl FnOnce() -> usize) -> Measurement {
    let before_allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let before_bytes = ALLOCATED_BYTES.load(Ordering::Relaxed);
    let elements = run();
    Measurement {
        case,
        allocations: ALLOCATIONS.load(Ordering::Relaxed) - before_allocations,
        bytes: ALLOCATED_BYTES.load(Ordering::Relaxed) - before_bytes,
        elements,
    }
}

fn main() -> Result<(), String> {
    let corpus = Corpus::load()?;
    let fixture = Fixture::build(&corpus)?;
    let elements = fixture.element_count;

    let mut measurements = Vec::new();

    measurements.push(measure("cold_build_stdlib", || {
        let model = cold_build(&corpus).expect("cold build");
        black_box(model.publication().completeness());
        elements
    }));

    measurements.push(measure("warm_relink_one_document", || {
        let model = warm_relink(&fixture, 1).expect("warm relink");
        black_box(model.publication().completeness());
        elements
    }));

    measurements.push(measure("q_visible_members", || {
        view_outcome_len(fixture.model.completion().visible_members(
            &fixture.workspace_document,
            completion_position(),
            None,
        ))
    }));

    measurements.push(measure("q_target_at", || {
        match fixture
            .model
            .navigation()
            .target_at(&fixture.workspace_document, navigation_position())
        {
            QueryOutcome::Resolved(target) | QueryOutcome::Recovered(target) => {
                black_box(&target);
                1
            }
            _ => 0,
        }
    }));

    let reference_symbol = match fixture
        .model
        .navigation()
        .target_at(&fixture.workspace_document, navigation_position())
    {
        QueryOutcome::Resolved(target) | QueryOutcome::Recovered(target) => Some(target.symbol),
        _ => None,
    };
    if let Some(symbol) = reference_symbol {
        measurements.push(measure("q_references", || {
            outcome_len(fixture.model.navigation().references(symbol, true))
        }));
    }

    measurements.push(measure("q_document_symbols", || {
        outcome_len(
            fixture
                .model
                .inspection()
                .document_symbols(&fixture.outline_document),
        )
    }));

    measurements.push(measure("q_diagnostics_for_document", || {
        fixture
            .model
            .diagnostics()
            .for_document(&fixture.workspace_document)
            .len()
    }));

    println!(
        "corpus: {} library documents, {} bytes, {elements} published elements",
        corpus.library.len(),
        corpus.library_bytes
    );
    println!(
        "{:<28} {:>12} {:>14} {:>10} {:>14}",
        "case", "allocations", "bytes", "elements", "allocs/element"
    );
    for measurement in &measurements {
        println!(
            "{:<28} {:>12} {:>14} {:>10} {:>14.3}",
            measurement.case,
            measurement.allocations,
            measurement.bytes,
            measurement.elements,
            measurement.per_element()
        );
    }

    let cold = measurements
        .iter()
        .find(|measurement| measurement.case == "cold_build_stdlib")
        .expect("the cold build case is always measured");
    if cold.elements == 0 {
        return Err("the cold build published no elements; the corpus or fixture is wrong".into());
    }
    Ok(())
}
