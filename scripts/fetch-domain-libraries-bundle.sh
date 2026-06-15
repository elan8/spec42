#!/usr/bin/env bash
# Fetch or build the pinned domain-libraries KPAR for embedded builds.

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
config_path="${repo_root}/config/domain-libraries.json"

if [[ ! -f "${config_path}" ]]; then
  echo "Missing ${config_path}" >&2
  exit 1
fi

cd "${repo_root}"
version="$(node -p "require('./config/domain-libraries.json').version")"
repo="$(node -p "require('./config/domain-libraries.json').repo")"
artifact="$(node -p "require('./config/domain-libraries.json').artifact || ''")"

if [[ -z "${artifact}" ]]; then
  artifact="elan8-domain-libraries-${version}.kpar"
fi
out="${SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP:-${repo_root}/.cache/${artifact}}"

mkdir -p "$(dirname "${out}")"

if [[ -f "${out}" ]] && unzip -tq "${out}" >/dev/null 2>&1; then
  echo "Using existing domain libraries KPAR at ${out}"
  exit 0
fi

fetch_kpar_release() {
  local url="https://github.com/${repo}/releases/download/v${version}/${artifact}"
  echo "Fetching domain libraries KPAR from ${url}"
  curl --fail --location \
    --retry 5 --retry-delay 5 --retry-all-errors \
    --connect-timeout 30 --max-time 600 \
    --output "${out}" "${url}"
  unzip -tq "${out}" >/dev/null
}

pack_kpar_from_dir() {
  local source_dir="$1"
  echo "Packing domain libraries KPAR from ${source_dir}"
  cargo run --quiet -p kpar --bin kpar-pack -- \
    --root "${source_dir}" \
    --version "${version}" \
    --output "${out}"
  unzip -tq "${out}" >/dev/null
}

resolve_source_dir() {
  if [[ -n "${SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR:-}" ]]; then
    echo "${SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR}"
    return 0
  fi
  local sibling="${repo_root}/../sysml-domain-libraries"
  if [[ -d "${sibling}/domain" || -d "${sibling}/technical" || -d "${sibling}/generic" ]]; then
    echo "${sibling}"
    return 0
  fi
  return 1
}

if fetch_kpar_release 2>/dev/null; then
  echo "Fetched domain libraries KPAR via GitHub release"
elif source_dir="$(resolve_source_dir)"; then
  pack_kpar_from_dir "${source_dir}"
  echo "Packed domain libraries KPAR locally"
else
  echo "Failed to fetch or pack domain libraries KPAR for ${repo}@${version}" >&2
  echo "Set SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR or publish release v${version} with asset ${artifact}" >&2
  exit 1
fi

echo "Domain libraries KPAR ready at ${out}"
