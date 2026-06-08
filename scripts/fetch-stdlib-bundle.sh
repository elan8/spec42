#!/usr/bin/env bash
# Fetch the pinned SysML v2 Release archive for embedded stdlib builds.
# Tries GitHub's zip archive first; falls back to a shallow git clone when archive
# downloads fail (GitHub archive URLs often return 504 for large tags).

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
out="${SPEC42_STDLIB_BUNDLE_ZIP:-${repo_root}/.cache/sysml-v2-release-${version}.zip}"
archive_url="https://github.com/${repo}/archive/refs/tags/${version}.zip"
root_dir="SysML-v2-Release-${version}"

mkdir -p "$(dirname "${out}")"

if [[ -f "${out}" ]] && unzip -tq "${out}" >/dev/null 2>&1; then
  echo "Using existing stdlib bundle at ${out}"
  exit 0
fi

fetch_via_archive() {
  echo "Fetching stdlib bundle from ${archive_url}"
  curl --fail --location \
    --retry 5 --retry-delay 5 --retry-all-errors \
    --connect-timeout 30 --max-time 600 \
    --output "${out}" "${archive_url}"
  unzip -tq "${out}" >/dev/null
}

fetch_via_git() {
  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' RETURN

  echo "Archive download failed; falling back to shallow git clone of ${repo}@${version}"
  git clone --depth 1 --branch "${version}" "https://github.com/${repo}.git" "${tmp}/${root_dir}"
  test -d "${tmp}/${root_dir}/sysml.library"

  rm -f "${out}"
  (cd "${tmp}" && zip -qr "${out}" "${root_dir}")
  unzip -tq "${out}" >/dev/null
}

if fetch_via_archive; then
  echo "Fetched stdlib bundle via GitHub archive"
elif fetch_via_git; then
  echo "Fetched stdlib bundle via git clone fallback"
else
  echo "Failed to fetch stdlib bundle for ${repo}@${version}" >&2
  exit 1
fi

echo "Stdlib bundle ready at ${out}"
