#!/usr/bin/env node
/** Deterministic SysML/KerML textual-BNF coverage inventory. */
import fs from "node:fs";
import path from "node:path";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
const mapPath = path.join(root, "docs", "reference", "textual-syntax-map.json");
const jsonOut = path.join(root, "docs", "reference", "textual-syntax-coverage.json");
const mdOut = path.join(root, "docs", "reference", "TEXTUAL-SYNTAX-INVENTORY.md");
const check = process.argv.includes("--check");

const STATUSES = ["supported", "recovered", "unsupported", "untested"];

function extractProductions(filePath) {
  const text = fs.readFileSync(filePath, "utf8");
  const names = [];
  for (const line of text.split(/\r?\n/)) {
    if (!line || !/^[A-Za-z]/.test(line)) continue;
    const match = line.match(/^([A-Za-z0-9_]+)/);
    if (!match) continue;
    const rest = line.slice(match[0].length).trimStart();
    if (rest.startsWith("=") || rest.includes(" =")) names.push(match[1]);
  }
  return [...new Set(names)].sort();
}

function parserRev(cargoToml) {
  const match = cargoToml.match(
    /sysml-v2-parser\s*=\s*\{[^}]*rev\s*=\s*"([0-9a-f]+)"/s,
  );
  if (!match) throw new Error("could not read sysml-v2-parser rev from Cargo.toml");
  return match[1];
}

function fileHash(filePath) {
  return `sha256:${createHash("sha256").update(fs.readFileSync(filePath)).digest("hex")}`;
}

function requireFile(relPath, context) {
  const absolute = path.join(root, relPath);
  if (!fs.existsSync(absolute) || !fs.statSync(absolute).isFile()) {
    throw new Error(`${context}: missing example file ${relPath}`);
  }
}

const map = JSON.parse(fs.readFileSync(mapPath, "utf8"));
const pin = map.pin;
if (!pin || !Array.isArray(map.overlays)) {
  throw new Error("textual-syntax-map.json must include pin and overlays");
}

const sysmlBnf = path.join(root, pin.sysml_bnf);
const kermlBnf = path.join(root, pin.kerml_bnf);
const sysml = extractProductions(sysmlBnf);
const kerml = extractProductions(kermlBnf);
if (sysml.length !== pin.sysml_bnf_productions) {
  throw new Error(
    `SysML production count ${sysml.length} != pin ${pin.sysml_bnf_productions}`,
  );
}
if (kerml.length !== pin.kerml_bnf_productions) {
  throw new Error(
    `KerML production count ${kerml.length} != pin ${pin.kerml_bnf_productions}`,
  );
}

const cargoToml = fs.readFileSync(path.join(root, "Cargo.toml"), "utf8");
const parserRevision = parserRev(cargoToml);

const byKey = new Map();
for (const overlay of map.overlays) {
  if (!["SysML", "KerML"].includes(overlay.grammar)) {
    throw new Error(`overlay grammar must be SysML or KerML: ${overlay.grammar}`);
  }
  if (!STATUSES.includes(overlay.status) || overlay.status === "untested") {
    throw new Error(
      `${overlay.grammar}.${overlay.production}: overlays must use supported, recovered, or unsupported`,
    );
  }
  const names = overlay.grammar === "SysML" ? sysml : kerml;
  if (!names.includes(overlay.production)) {
    throw new Error(
      `overlay production ${overlay.grammar}.${overlay.production} is not in the pinned BNF`,
    );
  }
  if (!Array.isArray(overlay.forms) || overlay.forms.length === 0) {
    throw new Error(
      `${overlay.grammar}.${overlay.production}: verified productions require at least one example form`,
    );
  }
  const kinds = new Set(overlay.forms.map((form) => form.kind));
  if (overlay.status === "supported" && !kinds.has("accept")) {
    throw new Error(
      `${overlay.grammar}.${overlay.production}: supported productions require an accept example`,
    );
  }
  if (
    (overlay.status === "recovered" || overlay.status === "unsupported") &&
    !kinds.has("reject")
  ) {
    throw new Error(
      `${overlay.grammar}.${overlay.production}: ${overlay.status} productions require a reject example`,
    );
  }
  for (const form of overlay.forms) {
    if (!["accept", "reject"].includes(form.kind) || typeof form.path !== "string") {
      throw new Error(
        `${overlay.grammar}.${overlay.production}: form must have kind accept|reject and a path`,
      );
    }
    requireFile(form.path, `${overlay.grammar}.${overlay.production}`);
  }
  const key = `${overlay.grammar}:${overlay.production}`;
  if (byKey.has(key)) throw new Error(`duplicate overlay ${key}`);
  byKey.set(key, overlay);
}

function classify(grammar, production) {
  const overlay = byKey.get(`${grammar}:${production}`);
  return overlay
    ? {
        grammar,
        production,
        status: overlay.status,
        notes: overlay.notes ?? "",
        issue: overlay.issue ?? null,
        forms: overlay.forms,
      }
    : {
        grammar,
        production,
        status: "untested",
        notes: "",
        issue: null,
        forms: [],
      };
}

const productions = [
  ...sysml.map((production) => classify("SysML", production)),
  ...kerml.map((production) => classify("KerML", production)),
];

const counts = Object.fromEntries(
  STATUSES.map((status) => [status, productions.filter((item) => item.status === status).length]),
);

const coverage = {
  source: {
    repository: pin.release_repo,
    version: pin.release_tag,
    parserRevision,
    sysmlBnf: pin.sysml_bnf,
    kermlBnf: pin.kerml_bnf,
    sysmlBnfSha256: fileHash(sysmlBnf),
    kermlBnfSha256: fileHash(kermlBnf),
  },
  counts: {
    productions: productions.length,
    sysml: sysml.length,
    kerml: kerml.length,
    ...counts,
  },
  productions,
};

const json = `${JSON.stringify(coverage, null, 2)}\n`;
const formCell = (item) => {
  if (item.forms.length === 0) return "—";
  return item.forms
    .map((form) => `\`${form.kind}\` \`${form.path}\``)
    .join("<br>");
};
const markdown = [
  "# SysML/KerML textual syntax coverage (generated)",
  "",
  `Source: \`${pin.release_repo}\` release \`${pin.release_tag}\`, parser \`${parserRevision}\`.`,
  "",
  `Productions: **${productions.length}** (SysML ${sysml.length}, KerML ${kerml.length}); supported: **${counts.supported}**; recovered: **${counts.recovered}**; unsupported: **${counts.unsupported}**; untested: **${counts.untested}**.`,
  "",
  "Status is Spec42's verified compiler/LSP coverage of the pinned textual BNF, not the parser's L1 classification map. Untested means the production is known in the pin but has no executable accept/reject example in this inventory yet.",
  "",
  "| Grammar | Production | Status | Examples | Notes |",
  "| --- | --- | --- | --- | --- |",
  ...productions.map(
    (item) =>
      `| ${item.grammar} | \`${item.production}\` | ${item.status} | ${formCell(item)} | ${item.notes.replaceAll("|", "\\|")}${item.issue ? ` (${item.issue})` : ""} |`,
  ),
  "",
  "Regenerate with:",
  "",
  "```sh",
  "node scripts/generate-textual-syntax-inventory.mjs",
  "```",
  "",
].join("\n");

const outputs = [
  [jsonOut, json],
  [mdOut, markdown],
];
let stale = false;
for (const [output, contents] of outputs) {
  if (check) {
    if (!fs.existsSync(output) || fs.readFileSync(output, "utf8") !== contents) stale = true;
  } else {
    fs.writeFileSync(output, contents, "utf8");
  }
}
if (stale) {
  console.error("SysML/KerML textual syntax coverage is stale");
  process.exit(1);
}
console.log(
  `${check ? "Checked" : "Wrote"} ${productions.length} textual productions (${counts.supported} supported, ${counts.untested} untested) for parser ${parserRevision}`,
);
