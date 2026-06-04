import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.99.0";
const testOut = process.env.SPEC42_VSCODE_TEST_OUT || "out";

export default defineConfig({
  files: [
    path.join(testOut, "test/suite/examplesViewProvider.test.js"),
    path.join(testOut, "test/suite/snippets.test.js"),
    path.join(testOut, "test/suite/statusBar.test.js"),
    path.join(testOut, "test/suite/workspaceDiagnostics.test.js"),
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "single-file"),
  version: vscodeTestVersion,
  mocha: {
    timeout: 20000,
    ui: "bdd",
  },
});
