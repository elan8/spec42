#!/usr/bin/env bash
# Ratchet gate for metadata-related diagnostics on OMG 14c-Language Extensions.sysml.
# Fails when the count of tracked codes exceeds the budget in omg_14c_metadata_warning_budget.txt.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
BUDGET_FILE="$ROOT/crates/semantic_core/tests/fixtures/omg_14c_metadata_warning_budget.txt"
RELEASE_DIR="${SYSML_V2_RELEASE_DIR:-$ROOT/sysml-v2-release}"
OMG_FILE="$RELEASE_DIR/sysml/src/validation/14-Language Extensions/14c-Language Extensions.sysml"
REPORT_JSON="$(mktemp)"
trap 'rm -f "$REPORT_JSON"' EXIT

TRACKED_CODES=(
  "incompatible_type_kind"
  "incompatible_specializes_kind"
  "unresolved_redefines_target"
)

if [[ ! -f "$OMG_FILE" ]]; then
  echo "Skipping OMG 14c metadata ratchet: file not found at $OMG_FILE"
  exit 0
fi

if [[ ! -f "$BUDGET_FILE" ]]; then
  echo "Missing budget file: $BUDGET_FILE" >&2
  exit 1
fi

BUDGET="$(tr -d '[:space:]' < "$BUDGET_FILE")"
if ! [[ "$BUDGET" =~ ^[0-9]+$ ]]; then
  echo "Invalid budget in $BUDGET_FILE: expected non-negative integer" >&2
  exit 1
fi

cargo build -p spec42 --release

SPEC42="$ROOT/target/release/spec42"
if [[ ! -x "$SPEC42" ]]; then
  SPEC42="$ROOT/target/release/spec42.exe"
fi

"$SPEC42" check "$OMG_FILE" --format json > "$REPORT_JSON"

COUNT=0
for code in "${TRACKED_CODES[@]}"; do
  n="$(jq -r --arg code "$code" '
    [.documents[]?.diagnostics[]? | select(.code == $code)] | length
  ' "$REPORT_JSON")"
  COUNT=$((COUNT + n))
done

echo "OMG 14c metadata diagnostic count: $COUNT (budget: $BUDGET)"
for code in "${TRACKED_CODES[@]}"; do
  n="$(jq -r --arg code "$code" '
    [.documents[]?.diagnostics[]? | select(.code == $code)] | length
  ' "$REPORT_JSON")"
  echo "  $code: $n"
done

if (( COUNT > BUDGET )); then
  echo "Ratchet failed: count $COUNT exceeds budget $BUDGET" >&2
  echo "If the drop is intentional, lower the budget in $BUDGET_FILE" >&2
  exit 1
fi

if (( COUNT < BUDGET )); then
  echo "Note: count is below budget; consider lowering budget to $COUNT"
fi
