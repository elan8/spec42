#!/usr/bin/env bash
# Builds the generator conformance plugin corpus to WebAssembly.
#
# The plugins live in a nested cargo workspace so `cargo clippy --workspace` at the repo root
# does not try to build wasm-only crates for the host target. Release profile matters: the
# debug build of a plugin is ~36x larger and costs ~3x more to compile, and module
# compilation dominates the harness runtime.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$'; then
  echo "installing the wasm32-unknown-unknown target"
  rustup target add wasm32-unknown-unknown
fi

cargo build \
  --manifest-path "$root/generator-tests/plugins/Cargo.toml" \
  --release \
  --target wasm32-unknown-unknown

echo "plugins built into generator-tests/plugins/target/wasm32-unknown-unknown/release"
