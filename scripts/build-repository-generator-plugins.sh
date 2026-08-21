#!/usr/bin/env bash
# Builds repository-owned WebAssembly plugins reproducibly and refreshes packaged artifacts.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
sysroot="$(cd "$root/generator-plugins" && rustc --print sysroot)"

if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$'; then
  echo "installing the wasm32-unknown-unknown target"
  rustup target add wasm32-unknown-unknown
fi

to_native_path() {
  local native
  if native="$(cd "$1" && pwd -W 2>/dev/null)"; then
    echo "${native//\//\\}"
  else
    echo "$1"
  fi
}
root_native="$(to_native_path "$root")"

RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=${root_native}=/spec42 --remap-path-prefix=${sysroot}=/rustc-toolchain" \
  cargo build \
  --manifest-path "$root/generator-plugins/Cargo.toml" \
  --release \
  --target wasm32-unknown-unknown

cp \
  "$root/generator-plugins/target/wasm32-unknown-unknown/release/spec42_diagram_generator.wasm" \
  "$root/vscode/generators/diagram.wasm"

echo "repository plugins built; refreshed vscode/generators/diagram.wasm"
