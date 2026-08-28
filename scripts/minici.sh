#!/usr/bin/env bash
# Run the high-signal, platform-independent portion of required CI locally.
#
# This script deliberately performs no downloads. Prepare the pinned library bundles with the
# repository fetch scripts before running it. CI-only packaging and cross-platform jobs remain in
# .github/workflows/ci.yml.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

step() {
  echo
  echo "==> $*"
}

require_command() {
  if ! command -v "$1" >/dev/null 2>&1; then
    echo "error: required command '$1' is not installed" >&2
    exit 2
  fi
}

require_path() {
  local path="$1"
  local preparation="$2"
  if [[ ! -e "$path" ]]; then
    echo "error: missing local CI prerequisite: $path" >&2
    echo "prepare it explicitly with: $preparation" >&2
    exit 2
  fi
}

for command in cargo git node python3 rustc rustup; do
  require_command "$command"
done

if ! rustup target list --installed | grep -q '^wasm32-unknown-unknown$'; then
  echo "error: the wasm32-unknown-unknown Rust target is not installed" >&2
  echo "install it explicitly with: rustup target add wasm32-unknown-unknown" >&2
  exit 2
fi

if git submodule status --recursive | grep -q '^-'; then
  echo "error: repository submodules are not initialized" >&2
  echo "initialize them explicitly with: git submodule update --init --recursive" >&2
  exit 2
fi

stdlib_version="$(node -p "require('./config/standard-library.json').version")"
domain_version="$(node -p "require('./config/libraries/domain.json').version")"
method_version="$(node -p "require('./config/libraries/method.json').version")"

export SPEC42_STDLIB_VERSION="$stdlib_version"
export SPEC42_STDLIB_KPAR_DIR="${SPEC42_STDLIB_KPAR_DIR:-$root/.cache/sysml-stdlib-kpar-$stdlib_version}"
export SPEC42_DOMAIN_LIBRARIES_VERSION="$domain_version"
export SPEC42_METHOD_LIBRARIES_VERSION="$method_version"
export SPEC42_KPAR_LIBRARY_BUNDLE_DOMAIN="${SPEC42_KPAR_LIBRARY_BUNDLE_DOMAIN:-$root/.cache/elan8-domain-libraries-$domain_version.kpar}"
export SPEC42_KPAR_LIBRARY_BUNDLE_METHOD="${SPEC42_KPAR_LIBRARY_BUNDLE_METHOD:-$root/.cache/elan8-method-libraries-$method_version.kpar}"

require_path "$SPEC42_STDLIB_KPAR_DIR" "bash scripts/fetch-stdlib-bundle.sh"
require_path "$SPEC42_KPAR_LIBRARY_BUNDLE_DOMAIN" "bash scripts/fetch-kpar-libraries-bundle.sh"
require_path "$SPEC42_KPAR_LIBRARY_BUNDLE_METHOD" "bash scripts/fetch-kpar-libraries-bundle.sh"

step "Formatting"
cargo fmt --all -- --check

step "Workspace Clippy (all targets and features)"
cargo clippy --workspace --all-targets --all-features -- -D warnings

step "Dependency policy"
require_command cargo-deny
cargo deny check bans

step "Generated configuration and contract checks"
node scripts/sync-standard-library-config.mjs --check
node scripts/sync-kpar-libraries-config.mjs --check
node scripts/sync-workspace-version.mjs --check
node scripts/generate-conformance-matrix.mjs --check
node scripts/sync-generator-abi.mjs --check

abi_manifest="$(mktemp "${TMPDIR:-/tmp}/spec42-generator-abi.XXXXXX")"
trap 'rm -f "$abi_manifest"' EXIT
cargo run -p spec42-generator-protocol --example abi-manifest >"$abi_manifest"
if ! cmp -s "$abi_manifest" docs/generation/generator-abi.json; then
  echo "error: docs/generation/generator-abi.json is stale" >&2
  diff -u docs/generation/generator-abi.json "$abi_manifest" || true
  exit 1
fi

step "Reference tooling"
python3 -m unittest tools/sysml_reference/test_tools.py

step "Core workspace tests"
export RUST_MIN_STACK=16777216
cargo test --workspace --exclude server --exclude lsp_server
cargo test -p lsp_server --lib --test debt_guardrails --test parse_validation
cargo test -p lsp_server --test lsp_integration -- --test-threads=1
cargo test -p server --lib
cargo test --manifest-path fuzz/Cargo.toml --test sysml_seed_corpus

step "Generator conformance and smoke tests"
scripts/build-generator-plugins.sh
cargo run -p generator_conformance --bin generator-conformance
if [[ "${CI:-false}" == "true" ]]; then
  git diff --exit-code generator-tests/golden
fi
cargo test -p server --test integration generator_cli

# Build the local guests needed by smoke tests and snapshots. The extension stages its Wasm guest
# during packaging; generated Wasm is deliberately not a source-controlled artifact.
SPEC42_PACKAGE_REPOSITORY_GENERATORS=0 scripts/build-repository-generator-plugins.sh
cargo run -p server --bin spec42 -- --no-stdlib generate \
  generator-plugins/target/wasm32-unknown-unknown/release/spec42_example_generator.wasm \
  vscode/testFixture/workspaces/multi-file/def.sysml --output target/generator-smoke -- target=rust
cargo run -p server --bin spec42 -- --no-stdlib generate \
  generator-plugins/target/wasm32-unknown-unknown/release/spec42_example_generator.wasm \
  vscode/testFixture/workspaces/multi-file/def.sysml --output target/generator-smoke --check -- target=rust
cargo test -p server --test integration diagram_generator_smoke

step "Standard-library publication ratchet"
cargo snapshot check --fixture standard_library_admission.md

step "Semantic snapshot corpus"
cargo snapshot check

step "Workspace documentation"
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --workspace

echo
echo "mini CI passed"
