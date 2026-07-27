#!/usr/bin/env bash
# Compatibility wrapper — prefer scripts/fetch-kpar-libraries-bundle.sh domain
set -euo pipefail
repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -n "${SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP:-}" ]]; then
  export SPEC42_KPAR_LIBRARY_BUNDLE_DOMAIN="${SPEC42_DOMAIN_LIBRARIES_BUNDLE_ZIP}"
fi
if [[ -n "${SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR:-}" ]]; then
  export SPEC42_KPAR_LIBRARY_SOURCE_DIR_DOMAIN="${SPEC42_DOMAIN_LIBRARIES_SOURCE_DIR}"
fi
exec bash "${repo_root}/scripts/fetch-kpar-libraries-bundle.sh" domain
