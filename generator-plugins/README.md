# Repository generator plugins

This nested Cargo workspace contains Spec42-owned Rust WebAssembly generators. Plugins consume
only the typed, read-only generator SDK backed by one immutable semantic publication.

- `example` is the minimal SDK example referenced by the generator ABI documentation.

Diagrams are not a plugin: Spec42 builds the diagram product natively (`crates/diagram_product`)
and serves it through `spec42 diagram` and the LSP `spec42/diagram` request.

Build all plugins without adding the WebAssembly-only crates to the root workspace:

```sh
cargo build --manifest-path generator-plugins/Cargo.toml \
  --target wasm32-unknown-unknown --release
```
