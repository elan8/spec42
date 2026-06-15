import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.99.0";
const mochaTimeout = 120000;
const serverBinary = path.resolve(
  __dirname,
  "..",
  "target",
  "debug",
  process.platform === "win32" ? "spec42.exe" : "spec42"
);

const workspaceFolder = process.env.SYSML_POWERSYSTEMS_DIR;
if (!workspaceFolder) {
  throw new Error(
    "SYSML_POWERSYSTEMS_DIR must be set to run power-systems visualization tests"
  );
}

export default defineConfig({
  files: [
    "out/test/suite/powersystems.visualization.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder,
  version: vscodeTestVersion,
  env: {
    SPEC42_SERVER_PATH: process.env.SPEC42_SERVER_PATH || serverBinary,
    SPEC42_TEST_DEBUG_INTERCONNECTION: "1",
  },
  mocha: {
    timeout: mochaTimeout,
    ui: "bdd",
  },
});
