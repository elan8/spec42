import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.99.0";
const testOut = process.env.SPEC42_VSCODE_TEST_OUT || "out";

export default defineConfig({
  files: [
    path.join(__dirname, testOut, "test/suite/examplesViewProvider.test.js"),
    path.join(__dirname, testOut, "test/suite/snippets.test.js"),
    path.join(__dirname, testOut, "test/suite/statusBar.test.js"),
    path.join(__dirname, testOut, "test/suite/workspaceLifecycle.test.js"),
    path.join(__dirname, testOut, "test/suite/workspaceDiagnostics.test.js"),
    path.join(__dirname, testOut, "test/suite/modelExplorerProvider.test.js"),
    path.join(__dirname, testOut, "test/suite/baseVisualizationPanelController.test.js"),
    path.join(__dirname, testOut, "test/suite/updateFlow.test.js"),
    path.join(__dirname, testOut, "test/suite/visualizationGate.test.js"),
    path.join(__dirname, testOut, "test/suite/dtoAdapter.test.js"),
    path.join(__dirname, testOut, "test/suite/modelFetcher.test.js"),
    path.join(__dirname, testOut, "test/suite/renderTracker.test.js"),
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "ux-unit"),
  version: vscodeTestVersion,
  mocha: {
    timeout: 20000,
    ui: "bdd",
  },
});
