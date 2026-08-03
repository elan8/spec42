# ADR-001: Direct core WebAssembly generator runtime

Status: accepted for generator ABI 1.

## Decision

Use core WebAssembly modules as generator plugins and execute them directly with Wasmtime's
`Module` API.

The ABI has three imports under module `spec42`: a compact Postcard model-query call, raw artifact
emission, and diagnostics. The guest exports its memory, allocation/deallocation functions, and a
generation entrypoint. The first-party Rust SDK implements this low-level boundary for generators
and language platform adapters.

The host compiles and pre-links the module before loading a model. Each invocation gets a fresh
Wasmtime store containing only the model view, artifact collector, diagnostics, resource limiter,
and counters. The engine may later cache compiled modules by SHA-256 digest, but no store or
instance state may be cached. No WASI implementation is linked.

## Consequences

- A plugin is an ordinary core `.wasm` module with no required custom metadata or post-processing.
- ABI compatibility is checked from ordinary import/export names and function signatures.
- Postcard keeps structured model transfers compact; emitted artifact bytes bypass serialization.
- Guests cannot gain ambient authority because undeclared imports fail linking.
- The binary schema is coupled to an explicit ABI version. Shape changes require a new ABI version.
- The SDK owns pointer and linear-memory details so generator authors and applications do not.
