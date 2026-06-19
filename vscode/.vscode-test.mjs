import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.99.0";
const serverBinary = path.resolve(
  __dirname,
  "..",
  "target",
  "debug",
  process.platform === "win32" ? "spec42.exe" : "spec42"
);

export default defineConfig({
  files: [
    "out/test/suite/extension.test.js",
    "out/test/suite/visualization.test.js",
    "out/test/suite/placeholder.test.js",
    "out/test/suite/messageHandlers.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "single-file"),
  version: vscodeTestVersion,
  env: {
    SPEC42_SERVER_PATH: process.env.SPEC42_SERVER_PATH || serverBinary,
  },
  mocha: {
    timeout: process.env.CI ? 60000 : 45000,
    ui: "bdd",
  },
});
