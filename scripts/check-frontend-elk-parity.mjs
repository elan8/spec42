import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");

function readJson(relativePath) {
  return JSON.parse(fs.readFileSync(path.join(repoRoot, relativePath), "utf8"));
}

function dependencyRange(packageJsonPath) {
  return readJson(packageJsonPath).dependencies?.elkjs;
}

function lockVersion(lockPath) {
  return readJson(lockPath).packages?.["node_modules/elkjs"]?.version;
}

const checks = [
  {
    name: "vscode",
    range: dependencyRange("vscode/package.json"),
    lock: lockVersion("vscode/package-lock.json"),
  },
  {
    name: "shared diagram renderer",
    range: dependencyRange("shared/diagram-renderer/package.json"),
    lock: lockVersion("shared/diagram-renderer/package-lock.json"),
  },
];

const versions = new Set(checks.map((check) => check.lock));
const ranges = new Set(checks.map((check) => check.range));

if (versions.size !== 1 || ranges.size !== 1) {
  console.error("ELK dependency mismatch:");
  for (const check of checks) {
    console.error(`- ${check.name}: package.json=${check.range ?? "(missing)"} lock=${check.lock ?? "(missing)"}`);
  }
  process.exit(1);
}

const [version] = versions;
console.log(`ELK dependency parity OK: elkjs ${version}`);
