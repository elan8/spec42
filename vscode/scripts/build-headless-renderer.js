#!/usr/bin/env node
const esbuild = require("esbuild");
const fs = require("fs");
const path = require("path");

const vscodeDir = path.join(__dirname, "..");
const repoRoot = path.join(vscodeDir, "..");
const rendererDir = path.join(vscodeDir, "diagram-renderer");
const entryPoint = path.join(rendererDir, "src", "headless-export.ts");
const outFile = path.join(
  repoRoot,
  "crates",
  "server",
  "assets",
  "diagram-renderer",
  "headless-renderer.js",
);

async function build() {
  fs.mkdirSync(path.dirname(outFile), { recursive: true });
  await esbuild.build({
    entryPoints: [entryPoint],
    bundle: true,
    outfile: outFile,
    format: "iife",
    platform: "browser",
    target: "es2020",
    sourcemap: false,
    minify: false,
    globalName: "Spec42HeadlessRendererBundle",
    define: {
      "process.env.NODE_ENV": '"production"',
    },
    alias: {
      "elkjs/lib/elk.bundled.js": path.join(rendererDir, "src", "headless-elk-shim.ts"),
    },
  });
  console.log(`Headless renderer bundle written to ${outFile}`);
}

build().catch((error) => {
  console.error("Headless renderer build failed:", error);
  process.exitCode = 1;
});
