#!/usr/bin/env node
/**
 * Sync Spec42 generated reference pages from product truth.
 *
 * Sources:
 *   - vscode/package.json (Spec42 version)
 *   - config/standard-library.json
 *   - config/libraries/*.json
 *   - bundled KPAR artifacts in .cache/ (downloaded from GitHub releases when missing)
 *
 * Writes docs/reference/WHATS-INCLUDED.md, docs/reference/DOMAIN-LIBRARIES.md, and
 * docs/reference/METHOD-LIBRARIES.md so they stay aligned with the extension release.
 *
 * Usage:
 *   node scripts/sync-docs-meta.mjs [--check]
 */
import fs from "node:fs";
import path from "node:path";
import process from "node:process";
import { fileURLToPath } from "node:url";
import { readZipEntries } from "./lib/simple-zip.mjs";

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(scriptDir, "..");
const cacheDir = path.join(repoRoot, ".cache");
const materializeRoot = path.join(cacheDir, "docs-kpar-materialized");
const check = process.argv.includes("--check");

const whatsIncludedPath = path.join(repoRoot, "docs", "reference", "WHATS-INCLUDED.md");
const domainLibrariesPath = path.join(repoRoot, "docs", "reference", "DOMAIN-LIBRARIES.md");
const methodLibrariesPath = path.join(repoRoot, "docs", "reference", "METHOD-LIBRARIES.md");

function readJson(filePath) {
  return JSON.parse(fs.readFileSync(filePath, "utf8"));
}

function writeOrCheck(filePath, content) {
  const next = content.endsWith("\n") ? content : `${content}\n`;
  const relative = path.relative(repoRoot, filePath);
  if (check) {
    const current = fs.existsSync(filePath) ? fs.readFileSync(filePath, "utf8") : "";
    if (current !== next) {
      throw new Error(`${relative} is out of date. Run \`node scripts/sync-docs-meta.mjs\`.`);
    }
    return false;
  }
  const prev = fs.existsSync(filePath) ? fs.readFileSync(filePath, "utf8") : "";
  if (prev === next) return false;
  fs.mkdirSync(path.dirname(filePath), { recursive: true });
  fs.writeFileSync(filePath, next, "utf8");
  return true;
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
      const id = typeof config.id === "string" && config.id.trim() ? config.id : stem;
      return {
        id,
        displayName: config.displayName,
        version: config.version,
        repo: config.repo,
        format: config.format,
        artifact:
          config.artifact ||
          `elan8-${id}-libraries-${config.version}.kpar`,
      };
    });
}

function isUsableKpar(filePath) {
  if (!fs.existsSync(filePath)) return false;
  try {
    const entries = readZipEntries(fs.readFileSync(filePath));
    return entries.has(".project.json");
  } catch {
    return false;
  }
}

async function downloadFile(url, destPath) {
  const response = await fetch(url);
  if (!response.ok) {
    throw new Error(`HTTP ${response.status} for ${url}`);
  }
  const buffer = Buffer.from(await response.arrayBuffer());
  fs.mkdirSync(path.dirname(destPath), { recursive: true });
  fs.writeFileSync(destPath, buffer);
}

async function ensureKparArtifact(library) {
  const overrideVariable = `SPEC42_DOCS_${library.id.toUpperCase()}_KPAR`;
  const overridePath = process.env[overrideVariable];
  if (overridePath) {
    const resolvedOverride = path.resolve(overridePath);
    if (!isUsableKpar(resolvedOverride)) {
      throw new Error(
        `${overrideVariable} does not point to a usable KPAR: ${resolvedOverride}`
      );
    }
    console.log(`Using local KPAR override ${resolvedOverride}`);
    return resolvedOverride;
  }

  const out = path.join(cacheDir, library.artifact);
  if (isUsableKpar(out)) {
    console.log(`Using existing KPAR ${out}`);
    return out;
  }
  const url = `https://github.com/${library.repo}/releases/download/v${library.version}/${library.artifact}`;
  console.log(`Fetching KPAR ${url}`);
  await downloadFile(url, out);
  if (!isUsableKpar(out)) {
    fs.rmSync(out, { force: true });
    throw new Error(`Downloaded file is not a usable KPAR: ${url}`);
  }
  console.log(`Fetched ${out}`);
  return out;
}

function materializeKpar(kparPath, destinationRoot) {
  if (fs.existsSync(destinationRoot)) {
    fs.rmSync(destinationRoot, { recursive: true, force: true });
  }
  fs.mkdirSync(destinationRoot, { recursive: true });

  const rawEntries = readZipEntries(fs.readFileSync(kparPath));
  const projectData = rawEntries.get(".project.json");
  const metaData = rawEntries.get(".meta.json");
  if (!projectData || !metaData) {
    throw new Error(`KPAR missing manifests: ${kparPath}`);
  }
  const project = JSON.parse(projectData.toString("utf8"));
  const meta = JSON.parse(metaData.toString("utf8"));
  const entries = new Map(
    [...rawEntries.entries()].map(([entryName, data]) => [entryName.replace(/\\/g, "/"), data])
  );

  const pairs = [];
  const index = meta.index && typeof meta.index === "object" ? meta.index : {};
  if (Object.keys(index).length === 0) {
    for (const name of entries.keys()) {
      if (name.endsWith(".sysml") || name.endsWith(".kerml")) {
        pairs.push([name, name]);
      }
    }
  } else {
    for (const [logicalPath, archivePath] of Object.entries(index)) {
      const logical = String(logicalPath).replace(/\\/g, "/");
      const archive = String(archivePath).replace(/\\/g, "/");
      if (logical.endsWith(".sysml") || logical.endsWith(".kerml")) {
        pairs.push([logical, archive]);
      } else if (archive.endsWith(".sysml") || archive.endsWith(".kerml")) {
        pairs.push([archive, archive]);
      }
    }
  }

  const sourceFiles = [];
  for (const [logicalPath, archivePath] of pairs) {
    const bytes = entries.get(archivePath);
    if (!bytes) continue;
    const dest = path.join(destinationRoot, logicalPath);
    fs.mkdirSync(path.dirname(dest), { recursive: true });
    fs.writeFileSync(dest, bytes);
    sourceFiles.push(logicalPath);
  }
  sourceFiles.sort();
  return { project, meta, sourceFiles, root: destinationRoot };
}

function extractPackageNames(sourceText) {
  const mask = (value) => value.replace(/[^\r\n]/g, " ");
  const sanitized = sourceText
    .replace(/\/\*[\s\S]*?\*\//g, mask)
    .replace(/\/\/[^\r\n]*/g, mask)
    .replace(/"(?:\\.|[^"\\])*"/g, mask);

  const paths = [];
  const packageStack = [];
  let braceDepth = 0;
  const tokenPattern =
    /\b(?:private\s+|public\s+|protected\s+)*(?:standard\s+|library\s+)*package\s+([A-Za-z_][\w]*)\s*\{|[{}]/g;

  let match;
  while ((match = tokenPattern.exec(sanitized)) !== null) {
    if (match[1]) {
      braceDepth += 1;
      packageStack.push({ name: match[1], depth: braceDepth });
      paths.push(packageStack.map((entry) => entry.name).join("::"));
      continue;
    }

    if (match[0] === "{") {
      braceDepth += 1;
      continue;
    }

    while (
      packageStack.length > 0 &&
      packageStack[packageStack.length - 1].depth === braceDepth
    ) {
      packageStack.pop();
    }
    braceDepth = Math.max(0, braceDepth - 1);
  }

  const uniquePaths = [...new Set(paths)];
  return uniquePaths.filter(
    (candidate) =>
      !uniquePaths.some(
        (other) => other !== candidate && other.startsWith(`${candidate}::`)
      )
  );
}

function buildSourceTree(sourceFiles, rootDir) {
  /** @type {Map<string, any>} */
  const root = new Map();

  function ensureDir(map, name) {
    if (!map.has(name)) {
      map.set(name, { type: "dir", name, children: new Map() });
    }
    return map.get(name);
  }

  for (const relativePath of sourceFiles) {
    const parts = relativePath.split("/");
    let current = root;
    for (let i = 0; i < parts.length - 1; i++) {
      const node = ensureDir(current, parts[i]);
      current = node.children;
    }
    const fileName = parts[parts.length - 1];
    const abs = path.join(rootDir, relativePath);
    const text = fs.readFileSync(abs, "utf8");
    const packages = extractPackageNames(text);
    current.set(fileName, {
      type: "file",
      name: fileName,
      path: relativePath,
      packages,
    });
  }

  function toObject(map) {
    return [...map.values()]
      .sort((a, b) => {
        if (a.type !== b.type) return a.type === "dir" ? -1 : 1;
        return a.name.localeCompare(b.name);
      })
      .map((node) => {
        if (node.type === "dir") {
          return {
            type: "dir",
            name: node.name,
            children: toObject(node.children),
          };
        }
        return node;
      });
  }

  return toObject(root);
}

function renderTreeMarkdown(nodes, indent = 0) {
  const pad = "  ".repeat(indent);
  const lines = [];
  for (const node of nodes) {
    if (node.type === "dir") {
      lines.push(`${pad}- **${node.name}/**`);
      lines.push(...renderTreeMarkdown(node.children, indent + 1));
    } else {
      const pkgLabel =
        node.packages.length > 0
          ? ` — ${node.packages.map((name) => `\`${name}\``).join(", ")}`
          : "";
      lines.push(`${pad}- \`${node.path}\`${pkgLabel}`);
    }
  }
  return lines;
}

function countFiles(nodes) {
  let count = 0;
  for (const node of nodes) {
    if (node.type === "file") count += 1;
    else count += countFiles(node.children);
  }
  return count;
}

function flattenPackages(nodes, acc = []) {
  for (const node of nodes) {
    if (node.type === "file") {
      for (const packageName of node.packages) {
        acc.push({ packageName, file: node.path });
      }
    } else {
      flattenPackages(node.children, acc);
    }
  }
  return acc;
}

function renderDomainPage(meta, tree, project) {
  const domain = meta.libraries.find((library) => library.id === "domain");
  const fileCount = countFiles(tree);
  const packages = flattenPackages(tree);
  const lines = [
    "<!-- GENERATED by scripts/sync-docs-meta.mjs — do not edit. -->",
    "",
    "# Domain Libraries",
    "",
    "Elan8 domain libraries provide reusable SysML v2 **vocabulary** for things in the system.",
    "This overview is generated from the bundled KPAR artifact so it matches what Spec42 ships.",
    "",
    `| | |`,
    `| --- | --- |`,
    `| Bundled version | \`${domain?.version ?? project?.version ?? "unknown"}\` |`,
    `| Format | \`kpar\` |`,
    `| Artifact | \`${domain?.artifact ?? ""}\` |`,
    `| Packages / files | ${packages.length} packages · ${fileCount} source files |`,
    `| Source | [${domain?.repo ?? "elan8/sysml-domain-libraries"}](https://github.com/${domain?.repo ?? "elan8/sysml-domain-libraries"}) |`,
    "",
    "Method / process packages live in the [Method libraries](./METHOD-LIBRARIES.md).",
    "",
    "## Package tree",
    "",
    "Tree of source files inside the KPAR. Package names are scanned from each `.sysml` file.",
    "",
    ...renderTreeMarkdown(tree),
    "",
    "## Package index",
    "",
    "| Package | Source file |",
    "| --- | --- |",
  ];
  for (const row of packages.sort((a, b) => a.packageName.localeCompare(b.packageName))) {
    lines.push(`| \`${row.packageName}\` | \`${row.file}\` |`);
  }
  lines.push(
    "",
    "## In Spec42",
    "",
    "- Open the **Library** view to search and browse these symbols.",
    "- See [Library & Dependencies](../user/LIBRARIES.md) for custom paths and Sysand.",
    "- See [What's included](./WHATS-INCLUDED.md) for release pins.",
    ""
  );
  return lines.join("\n");
}

function renderMethodPage(meta, tree, project) {
  const method = meta.libraries.find((library) => library.id === "method");
  const packages = flattenPackages(tree);
  const fileCount = countFiles(tree);
  const lines = [
    "<!-- GENERATED by scripts/sync-docs-meta.mjs — do not edit. -->",
    "",
    "# Method Libraries",
    "",
    "Elan8 Method libraries provide SysML v2 packages for requirements metadata, concerns, viewpoints, and related method concepts.",
    "This overview is generated from the bundled KPAR artifact so it matches what Spec42 ships.",
    "",
    `| | |`,
    `| --- | --- |`,
    `| Bundled version | \`${method?.version ?? project?.version ?? "unknown"}\` |`,
    `| Format | \`kpar\` |`,
    `| Artifact | \`${method?.artifact ?? ""}\` |`,
    `| Packages / files | ${packages.length} packages · ${fileCount} source files |`,
    `| Source | [${method?.repo ?? "elan8/mbse-methodology"}](https://github.com/${method?.repo ?? "elan8/mbse-methodology"}) |`,
    "",
    "## Package tree",
    "",
    ...renderTreeMarkdown(tree),
    "",
    "## Package index",
    "",
    "| Package | Source file |",
    "| --- | --- |",
  ];
  for (const row of packages.sort((a, b) => a.packageName.localeCompare(b.packageName))) {
    lines.push(`| \`${row.packageName}\` | \`${row.file}\` |`);
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
    "- Domain vocabulary is documented separately in [Domain libraries](./DOMAIN-LIBRARIES.md).",
    "- Exact release pins are listed in [What's included](./WHATS-INCLUDED.md).",
    ""
  );
  return lines.join("\n");
}

function renderWhatsIncluded(meta) {
  const lines = [
    "<!-- GENERATED by scripts/sync-docs-meta.mjs — do not edit. -->",
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
    "- [Domain libraries](./DOMAIN-LIBRARIES.md) — KPAR package/file tree",
    "- [Method libraries](./METHOD-LIBRARIES.md) — Elan8 Method packages",
    "- [Library & Dependencies](../user/LIBRARIES.md) — Library view, custom paths, and Sysand",
    ""
  );
  return lines.join("\n");
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

const domainLib = libraries.find((library) => library.id === "domain");
const methodLib = libraries.find((library) => library.id === "method");
if (!domainLib || !methodLib) {
  throw new Error("Expected domain and method entries in config/libraries.");
}

const domainKpar = await ensureKparArtifact(domainLib);
const methodKpar = await ensureKparArtifact(methodLib);

const domainMaterialized = materializeKpar(
  domainKpar,
  path.join(materializeRoot, "domain")
);
const methodMaterialized = materializeKpar(
  methodKpar,
  path.join(materializeRoot, "method")
);

const domainTree = buildSourceTree(domainMaterialized.sourceFiles, domainMaterialized.root);
const methodTree = buildSourceTree(methodMaterialized.sourceFiles, methodMaterialized.root);

console.log(
  `Domain KPAR tree: ${domainMaterialized.sourceFiles.length} files, ${flattenPackages(domainTree).length} packages`
);
console.log(
  `Method KPAR tree: ${methodMaterialized.sourceFiles.length} files, ${flattenPackages(methodTree).length} packages`
);

const changed = [];
if (writeOrCheck(whatsIncludedPath, renderWhatsIncluded(meta))) {
  changed.push(path.relative(repoRoot, whatsIncludedPath));
}
if (writeOrCheck(domainLibrariesPath, renderDomainPage(meta, domainTree, domainMaterialized.project))) {
  changed.push(path.relative(repoRoot, domainLibrariesPath));
}
if (writeOrCheck(methodLibrariesPath, renderMethodPage(meta, methodTree, methodMaterialized.project))) {
  changed.push(path.relative(repoRoot, methodLibrariesPath));
}

if (changed.length === 0) {
  console.log(check ? "Generated reference pages are up to date." : "Docs meta already up to date.");
} else {
  console.log("Updated:");
  for (const file of changed) console.log(`  ${file}`);
}
