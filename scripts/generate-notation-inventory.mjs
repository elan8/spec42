#!/usr/bin/env node
/** Deterministic SysML graphical-BNF coverage inventory. */
import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const releaseDir = process.env.SYSML_V2_RELEASE_DIR;
if (!releaseDir) throw new Error("SYSML_V2_RELEASE_DIR must name an explicit pinned SysML-v2-Release checkout");
const configured = JSON.parse(fs.readFileSync(path.join(root, "config", "standard-library.json"), "utf8"));
const htmlPath = path.join(releaseDir, "bnf", "SysML-graphical-bnf.html");
if (!fs.existsSync(htmlPath)) throw new Error(`missing graphical BNF: ${htmlPath}`);
const html = fs.readFileSync(htmlPath, "utf8");

const implemented = new Set([
  "definition", "usage", "extended-def", "extended-usage", "part-def", "part", "part-ref",
  "port-def", "port-usage", "attribute-def", "attribute", "item-def", "item", "item-ref",
  "occurrence-def", "occurrence", "action-def", "action", "state-def", "state",
  "requirement-def", "requirement", "connection", "binding-connection", "flow", "interface",
  "interface-connection", "binary-dependency", "specializes", "typing", "hierarchy",
  "composition", "allocate", "satisfy", "verify", "bind", "dependency", "redefinition",
  "package-with-name-inside", "general-view", "interconnection-view",
]);
const partialPrefixes = ["sequence", "sq-", "control", "succession", "initial", "final", "compartment"];

const clauses = [];
for (const match of html.matchAll(/<h4><a id="([^"]+)"><\/a>\/\/ Clause ([^<]+)<\/h4>/g)) {
  clauses.push({ offset: match.index ?? 0, id: match[1], title: match[2].trim() });
}
const clauseAt = (offset) => [...clauses].reverse().find((clause) => clause.offset <= offset);
const strip = (value) => value.replace(/<[^>]+>/g, " ").replace(/&[^;]+;/g, " ").replace(/\s+/g, " ").trim();
const occurrences = [];
for (const match of html.matchAll(/<a id="([^"]+)"><\/a>\s*<pre>([\s\S]*?)<\/pre>/g)) {
  const id = match[1];
  const body = match[2];
  const images = [...body.matchAll(/images\/([^"']+\.svg)/g)].map((image) => image[1]);
  const clause = clauseAt(match.index ?? 0);
  const status = implemented.has(id)
    ? "supported"
    : partialPrefixes.some((prefix) => id.startsWith(prefix) || id.includes(`-${prefix}`))
      ? "partial"
      : "unsupported";
  occurrences.push({
    production: id,
    clause: clause?.id ?? "c8.2.3",
    clauseTitle: clause?.title ?? "Graphical Notation",
    images: [...new Set(images)].sort(),
    status,
    syntax: strip(body),
  });
}
const byProduction = new Map();
for (const occurrence of occurrences) {
  const existing = byProduction.get(occurrence.production);
  if (!existing) {
    byProduction.set(occurrence.production, { ...occurrence, occurrences: 1 });
  } else {
    existing.occurrences += 1;
    existing.images = [...new Set([...existing.images, ...occurrence.images])].sort();
    existing.syntax = `${existing.syntax} | ${occurrence.syntax}`;
  }
}
const productions = [...byProduction.values()];
productions.sort((left, right) => left.production.localeCompare(right.production));
if (productions.length < 100) throw new Error(`graphical BNF parse found only ${productions.length} productions`);

const coverage = {
  source: { repository: configured.repo, version: configured.version, document: "bnf/SysML-graphical-bnf.html" },
  productions,
};
const json = `${JSON.stringify(coverage, null, 2)}\n`;
const counts = Object.fromEntries(["supported", "partial", "unsupported"].map((status) => [status, productions.filter((item) => item.status === status).length]));
const markdown = [
  "# SysML graphical notation coverage (generated)", "",
  `Source: \`${configured.repo}\` release \`${configured.version}\`, \`bnf/SysML-graphical-bnf.html\`.`, "",
  `Productions: **${productions.length}**; supported: **${counts.supported}**; partial: **${counts.partial}**; unsupported: **${counts.unsupported}**.`, "",
  "| Production | Clause | SVG examples | Status |", "| --- | --- | --- | --- |",
  ...productions.map((item) => `| \`${item.production}\` | \`${item.clause}\` | ${item.images.map((image) => `\`${image}\``).join(", ") || "—"} | ${item.status} |`),
  "",
  "Regenerate with an explicit checkout of the configured release:", "",
  "```sh", "SYSML_V2_RELEASE_DIR=/path/to/SysML-v2-Release node scripts/generate-notation-inventory.mjs", "```", "",
].join("\n");
const outputs = [
  [path.join(root, "docs", "reference", "sysml-graphical-notation-coverage.json"), json],
  [path.join(root, "docs", "reference", "SYSML-NOTATION-INVENTORY.md"), markdown],
];
const check = process.argv.includes("--check");
let stale = false;
for (const [output, contents] of outputs) {
  if (check) {
    if (!fs.existsSync(output) || fs.readFileSync(output, "utf8") !== contents) stale = true;
  } else {
    fs.writeFileSync(output, contents, "utf8");
  }
}
if (stale) {
  console.error("SysML graphical notation coverage is stale");
  process.exit(1);
}
console.log(`${check ? "Checked" : "Wrote"} ${productions.length} graphical productions for SysML ${configured.version}`);
