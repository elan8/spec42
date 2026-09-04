#!/usr/bin/env bash
# Audit the production dependency tree of every npm package we ship.
#
# `npm audit` exits non-zero both for a real advisory and for a transient failure to reach the
# registry advisory endpoint. Only the first should fail CI, so a suspected network error is
# retried a few times before giving up. `--omit=dev` keeps the gate about shipped code: an
# advisory against test-only tooling (vitest and its tree) is Dependabot's problem, not a reason
# to fail a build.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

audit_dir() {
  local dir="$1" attempt output
  for attempt in 1 2 3; do
    if output=$(cd "$root/$dir" && npm audit --omit=dev 2>&1); then
      printf '%s\n' "$output"
      return 0
    fi
    printf '%s\n' "$output"
    if ! grep -qiE 'audit endpoint returned an error|network|ETIMEDOUT|ECONNRESET|EAI_AGAIN' <<<"$output"; then
      return 1
    fi
    echo "::warning::npm audit could not reach the registry for ${dir} (attempt ${attempt}/3); retrying in 15s"
    sleep 15
  done
  echo "::error::npm audit still cannot reach the registry for ${dir} after 3 attempts"
  return 1
}

status=0
for dir in vscode/diagram-renderer vscode; do
  echo "==> npm audit --omit=dev (${dir})"
  audit_dir "${dir}" || status=1
done
exit "${status}"
