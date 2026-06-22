#!/usr/bin/env bash
# Fetch the pinned sysml-robot-vacuum-cleaner showcase model into third_party/.
# Used by local integration tests (cargo test -- --ignored).

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
config_path="${repo_root}/config/robot-vacuum-cleaner.json"

if [[ ! -f "${config_path}" ]]; then
  echo "Missing ${config_path}" >&2
  exit 1
fi

cd "${repo_root}"
version="$(node -p "require('./config/robot-vacuum-cleaner.json').version")"
repo="$(node -p "require('./config/robot-vacuum-cleaner.json').repo")"
checkout_path="$(node -p "require('./config/robot-vacuum-cleaner.json').checkoutPath")"
sparse_paths="$(node -p "require('./config/robot-vacuum-cleaner.json').sparsePaths.join(' ')")"
out="${SPEC42_ROBOT_VACUUM_DIR:-${repo_root}/${checkout_path}}"

robot_vacuum_fixture_is_valid() {
  [[ -d "${out}/model" ]] || return 1
  local count=0
  shopt -s nullglob
  local files=("${out}"/model/*.sysml)
  shopt -u nullglob
  [[ "${#files[@]}" -gt 0 ]]
}

if robot_vacuum_fixture_is_valid; then
  echo "Using existing robot vacuum fixture at ${out}"
  exit 0
fi

fetch_via_sparse_git() {
  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' RETURN

  echo "Fetching ${repo}@${version} into ${out} via sparse git checkout"
  git clone --depth 1 --filter=blob:none --sparse \
    --branch "${version}" "https://github.com/${repo}.git" "${tmp}/checkout"

  git -C "${tmp}/checkout" sparse-checkout set ${sparse_paths}
  for path in ${sparse_paths}; do
    test -d "${tmp}/checkout/${path}"
  done

  rm -rf "${out}"
  mkdir -p "$(dirname "${out}")"
  cp -a "${tmp}/checkout/." "${out}/"
}

if fetch_via_sparse_git && robot_vacuum_fixture_is_valid; then
  echo "Robot vacuum fixture ready at ${out}"
else
  echo "Failed to fetch robot vacuum fixture for ${repo}@${version}" >&2
  exit 1
fi

echo "Run ignored showcase tests with:"
echo "  cargo test -p spec42_host --test robot_vacuum_snapshot -- --ignored --nocapture"
echo "  cargo test -p kernel --test lsp_integration robot_vacuum -- --ignored --nocapture"
