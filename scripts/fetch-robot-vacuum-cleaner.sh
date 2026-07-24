#!/usr/bin/env bash
# Fetch the pinned sysml-robot-vacuum-cleaner showcase model into third_party/.
# Pin may be a tag, branch, or commit SHA (see config/robot-vacuum-cleaner.json).

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
pin_file="${out}/.spec42-robot-vacuum-pin"

robot_vacuum_fixture_is_valid() {
  [[ -d "${out}/model" ]] || return 1
  shopt -s nullglob globstar
  local files=("${out}"/model/*.sysml "${out}"/model/**/*.sysml)
  shopt -u nullglob globstar
  [[ "${#files[@]}" -gt 0 ]]
}

pin_matches() {
  [[ -f "${pin_file}" ]] && [[ "$(cat "${pin_file}")" == "${version}" ]]
}

if [[ "${FORCE_ROBOT_VACUUM_FETCH:-}" != "1" ]] \
  && pin_matches \
  && robot_vacuum_fixture_is_valid; then
  echo "Using existing robot vacuum fixture at ${out} (pin ${version})"
  exit 0
fi

fetch_via_sparse_git() {
  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' RETURN

  echo "Fetching ${repo}@${version} into ${out} via sparse git checkout"
  git clone --filter=blob:none --sparse \
    "https://github.com/${repo}.git" "${tmp}/checkout"

  git -C "${tmp}/checkout" sparse-checkout set ${sparse_paths}
  git -C "${tmp}/checkout" fetch --depth 1 origin "${version}"
  git -C "${tmp}/checkout" checkout --detach FETCH_HEAD

  for path in ${sparse_paths}; do
    test -d "${tmp}/checkout/${path}"
  done

  rm -rf "${out}"
  mkdir -p "$(dirname "${out}")"
  cp -a "${tmp}/checkout/." "${out}/"
  printf '%s\n' "${version}" > "${pin_file}"
}

if fetch_via_sparse_git && robot_vacuum_fixture_is_valid; then
  echo "Robot vacuum fixture ready at ${out} (pin ${version})"
else
  echo "Failed to fetch robot vacuum fixture for ${repo}@${version}" >&2
  exit 1
fi

echo "Run the zero-warning gate with:"
echo "  cargo test -p server --test robot_vacuum_check -- --ignored --nocapture"
echo "  cargo test -p workspace --test robot_vacuum_snapshot -- --ignored --nocapture"
