#!/usr/bin/env bash
# Fetch or pack pinned managed KPAR libraries from config/libraries/*.json.
#
# Usage:
#   bash scripts/fetch-kpar-libraries-bundle.sh            # all libraries
#   bash scripts/fetch-kpar-libraries-bundle.sh domain     # one id
#   bash scripts/fetch-kpar-libraries-bundle.sh domain method
#
# Resolution order per library:
#   1. Existing usable .kpar at SPEC42_KPAR_LIBRARY_BUNDLE_<ID> or .cache/<artifact>
#   2. GitHub release asset
#   3. SPEC42_KPAR_LIBRARY_SOURCE_DIR_<ID>
#   4. Sibling path from pack.siblingRelative

set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
libraries_dir="${repo_root}/config/libraries"

if [[ ! -d "${libraries_dir}" ]]; then
  echo "Missing ${libraries_dir}" >&2
  exit 1
fi

cd "${repo_root}"

download_url() {
  local url="$1"
  local dest="$2"
  if command -v curl >/dev/null 2>&1; then
    curl --fail --location \
      --retry 5 --retry-delay 5 --retry-all-errors \
      --connect-timeout 30 --max-time 600 \
      --output "${dest}" "${url}"
  elif command -v wget >/dev/null 2>&1; then
    wget --quiet --tries=5 --timeout=30 --output-document="${dest}" "${url}"
  else
    echo "Need curl or wget to download ${url}" >&2
    return 1
  fi
}

upper_id() {
  echo "$1" | tr '[:lower:]-' '[:upper:]_'
}

list_library_ids() {
  node --input-type=module -e '
import fs from "node:fs";
import path from "node:path";
const dir = path.join(process.cwd(), "config", "libraries");
for (const name of fs.readdirSync(dir).filter((n) => n.endsWith(".json")).sort()) {
  const raw = JSON.parse(fs.readFileSync(path.join(dir, name), "utf8"));
  const id = typeof raw.id === "string" && raw.id.trim() ? raw.id : path.basename(name, ".json");
  console.log(id);
}
'
}

library_field() {
  local id="$1"
  local expr="$2"
  node --input-type=module -e '
import fs from "node:fs";
import path from "node:path";
const wanted = process.argv[1];
const expr = process.argv[2];
const dir = path.join(process.cwd(), "config", "libraries");
for (const name of fs.readdirSync(dir).filter((n) => n.endsWith(".json"))) {
  const file = path.join(dir, name);
  const raw = JSON.parse(fs.readFileSync(file, "utf8"));
  const id = typeof raw.id === "string" && raw.id.trim() ? raw.id : path.basename(name, ".json");
  if (id !== wanted) continue;
  const value = Function("c", `return (${expr});`)(raw);
  if (value === undefined || value === null) process.exit(2);
  process.stdout.write(String(value));
  process.exit(0);
}
process.exit(1);
' "${id}" "${expr}"
}

pack_library() {
  local id="$1"
  local version="$2"
  local kind="$3"
  local source_dir="$4"
  local archive_prefix="$5"
  local out="$6"

  echo "Packing ${id} KPAR from ${source_dir}"
  if [[ "${kind}" == "named-prefix" ]]; then
    cargo run --quiet -p kpar --bin kpar-pack -- \
      --root "${source_dir}" \
      --name "elan8-${id}-libraries" \
      --version "${version}" \
      --named-source "${archive_prefix}=${source_dir}" \
      --output "${out}"
  else
    cargo run --quiet -p kpar --bin kpar-pack -- \
      --root "${source_dir}" \
      --name "elan8-${id}-libraries" \
      --version "${version}" \
      --output "${out}"
  fi
  unzip -tq "${out}" >/dev/null
}

fetch_one() {
  local id="$1"
  local version repo artifact kind sibling prefix
  if ! version="$(library_field "${id}" "c.version")"; then
    echo "Unknown KPAR library id '${id}' (no matching config/libraries/*.json)" >&2
    exit 1
  fi
  repo="$(library_field "${id}" "c.repo")"
  artifact="$(library_field "${id}" "c.artifact || ('elan8-' + (c.id || '${id}') + '-libraries-' + c.version + '.kpar')")"
  kind="$(library_field "${id}" "c.pack.kind")"
  sibling="$(library_field "${id}" "c.pack.siblingRelative")"
  prefix="$(library_field "${id}" "c.pack.archivePrefix || c.id || '${id}'")"

  local env_bundle="SPEC42_KPAR_LIBRARY_BUNDLE_$(upper_id "${id}")"
  local env_source="SPEC42_KPAR_LIBRARY_SOURCE_DIR_$(upper_id "${id}")"
  local out="${!env_bundle:-${repo_root}/.cache/${artifact}}"

  mkdir -p "$(dirname "${out}")"
  if [[ -f "${out}" ]] && unzip -tq "${out}" >/dev/null 2>&1; then
    echo "Using existing ${id} KPAR at ${out}"
    return 0
  fi

  local url="https://github.com/${repo}/releases/download/v${version}/${artifact}"
  echo "Fetching ${id} KPAR from ${url}"
  if download_url "${url}" "${out}" && unzip -tq "${out}" >/dev/null 2>&1; then
    echo "Fetched ${id} KPAR via GitHub release"
    return 0
  fi
  rm -f "${out}"

  local source_dir=""
  if [[ -n "${!env_source:-}" && -d "${!env_source}" ]]; then
    source_dir="${!env_source}"
  elif [[ -d "${repo_root}/${sibling}" ]]; then
    source_dir="${repo_root}/${sibling}"
  else
    echo "Failed to fetch or pack ${id} KPAR for ${repo}@${version}" >&2
    echo "Set ${env_source}, place sibling ${sibling}, or publish release v${version} with asset ${artifact}" >&2
    exit 1
  fi

  pack_library "${id}" "${version}" "${kind}" "${source_dir}" "${prefix}" "${out}"
  echo "Packed ${id} KPAR locally at ${out}"
}

ids=("$@")
if [[ ${#ids[@]} -eq 0 ]]; then
  while IFS= read -r id; do
    if [[ -n "${id}" ]]; then
      ids+=("${id}")
    fi
  done < <(list_library_ids)
fi

for id in "${ids[@]}"; do
  fetch_one "${id}"
done

echo "KPAR libraries ready: ${ids[*]}"
