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
# Registry sources (`$CARGO_HOME/registry/src/...`) are embedded by panic locations and debug
# info too; without this remap a macOS-built guest and a Linux-built guest differ in bytes.
cargo_home_native="$(to_native_path "${CARGO_HOME:-$HOME/.cargo}")"

RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=${root_native}=/spec42 --remap-path-prefix=${sysroot}=/rustc-toolchain --remap-path-prefix=${cargo_home_native}=/cargo-home" \
  cargo build \
  --manifest-path "$root/generator-plugins/Cargo.toml" \
  --release \
  --target wasm32-unknown-unknown

if [[ "${SPEC42_PACKAGE_REPOSITORY_GENERATORS:-1}" == "1" ]]; then
  cp \
    "$root/generator-plugins/target/wasm32-unknown-unknown/release/spec42_diagram_generator.wasm" \
    "$root/vscode/generators/diagram.wasm"

  echo "repository plugins built; refreshed vscode/generators/diagram.wasm"
  echo "note: the committed diagram.wasm must be the bytes CI's ubuntu job builds (a guest links the"
  echo "      host toolchain's precompiled wasm32 std, so hosts differ by a few bytes); to refresh the"
  echo "      committed file, download the repository-generator-plugins artifact from a CI run."
else
  echo "repository plugins built without refreshing packaged artifacts"
fi
