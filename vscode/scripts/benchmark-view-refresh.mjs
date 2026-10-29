#!/usr/bin/env node
// Launch an already installed VS Code in a new, disposable profile. Never downloads software.
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { parseArgs } from "node:util";
import { spawn, execFileSync } from "node:child_process";

const { values } = parseArgs({ options: {
  code: { type: "string" }, server: { type: "string" }, workspace: { type: "string" },
  document: { type: "string" }, output: { type: "string" },
  "library-input": { type: "string", multiple: true },
  iterations: { type: "string", default: "20" }, "burst-size": { type: "string", default: "10" },
  "inactive-iterations": { type: "string", default: "1" },
  visible: { type: "boolean", default: false },
} });
for (const name of ["code", "server", "workspace", "document", "output"]) {
  if (!values[name]) throw new Error(`--${name} is required`);
}
if (!values["library-input"]?.length) throw new Error("Supply --library-input for every admitted source/archive library root");
for (const name of ["iterations", "inactive-iterations", "burst-size"]) {
  if (!Number.isSafeInteger(Number(values[name])) || Number(values[name]) < 1) throw new Error(`--${name} must be a positive integer`);
}
for (const input of [values.code, values.server, values.workspace, values.document, ...values["library-input"]]) {
  if (!fs.existsSync(input)) throw new Error(`Missing input: ${input}`);
}
const extension = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const runner = path.join(extension, "out", "test", "benchmark", "viewRefresh.js");
if (!fs.existsSync(runner)) throw new Error("Compile the extension before running the benchmark");
const output = path.resolve(values.output);
const profile = `${output}.profile`;
if (fs.existsSync(output) || fs.existsSync(profile)) throw new Error("Use a new output path; existing evidence/profiles are never overwritten");
const userData = path.join(profile, "user-data");
const extensions = path.join(profile, "extensions");
fs.mkdirSync(path.join(userData, "User"), { recursive: true });
fs.mkdirSync(extensions, { recursive: true });
fs.writeFileSync(path.join(userData, "User", "settings.json"), JSON.stringify({
  "files.autoSave": "off", "files.hotExit": "off",
  "spec42.performanceLogging.enabled": true,
  "security.workspace.trust.enabled": false,
  "extensions.autoUpdate": false, "extensions.autoCheckUpdates": false,
  "update.mode": "none", "telemetry.telemetryLevel": "off",
  "workbench.startupEditor": "none",
}, null, 2));
const args = [
  `--user-data-dir=${userData}`, `--extensions-dir=${extensions}`,
  `--extensionDevelopmentPath=${extension}`, `--extensionTestsPath=${runner}`,
  "--new-window", "--disable-gpu", "--skip-welcome", "--skip-release-notes",
  "--disable-background-timer-throttling", "--disable-renderer-backgrounding",
  `--folder-uri=${pathToFileURL(path.resolve(values.workspace)).href}`,
];
const env = { ...process.env,
  SPEC42_VIEW_WORKSPACE: path.resolve(values.workspace), SPEC42_VIEW_DOCUMENT: path.resolve(values.document),
  SPEC42_VIEW_REPORT: output, SPEC42_VIEW_USER_DATA: userData,
  SPEC42_SERVER_PATH: path.resolve(values.server),
  SPEC42_VIEW_LIBRARY_INPUTS: JSON.stringify(values["library-input"].map(input => path.resolve(input))),
  SPEC42_VIEW_ITERATIONS: values.iterations, SPEC42_VIEW_BURST_SIZE: values["burst-size"],
  SPEC42_VIEW_INACTIVE_ITERATIONS: values["inactive-iterations"],
  SPEC42_VIEW_LAUNCH_ARGS: JSON.stringify(args),
};
delete env.ELECTRON_RUN_AS_NODE;
fs.writeFileSync(`${output}.launch.json`, JSON.stringify({ executable: path.resolve(values.code), args,
  visibleLaunchRequested: values.visible,
  revision: execFileSync("git", ["rev-parse", "HEAD"], { cwd: extension, encoding: "utf8", windowsHide: true }).trim(),
  workingTree: execFileSync("git", ["status", "--short"], { cwd: extension, encoding: "utf8", windowsHide: true }),
  environment: Object.fromEntries(Object.entries(env).filter(([key]) => key.startsWith("SPEC42_"))),
}, null, 2));
// Launch intent does not establish presentation: the report must confirm focus and visible frames.
const child = spawn(path.resolve(values.code), args, { env, windowsHide: !values.visible, stdio: ["ignore", "pipe", "pipe"] });
const log = fs.createWriteStream(`${output}.console.log`);
child.stdout.pipe(log, { end: false });
child.stderr.pipe(log, { end: false });
let timedOut = false;
const timeout = setTimeout(() => { timedOut = true; child.kill(); }, 20 * 60_000);
child.on("error", error => { clearTimeout(timeout); log.end(String(error)); process.exitCode = 1; });
child.on("close", code => {
  clearTimeout(timeout);
  log.end();
  if (timedOut || code !== 0 || !fs.existsSync(output)) {
    console.error(`VS Code measurement failed (${timedOut ? "timeout" : code}); see ${output}.console.log`);
    process.exitCode = 1;
    return;
  }
  const report = JSON.parse(fs.readFileSync(output, "utf8"));
  console.log(JSON.stringify({ status: report.status, timingQualification: report.timingQualification, samples: report.samples.length, output }));
  if (report.status !== "passed") process.exitCode = 1;
});
