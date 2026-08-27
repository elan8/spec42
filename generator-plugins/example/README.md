# Rust generator example

Build a core module for the authority-free target:

```sh
rustup target add wasm32-unknown-unknown
cargo build --manifest-path generator-plugins/Cargo.toml -p spec42-example-generator --target wasm32-unknown-unknown --release
```

Run it from the repository root:

```sh
cargo run -p server --bin spec42 -- generate \
  generator-plugins/target/wasm32-unknown-unknown/release/spec42_example_generator.wasm \
  path/to/model.sysml \
  --output generated \
  -- target=rust
```

Spec42 loads the module directly with Wasmtime. The guest receives only the `spec42.query` and
`spec42.diagnostic` imports, and returns its generated files from the entrypoint. It does not
receive WASI filesystem, network, environment, clock, random, or subprocess imports.
