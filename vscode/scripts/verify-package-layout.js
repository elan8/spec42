#!/usr/bin/env node

const fs = require("fs");
const path = require("path");
const yauzl = require("yauzl");

function usage() {
  console.error("Usage: node scripts/verify-package-layout.js --vsix <path>");
  process.exit(2);
}

const argumentIndex = process.argv.indexOf("--vsix");
if (argumentIndex < 0 || !process.argv[argumentIndex + 1]) {
  usage();
}

const vsixPath = path.resolve(process.argv[argumentIndex + 1]);
if (!fs.existsSync(vsixPath)) {
  throw new Error(`VSIX does not exist: ${vsixPath}`);
}

const requiredEntries = new Set([
  "extension/generators/diagram.wasm",
  "extension/package.json",
]);

yauzl.open(vsixPath, { lazyEntries: true }, (openError, archive) => {
  if (openError) {
    throw openError;
  }

  archive.on("entry", (entry) => {
    requiredEntries.delete(entry.fileName);
    archive.readEntry();
  });
  archive.on("end", () => {
    if (requiredEntries.size > 0) {
      throw new Error(
        `VSIX is missing required entries: ${[...requiredEntries].sort().join(", ")}`,
      );
    }
    console.log(`Verified generated diagram Wasm in ${path.basename(vsixPath)}`);
  });
  archive.readEntry();
});
