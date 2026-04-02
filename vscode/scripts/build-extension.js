#!/usr/bin/env node
const esbuild = require("esbuild");
const path = require("path");

const rootDir = path.join(__dirname, "..");
const entryPoint = path.join(rootDir, "src", "extension.ts");
const outFile = path.join(rootDir, "out", "extension.js");

async function build() {
  try {
    await esbuild.build({
      entryPoints: [entryPoint],
      outfile: outFile,
      bundle: true,
      platform: "node",
      format: "cjs",
      target: "node20",
      sourcemap: false,
      minify: true,
      external: ["vscode"],
      logLevel: "info",
    });
    console.log(`Extension bundle written to ${outFile}`);
  } catch (error) {
    console.error("Extension bundle build failed:", error);
    process.exit(1);
  }
}

build();
