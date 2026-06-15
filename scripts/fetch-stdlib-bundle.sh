#!/usr/bin/env bash
# Fetch the pinned OMG sysml.library.kpar archives for embedded stdlib builds.
# Uses a sparse git checkout of only sysml.library.kpar/ at the pinned release tag.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
config_path="${repo_root}/config/standard-library.json"

if [[ ! -f "${config_path}" ]]; then
  echo "Missing ${config_path}" >&2
  exit 1
fi

cd "${repo_root}"
version="$(node -p "require('./config/standard-library.json').version")"
repo="$(node -p "require('./config/standard-library.json').repo")"
out="${SPEC42_STDLIB_KPAR_DIR:-${repo_root}/.cache/sysml-stdlib-kpar-${version}}"

kpar_cache_is_valid() {
  local count=0
  shopt -s nullglob
  local files=("${out}"/*.kpar)
  shopt -u nullglob
  if [[ ${#files[@]} -eq 0 ]]; then
    return 1
  fi
  for file in "${files[@]}"; do
    unzip -tq "${file}" >/dev/null 2>&1 || return 1
    count=$((count + 1))
  done
  [[ "${count}" -gt 0 ]]
}

if kpar_cache_is_valid; then
  echo "Using existing stdlib KPAR cache at ${out}"
  exit 0
fi

fetch_via_sparse_git() {
  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' RETURN

  echo "Fetching sysml.library.kpar from ${repo}@${version} via sparse git checkout"
  git clone --depth 1 --filter=blob:none --sparse \
    --branch "${version}" "https://github.com/${repo}.git" "${tmp}/checkout"
  git -C "${tmp}/checkout" sparse-checkout set sysml.library.kpar
  test -d "${tmp}/checkout/sysml.library.kpar"

  rm -rf "${out}"
  mkdir -p "${out}"
  cp "${tmp}/checkout/sysml.library.kpar/"*.kpar "${out}/"
}

if fetch_via_sparse_git; then
  kpar_cache_is_valid
  echo "Fetched stdlib KPAR archives via sparse git checkout"
else
  echo "Failed to fetch stdlib KPAR archives for ${repo}@${version}" >&2
  exit 1
fi

echo "Stdlib KPAR cache ready at ${out}"
