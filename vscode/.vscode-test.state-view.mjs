import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  files: [
    "out/test/suite/state.visualization.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "state-view"),
  version: "stable",
  mocha: {
    timeout: 20000,
    ui: "bdd",
  },
});
