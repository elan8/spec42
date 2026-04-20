# Changes Since v0.19.0

This document captures all recorded git changes from `v0.19.0` to the pre-reset `HEAD` (`a90a40e`), before the repository was reset.

## Range

- From tag: `v0.19.0` (commit: `18b3a47`)
- To commit: `a90a40e`
- Net diff: **45 files changed, 4436 insertions, 1154 deletions**

## Commit History (Oldest to Newest)

1. `d81fd50` (2026-04-17) - Enhance workspace visualization support and refactor related functions
2. `86014ad` (2026-04-17) - Add new visualization commands and enhance testing framework
3. `370ac60` (2026-04-17) - Enhance sysml/model parameter parsing and workspace visualization support
4. `8e88ecc` (2026-04-17) - Enhance visualization package dropdown behavior and styling
5. `a149caa` (2026-04-17) - Refactor loading UI and improve package sorting in visualizer
6. `ff78807` (2026-04-17) - Implement part instance rendering checks and filter unnamed part usages
7. `735b8f1` (2026-04-17) - Update sysml-v2-parser dependency to local path and enhance general view rendering
8. `a90a40e` (2026-04-20) - Refactor SysML diagram scene handling and enhance package scoping

## Key Themes Across Those Commits

- Expanded and refactored visualization session/orchestration flow.
- Added and adjusted general view scene handling and package scoping behavior.
- Updated SysML model parsing/projection and workspace-related backend handling.
- Increased visualization test coverage (new suites and updates to existing tests).
- Adjusted extension/webview transport, protocol, and rendering pipeline integration.

## File-Level Changes (`v0.19.0..a90a40e`)

### Added

- `spec42-core/tests/fixtures/apollo_general_view_slice.sysml`
- `todo.md`
- `vscode/src/test/suite/generalViewScene.test.ts`
- `vscode/src/test/suite/sessionCoordinator.test.ts`
- `vscode/src/visualization/extensionTransport.ts`
- `vscode/src/visualization/generalViewScene.ts`
- `vscode/src/visualization/protocol.ts`
- `vscode/src/visualization/sessionCoordinator.ts`
- `vscode/src/visualization/webview/bootstrap.ts`
- `vscode/src/visualization/webview/transport.ts`
- `vscode/src/visualization/webview/uiControllers.ts`

### Modified

- `Cargo.lock`
- `Cargo.toml`
- `semantic-model/src/graph.rs`
- `semantic-model/src/graph_builder/expressions.rs`
- `semantic-model/src/workspace_uri.rs`
- `spec42-core/src/lsp_runtime/custom.rs`
- `spec42-core/src/lsp_runtime/lifecycle.rs`
- `spec42-core/src/views/diagram.rs`
- `spec42-core/src/views/dto.rs`
- `spec42-core/src/views/ibd.rs`
- `spec42-core/src/views/model.rs`
- `spec42-core/src/views/model_params.rs`
- `spec42-core/src/views/model_projection.rs`
- `spec42-core/tests/integration/model.rs`
- `spec42-core/tests/integration/workspace.rs`
- `vscode/.vscode-test.mjs`
- `vscode/media/webview/visualizer.html`
- `vscode/src/extension.ts`
- `vscode/src/providers/lspModelProvider.ts`
- `vscode/src/providers/sysmlModelTypes.ts`
- `vscode/src/test/suite/lspModelProvider.test.ts`
- `vscode/src/test/suite/messageHandlers.test.ts`
- `vscode/src/test/suite/modelFetcher.test.ts`
- `vscode/src/test/suite/updateFlow.test.ts`
- `vscode/src/test/suite/visualization.test.ts`
- `vscode/src/visualization/messageHandlers.ts`
- `vscode/src/visualization/modelFetcher.ts`
- `vscode/src/visualization/styles.ts`
- `vscode/src/visualization/updateFlow.ts`
- `vscode/src/visualization/visualizationPanel.ts`
- `vscode/src/visualization/webview/index.ts`
- `vscode/src/visualization/webview/orchestrator.ts`
- `vscode/src/visualization/webview/renderers/generalView.ts`
- `vscode/src/visualization/webview/renderers/sysmlNodeBuilder.ts`
