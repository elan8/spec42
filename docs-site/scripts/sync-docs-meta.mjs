#!/usr/bin/env node
/**
 * Sync Spec42 docs metadata and library overview pages from product truth.
 *
 * Sources:
 *   - vscode/package.json (Spec42 version)
 *   - config/standard-library.json
 *   - config/libraries/*.json
 *   - sibling sysml-domain-libraries / mbse-methodology when present (catalog refresh)
 *
 * Usage (from repo root or docs-site):
 *   node docs-site/scripts/sync-docs-meta.mjs
 *   node scripts/sync-docs-meta.mjs
 */
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const docsSiteRoot = path.resolve(scriptDir, "..");
const repoRoot = path.resolve(docsSiteRoot, "..");

const generatedMetaPath = path.join(docsSiteRoot, "docs", ".vitepress", "generated-meta.json");
const whatsIncludedPath = path.join(docsSiteRoot, "docs", "reference", "whats-included.md");
const domainLibrariesPath = path.join(docsSiteRoot, "docs", "reference", "domain-libraries.md");
const methodLibrariesPath = path.join(docsSiteRoot, "docs", "reference", "method-libraries.md");

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function writeIfChanged(filePath, content) {
  const next = content.endsWith("\n") ? content : `${content}\n`;
  if (fs.existsSync(filePath)) {
    const prev = fs.readFileSync(filePath, "utf8");
    if (prev === next) {
      return false;
    }
  }
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, next, "utf8");
  return true;
}

function findSibling(repoName) {
  const candidates = [
    path.join(repoRoot, "..", repoName),
    path.join(repoRoot, repoName),
  ];
  return candidates.find((candidate) => fs.existsSync(candidate));
}

function loadKparLibraries() {
  const librariesDir = path.join(repoRoot, "config", "libraries");
  return fs
    .readdirSync(librariesDir)
    .filter((name) => name.toLowerCase().endsWith(".json"))
    .sort()
    .map((fileName) => {
      const config = readJson(path.join(librariesDir, fileName));
      const stem = path.basename(fileName, ".json");
      return {
        id: typeof config.id === "string" && config.id.trim() ? config.id : stem,
        displayName: config.displayName,
        version: config.version,
        repo: config.repo,
        format: config.format,
        artifact: config.artifact ?? "",
      };
    });
}

function firstParagraph(markdown) {
  const lines = markdown.replace(/\r\n/g, "\n").split("\n");
  const body = [];
  let seenTitle = false;
  for (const line of lines) {
    const trimmed = line.trim();
    if (!seenTitle) {
      if (trimmed.startsWith("#")) {
        seenTitle = true;
      }
      continue;
    }
    if (!trimmed) {
      if (body.length > 0) break;
      continue;
    }
    if (trimmed.startsWith("#") || trimmed.startsWith("---") || trimmed.startsWith(">")) {
      if (body.length > 0) break;
      continue;
    }
    if (trimmed.startsWith("- ") || trimmed.startsWith("* ") || trimmed.startsWith("|")) {
      if (body.length > 0) break;
      continue;
    }
    body.push(trimmed.replace(/\*\*/g, ""));
    if (body.join(" ").length > 280) break;
  }
  return body.join(" ").trim();
}

function listSysmlRelative(rootDir, max = 40) {
  const results = [];
  function walk(dir) {
    if (results.length >= max) return;
    let entries = [];
    try {
      entries = fs.readdirSync(dir, { withFileTypes: true });
    } catch {
      return;
    }
    for (const entry of entries) {
      if (results.length >= max) return;
      if (entry.name.startsWith(".")) continue;
      const full = path.join(dir, entry.name);
      if (entry.isDirectory()) {
        if (entry.name === "examples" || entry.name === "node_modules") continue;
        walk(full);
      } else if (entry.isFile() && entry.name.endsWith(".sysml")) {
        results.push(path.relative(rootDir, full).replace(/\\/g, "/"));
      }
    }
  }
  walk(rootDir);
  return results;
}

function collectAreaCatalog(areaRoot, areaName) {
  if (!fs.existsSync(areaRoot)) return [];
  const families = fs
    .readdirSync(areaRoot, { withFileTypes: true })
    .filter((entry) => entry.isDirectory() && !entry.name.startsWith("."))
    .map((entry) => entry.name)
    .sort();

  return families.map((family) => {
    const familyRoot = path.join(areaRoot, family);
    const readmePath = path.join(familyRoot, "README.md");
    const readme = fs.existsSync(readmePath) ? fs.readFileSync(readmePath, "utf8") : "";
    const titleMatch = readme.match(/^#\s+(.+)$/m);
    const title = titleMatch ? titleMatch[1].trim() : family;
    const summary = firstParagraph(readme) || `${areaName} library family \`${family}\`.`;
    const packages = listSysmlRelative(familyRoot, 24);
    return { family, title, summary, packages };
  });
}

function renderDomainPage(meta, catalog) {
  const domain = meta.libraries.find((library) => library.id === "domain");
  const lines = [
    "<!-- GENERATED by docs-site/scripts/sync-docs-meta.mjs — do not edit. -->",
    "",
    "# Domain Libraries",
    "",
    "Elan8 domain libraries provide reusable SysML v2 **vocabulary** for things in the system.",
    "They are bundled with Spec42 and available automatically in the Library view.",
    "",
    `| | |`,
    `| --- | --- |`,
    `| Bundled version | \`${domain?.version ?? "unknown"}\` |`,
    `| Format | \`${domain?.format ?? "kpar"}\` |`,
    `| Source | [${domain?.repo ?? "elan8/sysml-domain-libraries"}](https://github.com/${domain?.repo ?? "elan8/sysml-domain-libraries"}) |`,
    "",
    "Use these libraries for domain and technical modeling content. Method / process packages live in the [Method libraries](./method-libraries).",
    "",
  ];

  if (catalog?.areas?.length) {
    lines.push("## Library families", "");
    for (const area of catalog.areas) {
      lines.push(`### ${area.name}`, "");
      if (area.blurb) {
        lines.push(area.blurb, "");
      }
      for (const family of area.families) {
        lines.push(`#### ${family.title}`, "");
        lines.push(family.summary, "");
        if (family.packages.length) {
          lines.push("Key packages:", "");
          for (const pkg of family.packages.slice(0, 12)) {
            lines.push(`- \`${pkg}\``);
          }
          if (family.packages.length > 12) {
            lines.push(`- …and ${family.packages.length - 12} more`);
          }
          lines.push("");
        }
      }
    }
  } else {
    lines.push(
      "## Structure",
      "",
      "The bundled domain libraries are organized as:",
      "",
      "- `domain/` — business-domain vocabulary (for example robotics)",
      "- `technical/` — business-agnostic technical capabilities (software, electronics, communication)",
      "- `generic/` — cross-domain foundation units",
      "",
      "Refresh this page's detailed catalog by running `npm run docs:sync` from a checkout that also contains `sysml-domain-libraries`.",
      ""
    );
  }

  lines.push(
    "## In Spec42",
    "",
    "- Open the **Library** view in the Spec42 sidebar to search and browse symbols.",
    "- See [Library & Dependencies](/guide/libraries) for custom paths and Sysand.",
    "- See [What's included](./whats-included) for the exact bundled versions in this Spec42 release.",
    ""
  );
  return lines.join("\n");
}

function parseMethodPackageTable(readme) {
  const rows = [];
  for (const line of readme.replace(/\r\n/g, "\n").split("\n")) {
    if (!line.startsWith("|")) continue;
    const cells = line
      .split("|")
      .slice(1, -1)
      .map((cell) => cell.trim());
    if (cells.length < 3) continue;
    if (/^-{3,}/.test(cells[0]) || cells[0].toLowerCase() === "package") continue;
    rows.push({
      packageName: cells[0].replace(/`/g, ""),
      file: cells[1].replace(/`/g, ""),
      purpose: cells[2],
    });
  }
  return rows;
}

function renderMethodPage(meta, catalog) {
  const method = meta.libraries.find((library) => library.id === "method");
  const lines = [
    "<!-- GENERATED by docs-site/scripts/sync-docs-meta.mjs — do not edit. -->",
    "",
    "# Method Libraries",
    "",
    "Elan8 Method libraries provide SysML v2 packages for requirements metadata, concerns, viewpoints, and related method concepts.",
    "They are bundled with Spec42 separately from domain vocabulary.",
    "",
    `| | |`,
    `| --- | --- |`,
    `| Bundled version | \`${method?.version ?? "unknown"}\` |`,
    `| Format | \`${method?.format ?? "kpar"}\` |`,
    `| Source | [${method?.repo ?? "elan8/mbse-methodology"}](https://github.com/${method?.repo ?? "elan8/mbse-methodology"}) |`,
    "",
  ];

  const packages = catalog?.packages?.length
    ? catalog.packages
    : [
        {
          packageName: "Elan8RequirementManagement",
          file: "Elan8RequirementManagement.sysml",
          purpose: "Evidence, baselines, traceability concerns",
        },
        {
          packageName: "Elan8RequirementMetadata",
          file: "Elan8RequirementMetadata.sysml",
          purpose: "Requirement role and identity annotations",
        },
        {
          packageName: "Elan8Method",
          file: "Elan8Method.sysml",
          purpose: "Concerns, abstraction levels, decisions, project info",
        },
        {
          packageName: "Elan8Viewpoints",
          file: "Elan8Viewpoints.sysml",
          purpose: "Standard viewpoints and view stubs",
        },
      ];

  lines.push("## Bundled packages", "", "| Package | File | Purpose |", "| --- | --- | --- |");
  for (const row of packages) {
    lines.push(`| \`${row.packageName}\` | \`${row.file}\` | ${row.purpose} |`);
  }
  lines.push(
    "",
    "## Methodology docs",
    "",
    "These packages are the library surface used by Spec42. For the full method (principles, recipes, workflow), see the",
    `[Elan8 Method repository](https://github.com/${method?.repo ?? "elan8/mbse-methodology"}).`,
    "",
    "## In Spec42",
    "",
    "- Browse symbols from the **Library** view.",
    "- Domain vocabulary is documented separately in [Domain libraries](./domain-libraries).",
    "- Exact release pins are listed in [What's included](./whats-included).",
    ""
  );
  return lines.join("\n");
}

function renderWhatsIncluded(meta) {
  const lines = [
    "<!-- GENERATED by docs-site/scripts/sync-docs-meta.mjs — do not edit. -->",
    "",
    "# What's Included",
    "",
    "This page is generated from Spec42 product configuration so the docs stay aligned with the extension release.",
    "",
    "## Spec42",
    "",
    `| | |`,
    `| --- | --- |`,
    `| Extension version | \`${meta.spec42Version}\` |`,
    `| Marketplace | [SysML v2 Editor (Spec42)](https://marketplace.visualstudio.com/items?itemName=Elan8.spec42) |`,
    `| Source | [elan8/spec42](https://github.com/elan8/spec42) |`,
    "",
    "## Bundled libraries",
    "",
    "| Library | Version | Format | Source |",
    "| --- | --- | --- | --- |",
    `| SysML v2 standard library | \`${meta.standardLibrary.version}\` | \`${meta.standardLibrary.format}\` | [${meta.standardLibrary.repo}](https://github.com/${meta.standardLibrary.repo}) |`,
  ];

  for (const library of meta.libraries) {
    lines.push(
      `| ${library.displayName} | \`${library.version}\` | \`${library.format}\` | [${library.repo}](https://github.com/${library.repo}) |`
    );
  }

  lines.push(
    "",
    "## Learn more",
    "",
    "- [Domain libraries](./domain-libraries) — vocabulary overview",
    "- [Method libraries](./method-libraries) — Elan8 Method packages",
    "- [Library & Dependencies](/guide/libraries) — using the Library view, custom paths, and Sysand",
    ""
  );
  return lines.join("\n");
}

function buildDomainCatalog(domainRoot) {
  return {
    areas: [
      {
        name: "Domain",
        blurb: "Business-domain vocabulary for modeling things in a specific industry or product domain.",
        families: collectAreaCatalog(path.join(domainRoot, "domain"), "Domain"),
      },
      {
        name: "Technical",
        blurb: "Business-agnostic technical capabilities such as software, electronics, and communication.",
        families: collectAreaCatalog(path.join(domainRoot, "technical"), "Technical"),
      },
      {
        name: "Generic",
        blurb: "Cross-domain foundation content such as units.",
        families: collectAreaCatalog(path.join(domainRoot, "generic"), "Generic"),
      },
    ].filter((area) => area.families.length > 0),
  };
}

function buildMethodCatalog(methodRoot) {
  const libraryReadme = path.join(methodRoot, "library", "README.md");
  if (fs.existsSync(libraryReadme)) {
    const packages = parseMethodPackageTable(fs.readFileSync(libraryReadme, "utf8"));
    if (packages.length) {
      return { packages };
    }
  }
  const libraryDir = path.join(methodRoot, "library");
  if (!fs.existsSync(libraryDir)) return { packages: [] };
  const packages = fs
    .readdirSync(libraryDir)
    .filter((name) => name.endsWith(".sysml"))
    .sort()
    .map((file) => ({
      packageName: path.basename(file, ".sysml"),
      file,
      purpose: "See source package documentation.",
    }));
  return { packages };
}

const packageJson = readJson(path.join(repoRoot, "vscode", "package.json"));
const standardLibrary = readJson(path.join(repoRoot, "config", "standard-library.json"));
const libraries = loadKparLibraries();

const meta = {
  generatedAt: new Date().toISOString(),
  spec42Version: packageJson.version,
  standardLibrary: {
    version: standardLibrary.version,
    repo: standardLibrary.repo,
    format: standardLibrary.format,
  },
  libraries,
};

const domainSibling = findSibling("sysml-domain-libraries");
const methodSibling = findSibling("mbse-methodology");

let domainCatalog = null;
let methodCatalog = null;
if (domainSibling) {
  domainCatalog = buildDomainCatalog(domainSibling);
  console.log(`Refreshed domain catalog from ${domainSibling}`);
} else {
  console.log("Sibling sysml-domain-libraries not found; writing domain page without local catalog scan.");
}
if (methodSibling) {
  methodCatalog = buildMethodCatalog(methodSibling);
  console.log(`Refreshed method catalog from ${methodSibling}`);
} else {
  console.log("Sibling mbse-methodology not found; using fallback method package table.");
}

const changed = [];
if (writeIfChanged(generatedMetaPath, `${JSON.stringify(meta, null, 2)}\n`)) {
  changed.push(path.relative(repoRoot, generatedMetaPath));
}
if (writeIfChanged(whatsIncludedPath, renderWhatsIncluded(meta))) {
  changed.push(path.relative(repoRoot, whatsIncludedPath));
}
if (writeIfChanged(domainLibrariesPath, renderDomainPage(meta, domainCatalog))) {
  changed.push(path.relative(repoRoot, domainLibrariesPath));
}
if (writeIfChanged(methodLibrariesPath, renderMethodPage(meta, methodCatalog))) {
  changed.push(path.relative(repoRoot, methodLibrariesPath));
}

if (changed.length === 0) {
  console.log("Docs meta already up to date.");
} else {
  console.log("Updated:");
  for (const file of changed) {
    console.log(`  ${file}`);
  }
}
