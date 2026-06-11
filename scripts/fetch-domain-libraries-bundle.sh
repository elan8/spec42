#!/usr/bin/env bash
# Fetch the pinned sysml-domain-libraries archive for embedded domain-library builds.

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
out="${SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP:-${repo_root}/.cache/sysml-domain-libraries-${version}.zip}"

mkdir -p "$(dirname "${out}")"

if [[ -f "${out}" ]] && unzip -tq "${out}" >/dev/null 2>&1; then
  echo "Using existing domain libraries bundle at ${out}"
  exit 0
fi

fetch_via_archive() {
  local url="https://github.com/${repo}/archive/${version}.zip"
  echo "Fetching domain libraries bundle from ${url}"
  curl --fail --location \
    --retry 5 --retry-delay 5 --retry-all-errors \
    --connect-timeout 30 --max-time 600 \
    --output "${out}" "${url}"
  unzip -tq "${out}" >/dev/null
}

fetch_via_git() {
  local tmp
  tmp="$(mktemp -d)"
  trap 'rm -rf "${tmp}"' RETURN

  echo "Archive download failed; falling back to git clone of ${repo}@${version}"
  if git clone --depth 1 --branch "${version}" "https://github.com/${repo}.git" "${tmp}/checkout" 2>/dev/null; then
    :
  else
    git clone --depth 1 "https://github.com/${repo}.git" "${tmp}/checkout"
    git -C "${tmp}/checkout" checkout "${version}"
  fi
  test -d "${tmp}/checkout/generic" || test -d "${tmp}/checkout/domain" || test -d "${tmp}/checkout/technical"

  rm -f "${out}"
  (cd "${tmp}" && zip -qr "${out}" "checkout")
  unzip -tq "${out}" >/dev/null
}

if fetch_via_archive; then
  echo "Fetched domain libraries bundle via GitHub archive"
elif fetch_via_git; then
  echo "Fetched domain libraries bundle via git clone fallback"
else
  echo "Failed to fetch domain libraries bundle for ${repo}@${version}" >&2
  exit 1
fi

echo "Domain libraries bundle ready at ${out}"
