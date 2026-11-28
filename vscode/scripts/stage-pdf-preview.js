#!/usr/bin/env node
const { copyFileSync, mkdirSync } = require("node:fs");
const { dirname, join } = require("node:path");

const extension = join(__dirname, "..");
const pdfjs = dirname(require.resolve("pdfjs-dist/package.json"));
const output = join(extension, "media", "pdfjs");
mkdirSync(output, { recursive: true });
for (const asset of ["pdf.min.mjs", "pdf.worker.min.mjs"]) {
  copyFileSync(join(pdfjs, "legacy", "build", asset), join(output, asset));
}
copyFileSync(join(pdfjs, "LICENSE"), join(output, "LICENSE"));
copyFileSync(join(extension, "media", "doc42-preview.mjs"), join(output, "preview.mjs"));
