#!/usr/bin/env node
const fs = require("fs");
const path = require("path");
const cp = require("child_process");

const rootDir = path.join(__dirname, "..");
const binaryName = process.platform === "win32"
  ? "spec42.exe"
  : "spec42";

function assertExists(filePath, label) {
  if (!fs.existsSync(filePath)) {
    throw new Error(`Missing ${label}: ${filePath}`);
  }
}

function verifyStagedLayout() {
  const requiredPaths = [
    ["compiled extension output", path.join(rootDir, "out", "extension.js")],
    ["webview bundle", path.join(rootDir, "media", "webview", "visualizer.js")],
    ["ELK worker", path.join(rootDir, "media", "webview", "elkWorker.js")],
    ["D3 vendor bundle", path.join(rootDir, "media", "vendor", "d3.min.js")],
    ["ELK vendor bundle", path.join(rootDir, "media", "vendor", "elk.bundled.js")],
  ];

  for (const [label, targetPath] of requiredPaths) {
    assertExists(targetPath, label);
  }

  // Bundled server binaries are staged by the release workflow; they are not expected
  // to exist in a normal dev workspace unless you intentionally staged them.
  const requireBundled = process.env.SPEC42_REQUIRE_BUNDLED === "1";
  const bundledRoots = [
    ["Linux", path.join(rootDir, "server", "linux-x64"), path.join(rootDir, "server", "linux-x64", "spec42")],
    ["macOS", path.join(rootDir, "server", "darwin-x64"), path.join(rootDir, "server", "darwin-x64", "spec42")],
    ["macOS (Apple Silicon)", path.join(rootDir, "server", "darwin-arm64"), path.join(rootDir, "server", "darwin-arm64", "spec42")],
    ["Windows", path.join(rootDir, "server", "win32-x64"), path.join(rootDir, "server", "win32-x64", "spec42.exe")],
  ];
  for (const [osLabel, dirPath, binPath] of bundledRoots) {
    const shouldCheck = requireBundled || fs.existsSync(dirPath);
    if (shouldCheck) {
      assertExists(binPath, `bundled ${osLabel} server`);
    }
  }
}

function listVsixEntries(vsixPath) {
  try {
    const output = cp.execFileSync(
      "unzip",
      ["-Z1", vsixPath],
      { encoding: "utf8", stdio: ["ignore", "pipe", "pipe"] },
    );
    return output
      .split(/\r?\n/)
      .map((line) => line.trim())
      .filter(Boolean);
  } catch (error) {
    throw new Error(`Unable to inspect VSIX with unzip: ${error instanceof Error ? error.message : String(error)}`);
  }
}

function verifyVsixContents(vsixPath) {
  assertExists(vsixPath, "VSIX artifact");
  const entries = new Set(listVsixEntries(vsixPath));
  const requiredEntries = [
    "extension/out/extension.js",
    "extension/media/webview/visualizer.js",
    "extension/media/webview/elkWorker.js",
    "extension/media/vendor/d3.min.js",
    "extension/media/vendor/elk.bundled.js",
    "extension/server/linux-x64/spec42",
    "extension/server/darwin-x64/spec42",
    "extension/server/darwin-arm64/spec42",
    "extension/server/win32-x64/spec42.exe",
  ];

  const missing = requiredEntries.filter((entry) => !entries.has(entry));
  if (missing.length > 0) {
    throw new Error(`VSIX is missing expected entries: ${missing.join(", ")}`);
  }
}

function findVsixArg() {
  const args = process.argv.slice(2);
  const idx = args.findIndex((arg) => arg === "--vsix");
  if (idx >= 0 && args[idx + 1]) {
    return args[idx + 1];
  }
  return null;
}

function main() {
  verifyStagedLayout();
  const vsixPath = findVsixArg();
  if (vsixPath) {
    verifyVsixContents(path.resolve(process.cwd(), vsixPath));
  }
  console.log("Packaging layout verification succeeded.");
}

main();
