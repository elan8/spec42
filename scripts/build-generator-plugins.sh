#!/usr/bin/env bash
# Builds the generator conformance plugin corpus to WebAssembly.
#
# The plugins live in a nested cargo workspace so `cargo clippy --workspace` at the repo root
# does not try to build wasm-only crates for the host target. Release profile matters: the
# debug build of a plugin is ~36x larger and costs ~3x more to compile, and module
# compilation dominates the harness runtime.
#
# Panic locations embed the compile-time absolute source path of every panicking call site
# (`.unwrap()`, `.expect()`, indexing, ...) reachable from a plugin, including in crates outside
# this nested workspace such as `generator_sdk`, and in monomorphized generic std functions
# recompiled fresh for wasm32 (most of the precompiled std sysroot is already remapped to a
# fixed `/rustc/<hash>/...` form by Rust's own release process, but code like this is not). A
# longer or shorter absolute path -- for either this checkout or the toolchain install -- shifts
# the embedded string's length, which shifts the compiled module's exact byte layout and
# therefore its exact metered fuel consumption, even though no plugin logic changed.
# `--remap-path-prefix` rewrites all three -- this checkout, the toolchain, and `$CARGO_HOME`
# (registry sources such as `serde_json` are embedded the same way) -- to fixed placeholders so
# the same source always compiles to the same bytes, regardless of where any of them live on disk.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
sysroot="$(cd "$root/generator-tests/plugins" && rustc --print sysroot)"

if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$'; then
  echo "installing the wasm32-unknown-unknown target"
  rustup target add wasm32-unknown-unknown
fi

# rustc embeds each path exactly as it resolves it at compile time. On Linux/macOS that is the
# same POSIX path `pwd`/`rustc --print sysroot` already give us. On Windows, MSYS/Git Bash's
# `pwd` returns a POSIX-style path (`/c/...`) that never matches rustc's own Windows-native,
# backslash-separated view of the same directory (`C:\...`), so a remap built from it silently
# no-ops unless converted with `pwd -W` first; `rustc --print sysroot` is already native.
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
  --manifest-path "$root/generator-tests/plugins/Cargo.toml" \
  --release \
  --target wasm32-unknown-unknown

echo "plugins built into generator-tests/plugins/target/wasm32-unknown-unknown/release"
