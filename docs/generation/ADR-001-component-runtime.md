# ADR-001: WebAssembly Component runtime

Status: accepted for generator API 0.1.

## Decision

Accept WebAssembly Components only and embed Wasmtime's Component Model API with generated WIT
bindings. Core modules are not adapted by the CLI. Rust guests build an authority-free
`wasm32-unknown-unknown` core module and explicitly componentize it during their build.

The host compiles and pre-links the component before loading a model. Each invocation gets a fresh
Wasmtime store containing only the model view, artifact collector, diagnostics, resource limiter,
and counters. The engine may later cache compiled components by SHA-256 digest, but no store or
instance state may be cached. Release builds enable Wasmtime's component model, Cranelift, runtime,
and standard-library features; no `wasmtime-wasi` dependency is present.

## Consequences

- WIT versions participate in Component Model linking, so incompatible imports fail before guest
  execution with the required package version in the error.
- There is no manually maintained pointer/linear-memory ABI.
- Guests cannot accidentally gain WASI merely by declaring it; undeclared host imports fail.
- Wasmtime and Cranelift increase binary and compile size. Release-size measurement and compiled
  artifact caching remain benchmark work before 1.0.
- A standalone core `.wasm` is rejected with guidance to produce a component. Automatic adapters
  are deferred because an adapter could add authority and obscure the effective capability set.
