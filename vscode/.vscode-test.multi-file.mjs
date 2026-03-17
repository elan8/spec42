import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  files: [
    "out/test/suite/multiFile.multifile.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "multi-file"),
  version: "stable",
  mocha: {
    timeout: 30000,
    ui: "bdd",
  },
});
