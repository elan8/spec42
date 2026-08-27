# Repository generator plugins

This nested Cargo workspace contains Spec42-owned Rust WebAssembly generators. Plugins consume
only the typed, read-only generator SDK backed by one immutable semantic publication.

- `diagram` emits the versioned JSON render product consumed by the VS Code diagram renderer.
- `example` is the minimal SDK example referenced by the generator ABI documentation.

Build all plugins without adding the WebAssembly-only crates to the root workspace:

```sh
cargo build --manifest-path generator-plugins/Cargo.toml \
  --target wasm32-unknown-unknown --release
```

The diagram plugin accepts a view id as its first argument and, for state-transition views, the
typed catalog handle as its second argument. Unsupported projections remain explicit incomplete
products; plugins must not reconstruct missing semantic facts from display strings.
