# elk-rust

`elk-rust` is an idiomatic Rust reimplementation of the Eclipse Layout Kernel (ELK).

The current workspace focuses on:

- a typed graph model and layout API in `elk-core`
- a first layered / Sugiyama-style engine in `elk-layered`
- fixtures, assertions, property tests, and benchmarks in `elk-testkit`

This is an early foundation release. The implementation currently prioritizes:

- deterministic layered layout for directed graphs
- compound nodes at the model level with recursive child layout
- typed options over stringly-typed configuration
- testability, parity scaffolding, and extension points for future engines

## SVG snapshots

You can generate opt-in SVG snapshots for the main test fixtures during `cargo test` by setting `ELK_TESTKIT_WRITE_SVG=1`.

Snapshots are written to `target/elk-testkit-snapshots/` and are intended for manual review of nodes, ports, bends, labels, and layout statistics.
