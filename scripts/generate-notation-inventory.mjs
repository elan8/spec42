#!/usr/bin/env node
/**
 * Generates docs/SYSML-NOTATION-INVENTORY.md from SysML v2 BNF SVG filenames.
 * Set SYSML_V2_RELEASE_DIR to the SysML-v2-Release repo root (optional).
 */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(__dirname, "..");
const releaseDir = process.env.SYSML_V2_RELEASE_DIR || "";
const imagesDir = releaseDir
  ? path.join(releaseDir, "bnf", "images")
  : path.join(repoRoot, "third_party", "sysml-v2-release", "bnf", "images");

const SHIPPED_VIEWS = {
  "part-def": "general-view",
  "part": "general-view, interconnection-view",
  "part-ref": "general-view, interconnection-view",
  "port-def": "general-view",
  "port": "general-view, interconnection-view",
  "action-def": "general-view",
  "action": "general-view, action-flow-view",
  "state-def": "general-view",
  "state": "general-view, state-transition-view",
  "requirement-def": "general-view",
  "requirement": "general-view",
};

function inferStatus(name) {
  const base = name.replace(/\.svg$/i, "");
  if (base.includes("sequence") || base.startsWith("sq-")) return "legacy|shared (sequence-view)";
  if (base.includes("action-flow") || base.includes("control-flow")) return "shared (action-flow-view)";
  if (base.includes("state-transition") || base === "initial" || base === "final") return "shared (state-transition-view)";
  if (Object.keys(SHIPPED_VIEWS).some((key) => base.includes(key.replace(/-/g, "")) || base === key)) {
    return "shared";
  }
  return "WONTFIX (not in shipped UI)";
}

function inferViews(name) {
  const base = name.replace(/\.svg$/i, "");
  for (const [key, views] of Object.entries(SHIPPED_VIEWS)) {
    if (base === key || base.includes(key)) return views;
  }
  if (base.includes("interconnection") || base.includes("connection")) return "interconnection-view";
  if (base.includes("general")) return "general-view";
  return "—";
}

let files = [];
if (fs.existsSync(imagesDir)) {
  files = fs.readdirSync(imagesDir).filter((f) => f.endsWith(".svg")).sort();
} else {
  files = Object.keys(SHIPPED_VIEWS).map((k) => `${k}.svg`);
}

const lines = [
  "# SysML notation inventory (generated)",
  "",
  `Generated: ${new Date().toISOString().slice(0, 10)}`,
  "",
  `Source: \`${imagesDir}\` (${files.length} entries)`,
  "",
  "| SVG | Inferred views | Status | Code pointer |",
  "|-----|----------------|--------|--------------|",
];

for (const file of files) {
  const views = inferViews(file);
  const status = inferStatus(file);
  let pointer = "—";
  if (status.includes("shared")) {
    if (views.includes("interconnection")) pointer = "`shared/diagram-renderer/src/renderer.ts` (IBD)";
    else if (views.includes("action-flow")) pointer = "`shared/diagram-renderer/src/views/action-flow.ts`";
    else if (views.includes("state")) pointer = "`shared/diagram-renderer/src/views/state-transition.ts`";
    else if (views.includes("sequence")) pointer = "`shared/diagram-renderer/src/views/sequence.ts`";
    else pointer = "`shared/diagram-renderer/src/node-notation.ts`";
  }
  lines.push(`| ${file} | ${views} | ${status} | ${pointer} |`);
}

lines.push("");
lines.push("Regenerate: `node scripts/generate-notation-inventory.mjs`");
lines.push("");

const outPath = path.join(repoRoot, "docs", "SYSML-NOTATION-INVENTORY.md");
fs.writeFileSync(outPath, lines.join("\n"), "utf8");
console.log(`Wrote ${outPath} (${files.length} rows)`);
