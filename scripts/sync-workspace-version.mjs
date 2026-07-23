import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const repoRoot = process.cwd();
const check = process.argv.includes("--check");

const cargoToml = fs.readFileSync(path.join(repoRoot, "Cargo.toml"), "utf8");
const versionMatch = cargoToml.match(
  /\[workspace\.package\]\s*\r?\nversion\s*=\s*"([^"]+)"/
);
if (!versionMatch) {
  throw new Error('Cargo.toml must define [workspace.package] version = "...".');
}
const workspaceVersion = versionMatch[1];

function updateVscodeManifest(relativePath, includeLockRoot) {
  const filePath = path.join(repoRoot, relativePath);
  const current = fs.readFileSync(filePath, "utf8");
  const topLevelVersion = /^  "version": "[^"]+",$/m;
  if (!topLevelVersion.test(current)) {
    throw new Error(`Could not locate the top-level version in ${relativePath}.`);
  }
  let generated = current.replace(
    topLevelVersion,
    `  "version": "${workspaceVersion}",`
  );
  if (includeLockRoot) {
    const lockedRootVersion = /^      "version": "[^"]+",$/m;
    if (!lockedRootVersion.test(current)) {
      throw new Error(`Could not locate packages[""].version in ${relativePath}.`);
    }
    generated = generated.replace(
      lockedRootVersion,
      `      "version": "${workspaceVersion}",`
    );
  }
  return [filePath, generated];
}

function replaceRequired(relativePath, pattern, replacement) {
  const filePath = path.join(repoRoot, relativePath);
  const current = fs.readFileSync(filePath, "utf8");
  if (!pattern.test(current)) {
    throw new Error(`Could not locate the version field in ${relativePath}.`);
  }
  const generated = current.replace(pattern, replacement);
  return [filePath, generated];
}

const outputs = [
  updateVscodeManifest("vscode/package.json", false),
  updateVscodeManifest("vscode/package-lock.json", true),
  replaceRequired(
    "zed/Cargo.toml",
    /(\[package\]\s*\r?\nname\s*=\s*"spec42-zed"\s*\r?\nversion\s*=\s*")[^"]+"/,
    `$1${workspaceVersion}"`
  ),
  replaceRequired(
    "zed/extension.toml",
    /(^version\s*=\s*")[^"]+"/m,
    `$1${workspaceVersion}"`
  ),
  replaceRequired(
    "zed/Cargo.lock",
    /(\[\[package\]\]\s*\r?\nname\s*=\s*"spec42-zed"\s*\r?\nversion\s*=\s*")[^"]+"/,
    `$1${workspaceVersion}"`
  ),
];

const stale = outputs.filter(([filePath, generated]) => {
  return fs.readFileSync(filePath, "utf8") !== generated;
});

if (check) {
  if (stale.length > 0) {
    const paths = stale.map(([filePath]) => path.relative(repoRoot, filePath)).join(", ");
    throw new Error(
      `Release versions differ from Cargo workspace ${workspaceVersion}: ${paths}. ` +
        "Run `node scripts/sync-workspace-version.mjs`."
    );
  }
} else {
  for (const [filePath, generated] of stale) {
    fs.writeFileSync(filePath, generated, "utf8");
  }
}
