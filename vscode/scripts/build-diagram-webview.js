#!/usr/bin/env node
const esbuild = require("esbuild");
const path = require("path");

const vscodeDir = path.join(__dirname, "..");

async function build() {
  try {
    await esbuild.build({
      entryPoints: [path.join(vscodeDir, "src", "diagram", "diagramWebview.ts")],
      outfile: path.join(vscodeDir, "media", "diagram-viewer.js"),
      bundle: true,
      platform: "browser",
      format: "iife",
      target: "es2020",
      minify: true,
      logLevel: "info",
    });
    console.log("Diagram webview bundle written to media/diagram-viewer.js");
  } catch (error) {
    console.error("Diagram webview bundle failed:", error);
    process.exit(1);
  }
}

build();
