import { defineConfig } from "@vscode/test-cli";
import path from "path";
import { fileURLToPath } from "url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const vscodeTestVersion = process.env.VSCODE_TEST_VERSION || "1.85.0";

export default defineConfig({
  files: [
    "out/test/suite/interconnection.drone.visualization.test.js",
    "out/test/suite/svgArtifacts.interconnectionDrone.test.js",
  ],
  extensionDevelopmentPath: __dirname,
  workspaceFolder: path.resolve(__dirname, "testFixture", "workspaces", "interconnection-drone"),
  version: vscodeTestVersion,
  mocha: {
    timeout: 30000,
    ui: "bdd",
  },
});
