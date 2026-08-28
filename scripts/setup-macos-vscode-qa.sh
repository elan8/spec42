#!/usr/bin/env bash
set -euo pipefail

root_dir=$(cd "$(dirname "$0")/.." && pwd)
abi_token=$(python3 - "$root_dir/docs/generation/generator-abi.json" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as stream:
	print(json.load(stream)["compatibilityToken"].removeprefix("0x"))
PY
)
if [[ -n "${SPEC42_VSCODE_QA_DIR:-}" ]]; then
	qa_dir=$SPEC42_VSCODE_QA_DIR
else
	qa_dir="$root_dir/.cache/vscode-qa-$abi_token"
fi
target_dir="$qa_dir/cargo-target"
model_dir="$root_dir/examples"
model_path="$model_dir/timer/Views.sysml"
plugin_path="$root_dir/vscode/generators/diagram.wasm"
server_path="$target_dir/release/spec42"
open_vscode=1

usage() {
	cat <<'EOF'
Usage: scripts/setup-macos-vscode-qa.sh [--no-open]

Builds and installs an isolated local VS Code QA environment for the packaged
Spec42 server, Rust diagram generator, and D3/ELK diagram webview. It does not
change global VS Code settings or the normal extension directory.

Environment override:
  SPEC42_VSCODE_QA_DIR   use a different QA state root
EOF
}

while [[ $# -gt 0 ]]; do
	case "$1" in
		--no-open) open_vscode=0 ;;
		-h|--help) usage; exit 0 ;;
		*) echo "error: unknown argument: $1" >&2; usage >&2; exit 2 ;;
	esac
	shift
done

if [[ "$(uname -s)" != "Darwin" ]]; then
	echo "error: this setup script is intended for macOS" >&2
	exit 1
fi

for command_name in cargo npm python3 rustc rustup; do
	if ! command -v "$command_name" >/dev/null 2>&1; then
		echo "error: required command is not on PATH: $command_name" >&2
		exit 1
	fi
done

if command -v code >/dev/null 2>&1; then
	code_bin=$(command -v code)
elif [[ -x "/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code" ]]; then
	code_bin="/Applications/Visual Studio Code.app/Contents/Resources/app/bin/code"
else
	echo "error: VS Code was not found" >&2
	echo "Install Visual Studio Code, or run 'Shell Command: Install code command in PATH' from its command palette." >&2
	exit 1
fi

for required_path in \
	"$root_dir/Cargo.toml" \
	"$root_dir/generator-plugins/Cargo.toml" \
	"$root_dir/vscode/package.json" \
	"$root_dir/vscode/diagram-renderer/package.json" \
	"$model_path"; do
	if [[ ! -e "$required_path" ]]; then
		echo "error: required path is missing: $required_path" >&2
		exit 1
	fi
done

mkdir -p "$qa_dir"
# VS Code routes another invocation with the same user-data directory to the already-running
# process. Give every QA launch a fresh process and extension installation so a rebuilt server or
# packaged plugin cannot be shadowed by the previous extension host. The Cargo target directory
# remains above this per-run directory so repeated QA builds still reuse compiled dependencies.
run_dir=$(mktemp -d "$qa_dir/run.XXXXXX")
vsix_path="$run_dir/spec42-local-qa.vsix"
workspace_path="$run_dir/spec42-diagram-qa.code-workspace"
extensions_dir="$run_dir/extensions"
# VS Code appends its IPC socket name to the user-data path. Keep this disposable profile under
# the short macOS temporary-directory alias so the resulting Unix socket stays below macOS's
# 104-byte sockaddr_un limit even when the repository checkout path is long.
user_data_dir=$(mktemp -d /tmp/spec42-vscode-qa.XXXXXX)
mkdir -p "$extensions_dir"

stdlib_cache="$root_dir/.cache/sysml-stdlib-kpar-2026-04"
if [[ ! -d "$stdlib_cache" ]]; then
	echo "Fetching Spec42's pinned SysML standard-library bundle..."
	"$root_dir/scripts/fetch-stdlib-bundle.sh"
fi

echo "Building the matching Spec42 server with its embedded standard library..."
CARGO_TARGET_DIR="$target_dir" cargo build \
	--manifest-path "$root_dir/Cargo.toml" \
	-p server \
	--bin spec42 \
	--release \
	--no-default-features \
	--features embed-stdlib

echo "Installing diagram renderer dependencies..."
(
	cd "$root_dir/vscode/diagram-renderer"
	npm ci
)

echo "Installing VS Code extension dependencies and packaging the VSIX..."
(
	cd "$root_dir/vscode"
	npm ci
	npm run package -- --out "$vsix_path"
)

if [[ ! -f "$plugin_path" ]]; then
	echo "error: extension packaging did not produce $plugin_path" >&2
	exit 1
fi

echo "Installing the VSIX into an isolated extension directory..."
"$code_bin" \
	--extensions-dir "$extensions_dir" \
	--user-data-dir "$user_data_dir" \
	--install-extension "$vsix_path" \
	--force

python3 - "$workspace_path" "$model_dir" "$server_path" <<'PY'
import json
import sys

workspace_path, model_dir, server_path = sys.argv[1:]
workspace = {
    "folders": [{"path": model_dir}],
    "settings": {
        "spec42.serverPath": server_path,
        "spec42.diagramViewer.pluginPath": "",
    },
}
with open(workspace_path, "w", encoding="utf-8") as stream:
    json.dump(workspace, stream, indent=2)
    stream.write("\n")
PY

echo
echo "QA environment ready:"
echo "  VSIX:      $vsix_path"
echo "  Spec42:    $server_path"
echo "  Plugin:    $plugin_path"
echo "  Workspace: $workspace_path"
echo "  Runtime:   $run_dir"
echo "  Profile:   $user_data_dir"
echo
echo "In VS Code, run: Spec42: Open Diagram"
echo "The workspace contains every repository example; open an example's Views.sysml first."
echo "Open timer/Views.sysml and choose any authored diagram view to inspect its typed projection."
echo "Incomplete products now identify only concrete unsupported or unavailable semantic facts."

if [[ "$open_vscode" -eq 1 ]]; then
	echo "Opening the isolated VS Code QA instance..."
	"$code_bin" \
		--new-window \
		--extensions-dir "$extensions_dir" \
		--user-data-dir "$user_data_dir" \
		--goto "$model_path:1:1" \
		"$workspace_path"
else
	echo
	echo "Open it later with:"
	printf '  %q --new-window --extensions-dir %q --user-data-dir %q --goto %q %q\n' \
		"$code_bin" "$extensions_dir" "$user_data_dir" "$model_path:1:1" "$workspace_path"
fi
