#!/usr/bin/env node
/**
 * Generates docs/reference/SYSML-NOTATION-INVENTORY.md from SysML v2 BNF SVG filenames.
 * Set SYSML_V2_RELEASE_DIR to the SysML-v2-Release repo root (optional).
 * Also writes docs/archive/GENERAL-IBD-BNF-SIGNOFF.md for shipped general + interconnection views.
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

/** Explicit code pointers for sign-off (general + interconnection focus). */
const CODE_POINTERS = {
  "part-def.svg": "`node-notation.ts` `resolveNodeChrome`",
  "part.svg": "`node-notation.ts`, `sysml-node-builder.ts` (general); `renderer.ts` `renderIbdNode` (IBD)",
  "part-ref.svg": "`node-notation.ts` `isReferenceKind`",
  "port-def.svg": "`sysml-node-builder.ts` compartments (general)",
  "port.svg": "`sysml-node-builder.ts` (general); `renderer.ts` `drawIbdPorts` (IBD)",
  "port-usage.svg": "`renderer.ts` `drawIbdPorts`",
  "connection.svg": "`renderer.ts` `applyEdgeMarker` (IBD)",
  "binding-connection.svg": "`renderer.ts` `applyEdgeMarker` bind branch",
  "flow.svg": "`renderer.ts` `applyEdgeMarker` flow branch",
  "interface.svg": "`renderer.ts` `applyEdgeMarker` interface branch",
  "interface-connection.svg": "`renderer.ts` `applyEdgeMarker` interface branch",
  "binary-dependency.svg": "`renderer.ts` `applyEdgeMarker` dependency",
  "definition.svg": "`node-notation.ts` definition chrome",
  "extended-usage.svg": "`node-notation.ts` usage chrome",
  "package-with-name-inside.svg": "`renderer.ts` `drawGeneralPackageContainers`",
  "package-with-name-in-tab.svg": "WONTFIX (tab variant; inside-name only)",
  "n-ary-connection-dot.svg": "WONTFIX (hub-and-spoke binary edges)",
  "n-ary-dependency-client-link.svg": "WONTFIX (hub-and-spoke binary edges)",
  "n-ary-dependency-supplier-link.svg": "WONTFIX (hub-and-spoke binary edges)",
};

const SHIPPED_ELEMENT_KEYS = {
  "part-def": "general-view",
  part: "general-view, interconnection-view",
  "part-ref": "general-view, interconnection-view",
  "port-def": "general-view",
  port: "general-view, interconnection-view",
  "port-usage": "interconnection-view",
  "port-l-1": "interconnection-view",
  "port-r-1": "interconnection-view",
  "action-def": "general-view",
  action: "general-view, action-flow-view",
  "state-def": "general-view",
  state: "general-view, state-transition-view",
  "requirement-def": "general-view",
  requirement: "general-view",
  connection: "interconnection-view",
  "binding-connection": "interconnection-view",
  flow: "interconnection-view",
  interface: "interconnection-view",
  "interface-connection": "interconnection-view",
  "binary-dependency": "general-view",
  definition: "general-view",
  "extended-usage": "general-view",
  specializes: "general-view",
  typing: "general-view",
  hierarchy: "general-view",
  composition: "general-view",
  allocate: "general-view",
  satisfy: "general-view",
  verify: "general-view",
  bind: "general-view",
  dependency: "general-view",
  usage: "general-view",
  redefinition: "general-view",
  "package-with-name-inside": "general-view",
};

const IBD_CONNECTOR_SVGS = new Set([
  "connection.svg",
  "binding-connection.svg",
  "flow.svg",
  "interface.svg",
  "interface-connection.svg",
  "flow-on-connection.svg",
]);

const GENERAL_RELATIONSHIP_SVGS = new Set([
  "binary-dependency.svg",
  "n-ary-dependency-client-link.svg",
  "n-ary-dependency-supplier-link.svg",
  "portion-relationship.svg",
  "redefinition.svg",
]);

function baseName(file) {
  return file.replace(/\.svg$/i, "");
}

function inferViews(name) {
  const base = baseName(name);
  if (CODE_POINTERS[name]) {
    const explicit = SHIPPED_ELEMENT_KEYS[base];
    if (explicit) return explicit;
  }
  for (const [key, views] of Object.entries(SHIPPED_ELEMENT_KEYS)) {
    if (base === key) return views;
  }
  if (IBD_CONNECTOR_SVGS.has(name) || base.includes("interconnection")) return "interconnection-view";
  if (GENERAL_RELATIONSHIP_SVGS.has(name) || base.includes("dependency")) return "general-view";
  if (base.startsWith("port-") && !base.includes("def")) return "interconnection-view";
  if (base.includes("compartment")) return "general-view, interconnection-view (compartment text)";
  if (base.includes("general")) return "general-view";
  return "—";
}

function inferStatus(name, views) {
  const base = baseName(name);
  if (name in CODE_POINTERS && CODE_POINTERS[name].startsWith("WONTFIX")) {
    return CODE_POINTERS[name];
  }
  if (base.includes("sequence") || base.startsWith("sq-")) return "legacy|shared (sequence-view)";
  if (base.includes("action-flow") || base === "aflow-succession" || base.includes("control-flow")) {
    return "shared (action-flow-view)";
  }
  if (base.includes("state-transition") || base === "initial" || base === "final") {
    return "shared (state-transition-view)";
  }
  if (views.includes("general-view") || views.includes("interconnection-view")) {
    if (views === "—") return "WONTFIX (not in shipped UI)";
    if (base.includes("compartment")) return "shared (compartment text only)";
    return "shared";
  }
  return "WONTFIX (not in shipped UI)";
}

function codePointer(name, views, status) {
  if (CODE_POINTERS[name]) return CODE_POINTERS[name];
  if (status.startsWith("WONTFIX")) return "—";
  if (!status.includes("shared")) return "—";
  if (views.includes("interconnection-view") && IBD_CONNECTOR_SVGS.has(name)) {
    return "`renderer.ts` `applyEdgeMarker` (IBD)";
  }
  if (views.includes("interconnection-view")) {
    return "`renderer.ts` (IBD)";
  }
  if (GENERAL_RELATIONSHIP_SVGS.has(name)) {
    return "`renderer.ts` `applyEdgeMarker` (general)";
  }
  if (views.includes("action-flow")) return "`views/action-flow.ts`";
  if (views.includes("state-transition")) return "`views/state-transition.ts`";
  if (views.includes("sequence")) return "`views/sequence.ts`";
  return "`node-notation.ts` / `sysml-node-builder.ts`";
}

let files = [];
if (fs.existsSync(imagesDir)) {
  files = fs.readdirSync(imagesDir).filter((f) => f.endsWith(".svg")).sort();
} else {
  files = Object.keys(SHIPPED_ELEMENT_KEYS).map((k) => `${k}.svg`);
}

const lines = [
  "# SysML notation inventory (generated)",
  "",
  `Generated: ${new Date().toISOString().slice(0, 10)}`,
  "",
  `Source: \`${imagesDir}\` (${files.length} entries)`,
  "",
  "Shipped product views: **general-view**, **interconnection-view** (+ behavior views). See [GENERAL-IBD-BNF-SIGNOFF.md](../archive/GENERAL-IBD-BNF-SIGNOFF.md) for sign-off checklist.",
  "",
  "| SVG | Inferred views | Status | Code pointer |",
  "|-----|----------------|--------|--------------|",
];

for (const file of files) {
  const views = inferViews(file);
  const status = inferStatus(file, views);
  const pointer = codePointer(file, views, status);
  lines.push(`| ${file} | ${views} | ${status} | ${pointer} |`);
}

lines.push("");
lines.push("Regenerate:");
lines.push("");
lines.push("```powershell");
lines.push("$env:SYSML_V2_RELEASE_DIR = 'C:\\path\\to\\SysML-v2-Release'");
lines.push("node scripts/generate-notation-inventory.mjs");
lines.push("```");
lines.push("");

const outPath = path.join(repoRoot, "docs", "reference", "SYSML-NOTATION-INVENTORY.md");
fs.writeFileSync(outPath, lines.join("\n"), "utf8");
console.log(`Wrote ${outPath} (${files.length} rows)`);

// Sign-off checklist for general + interconnection only
const signoffRows = files
  .map((file) => {
    const views = inferViews(file);
    if (!views.includes("general-view") && !views.includes("interconnection-view")) return null;
    const status = inferStatus(file, views);
    const pointer = codePointer(file, views, status);
    let testRef = "—";
    if (status.includes("shared") && !status.includes("compartment")) {
      if (views.includes("interconnection-view")) {
        testRef = "`shared/diagram-renderer/src/renderer.test.ts`";
      } else {
        testRef = "`shared/diagram-renderer/src/renderer.test.ts`, `crates/kernel/tests/integration/model.rs`";
      }
    }
    return { file, views, status, pointer, testRef };
  })
  .filter(Boolean);

const shippedImplemented = signoffRows.filter((r) => r.status === "shared").length;
const shippedTotal = signoffRows.filter((r) => !r.status.startsWith("WONTFIX (not in shipped")).length;
const coveragePct =
  shippedTotal > 0 ? Math.round((shippedImplemented / shippedTotal) * 100) : 0;

const signoffLines = [
  "# General and interconnection view — BNF sign-off checklist",
  "",
  `Generated: ${new Date().toISOString().slice(0, 10)}`,
  "",
  "Normative figures: `SysML-v2-Release/bnf/images/`. Implementation: [`shared/diagram-renderer`](../../shared/diagram-renderer).",
  "",
  `**Shipped-element coverage:** ${shippedImplemented} / ${shippedTotal} primary notations marked **shared** (${coveragePct}%). Compartment-only rows count as partial notation.`,
  "",
  "| BNF SVG | View(s) | Status | Implementation | Tests |",
  "|---------|---------|--------|----------------|-------|",
];

for (const row of signoffRows) {
  signoffLines.push(
    `| ${row.file} | ${row.views} | ${row.status} | ${row.pointer} | ${row.testRef} |`,
  );
}

signoffLines.push("");
signoffLines.push("## Automated regression");
signoffLines.push("");
signoffLines.push("- `cd shared/diagram-renderer && npm test` — edge markers, def/usage/ref chrome, IBD ports/connectors");
signoffLines.push("- `cargo test -p kernel --test integration model` — visualization payloads");
signoffLines.push("");
signoffLines.push("## Manual validation fixtures");
signoffLines.push("");
signoffLines.push("- General: `sysml/src/validation/.../01-Parts Tree/1d-Parts Tree with Reference.sysml`");
signoffLines.push("- Interconnection: KitchenTimer workspace fixture; connected-blocks / webshop models");
signoffLines.push("");

const signoffPath = path.join(repoRoot, "docs", "archive", "GENERAL-IBD-BNF-SIGNOFF.md");
fs.writeFileSync(signoffPath, signoffLines.join("\n"), "utf8");
console.log(`Wrote ${signoffPath} (${signoffRows.length} rows, ${coveragePct}% shared)`);
