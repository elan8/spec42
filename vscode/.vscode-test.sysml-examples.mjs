import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default defineConfig({
  files: [
    "out/test/suite/sysmlExamples.generalView.visualization.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: "C:/Git/sysml-examples",
  extensionTestsEnv: {
    SPEC42_SERVER_PATH: path.resolve(__dirname, "..", "target", "debug", "spec42.exe"),
  },
  version: "stable",
  mocha: {
    timeout: 90000,
    ui: "bdd",
  },
});

