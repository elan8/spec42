#!/usr/bin/env node
// Keeps docs/generation/ABI.md and generator-abi.json in step with the Rust contract.
//
// The manifest is the derived artifact; this script projects it into the specification's
// operation, level and token tables so those cannot drift from the code either.
//
//   node scripts/sync-generator-abi.mjs           # rewrite ABI.md
//   node scripts/sync-generator-abi.mjs --check   # fail if it would change
import { readFileSync, writeFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, join } from "node:path";

const root = join(dirname(fileURLToPath(import.meta.url)), "..");
const manifest = JSON.parse(readFileSync(join(root, "docs/generation/generator-abi.json"), "utf8"));
const specPath = join(root, "docs/generation/ABI.md");

const snake = (name) => name.replace(/(.)([A-Z])/g, "$1_$2").toLowerCase();

const operations = [
  "| Op | Name | Request | Response `T` |",
  "| --: | --- | --- | --- |",
  ...manifest.operations.map(
    (op) => `| ${op.code} | \`${snake(op.name)}\` | \`${op.request}\` | \`${op.response}\` |`,
  ),
].join("\n");

const levels = [
  "| Level | Code |",
  "| --- | --: |",
  ...manifest.diagnosticLevels.map((level) => `| \`${level.name.toLowerCase()}\` | ${level.code} |`),
].join("\n");

const header =
  `Current version: **ABI ${manifest.abiVersion}**. ` +
  `Compatibility token: \`${manifest.compatibilityToken}\`.`;

const replaceBlock = (text, marker, body) => {
  const open = `<!-- generated:${marker} -->`;
  const close = `<!-- /generated:${marker} -->`;
  const pattern = new RegExp(`${open}[\\s\\S]*?${close}`);
  if (!pattern.test(text)) {
    throw new Error(`ABI.md is missing the ${marker} block`);
  }
  return text.replace(pattern, `${open}\n${body}\n${close}`);
};

let spec = readFileSync(specPath, "utf8");
spec = replaceBlock(spec, "abi-header", header);
spec = replaceBlock(spec, "abi-operations", operations);
spec = replaceBlock(spec, "abi-levels", levels);

if (process.argv.includes("--check")) {
  if (spec !== readFileSync(specPath, "utf8")) {
    console.error(
      "docs/generation/ABI.md is stale; run: node scripts/sync-generator-abi.mjs",
    );
    process.exit(1);
  }
  console.log("ABI.md is current");
} else {
  writeFileSync(specPath, spec);
  console.log("ABI.md updated from generator-abi.json");
}
