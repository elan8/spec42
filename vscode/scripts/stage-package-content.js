#!/usr/bin/env node
const fs = require("fs");
const path = require("path");

const vscodeDir = path.join(__dirname, "..");
const repoRoot = path.join(vscodeDir, "..");

const contentDirs = ["examples", "domain-libraries"];

function removeIfExists(targetPath) {
  if (fs.existsSync(targetPath)) {
    fs.rmSync(targetPath, { recursive: true, force: true });
  }
}

function copyDirectory(fromPath, toPath) {
  if (!fs.existsSync(fromPath) || !fs.statSync(fromPath).isDirectory()) {
    throw new Error(`Source directory is missing: ${fromPath}`);
  }
  fs.cpSync(fromPath, toPath, { recursive: true });
}

function stageContent() {
  for (const dirName of contentDirs) {
    const source = path.join(repoRoot, dirName);
    const target = path.join(vscodeDir, dirName);
    removeIfExists(target);
    copyDirectory(source, target);
  }
}

function main() {
  stageContent();
  console.log("Staged extension package content: examples, domain-libraries");
}

main();
