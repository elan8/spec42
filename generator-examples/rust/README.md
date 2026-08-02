# Rust generator example

Build a core module for the authority-free target, then wrap its WIT metadata as a component:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-tools
cargo build --manifest-path generator-examples/rust/Cargo.toml --target wasm32-unknown-unknown --release
wasm-tools component new \
  generator-examples/rust/target/wasm32-unknown-unknown/release/spec42_example_generator.wasm \
  -o generator-examples/rust/target/spec42-example-generator.component.wasm
```

Run it from the repository root:

```sh
cargo run -p server --bin spec42 -- generate \
  generator-examples/rust/target/spec42-example-generator.component.wasm \
  path/to/model.sysml \
  --output generated \
  -- target=rust
```

The guest receives only the model, artifact, and diagnostic imports in the versioned WIT world.
It does not receive WASI filesystem, network, environment, clock, random, or subprocess imports.
