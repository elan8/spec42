import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.99.0";
const mochaTimeout = process.env.CI ? 60000 : 30000;
const serverBinary = path.resolve(
  __dirname,
  "..",
  "target",
  "debug",
  process.platform === "win32" ? "spec42.exe" : "spec42"
);

export default defineConfig({
  files: [
    "out/test/suite/interconnection.visualization.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "interconnection"),
  version: vscodeTestVersion,
  env: {
    SPEC42_SERVER_PATH: process.env.SPEC42_SERVER_PATH || serverBinary,
  },
  mocha: {
    timeout: mochaTimeout,
    ui: "bdd",
  },
});
