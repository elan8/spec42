#!/usr/bin/env bash
set -euo pipefail

root_dir=$(cd "$(dirname "$0")/.." && pwd)
qa_dir=${SPEC42_VSCODE_QA_DIR:-"/tmp/spec42-vscode-qa"}
target_dir="$qa_dir/cargo-target"
vsix_path="$qa_dir/spec42-local-qa.vsix"
workspace_path="$qa_dir/spec42-diagram-qa.code-workspace"
extensions_dir="$qa_dir/extensions"
user_data_dir="$qa_dir/user-data"
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
  SPEC42_VSCODE_QA_DIR   generated QA state (default: /tmp/spec42-vscode-qa)
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

mkdir -p "$qa_dir" "$extensions_dir" "$user_data_dir"

stdlib_cache="$root_dir/.cache/sysml-stdlib-kpar-2026-04"
if [[ ! -d "$stdlib_cache" ]]; then
	echo "Fetching Spec42's pinned SysML standard-library bundle..."
	"$root_dir/scripts/fetch-stdlib-bundle.sh"
fi

echo "Building repository-owned Rust generator plugins..."
"$root_dir/scripts/build-repository-generator-plugins.sh"

if [[ ! -f "$plugin_path" ]]; then
	echo "error: diagram plugin build did not produce $plugin_path" >&2
	exit 1
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
echo
echo "In VS Code, run: Spec42: Open Diagram"
echo "The workspace contains every repository example; open an example's Views.sysml first."
echo "For a working projection, open timer/Views.sysml and choose State Transition View."
echo "General and Interconnection are authored there and should report their typed queries as incomplete."

if [[ "$open_vscode" -eq 1 ]]; then
	echo "Opening the isolated VS Code QA instance..."
	"$code_bin" \
		--new-window \
		--extensions-dir "$extensions_dir" \
		--user-data-dir "$user_data_dir" \
		"$workspace_path" \
		--goto "$model_path:1:1"
else
	echo
	echo "Open it later with:"
	printf '  %q --new-window --extensions-dir %q --user-data-dir %q %q --goto %q\n' \
		"$code_bin" "$extensions_dir" "$user_data_dir" "$workspace_path" "$model_path:1:1"
fi
