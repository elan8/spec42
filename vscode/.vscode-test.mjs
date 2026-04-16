import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.85.0";

export default defineConfig({
  files: [
    "out/test/suite/extension.test.js",
    "out/test/suite/visualization.test.js",
    "out/test/suite/prepareData.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "single-file"),
  version: vscodeTestVersion,
  mocha: {
    timeout: 20000,
    ui: "bdd",
  },
});
