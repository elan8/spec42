# Fuzzing

Spec42 keeps small, minimized SysML source seeds in `fuzz/corpus/sysml`.
They are valid UTF-8 because the parser, semantic workspace, and formatter
public APIs accept `&str`; fuzz targets reject arbitrary non-UTF-8 byte inputs
without lossy conversion.

Run the deterministic seed gate first:

```sh
cargo test --manifest-path fuzz/Cargo.toml --test sysml_seed_corpus
```

Install a nightly toolchain and `cargo-fuzz` to explore the language targets:

```sh
cd fuzz
cargo +nightly fuzz run sysml_parser corpus/sysml -- -max_total_time=300
cargo +nightly fuzz run sysml_publication corpus/sysml -- -max_total_time=300
cargo +nightly fuzz run sysml_formatter corpus/sysml -- -max_total_time=300
```

Fuzzing is advisory because its runtime exploration is not reproducible.
When it finds a failure, minimize the input, add a UTF-8 source seed when it
exercises a text API, and keep the deterministic seed test passing. Generated
crash artifacts remain under `fuzz/artifacts/` for inspection.
