#!/usr/bin/env node
const cp = require("child_process");
const fs = require("fs");
const os = require("os");
const path = require("path");
const {
  downloadAndUnzipVSCode,
  resolveCliPathFromVSCodeExecutablePath,
  runTests,
} = require("@vscode/test-electron");

function getArg(name) {
  const idx = process.argv.indexOf(name);
  if (idx >= 0 && process.argv[idx + 1]) {
    return process.argv[idx + 1];
  }
  return undefined;
}

function globToRegExp(pattern) {
  const escaped = pattern.replace(/[.+^${}()|[\]\\]/g, "\\$&");
  const regex = escaped.replace(/\*/g, ".*").replace(/\?/g, ".");
  return new RegExp(`^${regex}$`, "i");
}

function resolveSingleFileGlob(input, cwd) {
  const absoluteInput = path.resolve(cwd, input);
  const dir = path.dirname(absoluteInput);
  const base = path.basename(absoluteInput);
  const re = globToRegExp(base);

  let entries;
  try {
    entries = fs.readdirSync(dir, { withFileTypes: true });
  } catch (err) {
    throw new Error(
      `Unable to read directory for VSIX glob: ${dir}\n${err instanceof Error ? err.message : String(err)}`
    );
  }

  const candidates = entries
    .filter((e) => e.isFile() && re.test(e.name))
    .map((e) => path.join(dir, e.name));

  if (candidates.length === 0) {
    throw new Error(`No VSIX matched pattern: ${input}`);
  }
  if (candidates.length > 1) {
    throw new Error(`VSIX pattern is ambiguous: ${input}\n${candidates.join("\n")}`);
  }
  return candidates[0];
}

function runCodeCli(cliPath, args) {
  if (process.platform === "win32") {
    cp.execFileSync("cmd.exe", ["/c", cliPath, ...args], { stdio: "inherit" });
    return;
  }
  cp.execFileSync(cliPath, args, { stdio: "inherit" });
}

function resolveVsix(input, cwd) {
  if (!input) {
    throw new Error("Missing required --vsix argument.");
  }
  if (input.includes("*") || input.includes("?")) {
    return resolveSingleFileGlob(input, cwd);
  }
  return path.resolve(cwd, input);
}

function writeSmokeTestHost(tempDir) {
  const pkg = {
    name: "spec42-vsix-smoke-host",
    version: "0.0.1",
    private: true,
    publisher: "spec42",
    engines: {
      vscode: "^1.85.0",
    },
  };
  const smokeTest = `"use strict";
const vscode = require("vscode");

async function sleep(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

async function run() {
  const extensionId = process.env.SPEC42_SMOKE_EXTENSION_ID || "Elan8.spec42";
  const docPath = process.env.SPEC42_SMOKE_DOC;
  if (!docPath) {
    throw new Error("SPEC42_SMOKE_DOC is required.");
  }

  const doc = await vscode.workspace.openTextDocument(vscode.Uri.file(docPath));
  await vscode.window.showTextDocument(doc);

  const deadline = Date.now() + 30000;
  let extension;
  while (!extension && Date.now() < deadline) {
    extension = vscode.extensions.getExtension(extensionId);
    if (!extension) {
      await sleep(250);
    }
  }
  if (!extension) {
    throw new Error(\`Extension not found: \${extensionId}\`);
  }

  await extension.activate();

  const commands = await vscode.commands.getCommands(true);
  const required = ["sysml.restartServer", "sysml.showVisualizer"];
  const missing = required.filter((cmd) => !commands.includes(cmd));
  if (missing.length > 0) {
    throw new Error(\`Extension activated but expected commands are missing: \${missing.join(", ")}\`);
  }
}

module.exports = { run };
`;

  fs.writeFileSync(path.join(tempDir, "package.json"), JSON.stringify(pkg, null, 2), "utf8");
  fs.writeFileSync(path.join(tempDir, "smoke.test.js"), smokeTest, "utf8");
}

async function main() {
  const cwd = process.cwd();
  const vsixArg = getArg("--vsix");
  const workspaceArg = getArg("--workspace") || "testFixture/workspaces/single-file";
  const docArg = getArg("--doc") || "testFixture/workspaces/single-file/SurveillanceDrone.sysml";
  const extensionId = getArg("--extension-id") || "Elan8.spec42";
  const serverPath = getArg("--server-path");

  const vsixPath = resolveVsix(vsixArg, cwd);
  const workspacePath = path.resolve(cwd, workspaceArg);
  const docPath = path.resolve(cwd, docArg);
  if (!fs.existsSync(vsixPath)) {
    throw new Error(`VSIX does not exist: ${vsixPath}`);
  }
  if (!fs.existsSync(workspacePath)) {
    throw new Error(`Workspace does not exist: ${workspacePath}`);
  }
  if (!fs.existsSync(docPath)) {
    throw new Error(`Smoke-test document does not exist: ${docPath}`);
  }

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "spec42-vsix-smoke-"));
  const tempExtensionsDir = fs.mkdtempSync(path.join(os.tmpdir(), "spec42-vsix-exts-"));
  const tempUserDataDir = fs.mkdtempSync(path.join(os.tmpdir(), "spec42-vsix-user-"));
  writeSmokeTestHost(tempDir);

  const vscodeExecutablePath = await downloadAndUnzipVSCode("stable");
  const cliPath = resolveCliPathFromVSCodeExecutablePath(vscodeExecutablePath);

  runCodeCli(cliPath, [
    "--user-data-dir",
    tempUserDataDir,
    "--extensions-dir",
    tempExtensionsDir,
    "--install-extension",
    vsixPath,
    "--force",
  ]);
  runCodeCli(cliPath, [
    "--user-data-dir",
    tempUserDataDir,
    "--extensions-dir",
    tempExtensionsDir,
    "--list-extensions",
    "--show-versions",
  ]);

  const extensionTestsEnv = {
    ...process.env,
    SPEC42_SMOKE_EXTENSION_ID: extensionId,
    SPEC42_SMOKE_DOC: docPath,
  };
  if (serverPath) {
    extensionTestsEnv.SPEC42_SERVER_PATH = serverPath;
  }

  await runTests({
    vscodeExecutablePath,
    extensionDevelopmentPath: tempDir,
    extensionTestsPath: path.join(tempDir, "smoke.test.js"),
    extensionTestsEnv,
    launchArgs: [
      workspacePath,
      "--skip-welcome",
      "--disable-updates",
      "--disable-workspace-trust",
      "--user-data-dir",
      tempUserDataDir,
      "--extensions-dir",
      tempExtensionsDir,
    ],
  });
}

main().catch((error) => {
  console.error("VSIX activation smoke test failed:", error);
  process.exit(1);
});
