#!/usr/bin/env node
const esbuild = require("esbuild");
const path = require("path");

const rootDir = path.join(__dirname, "..");
const entryPoint = path.join(rootDir, "src", "extension.ts");
const outFile = path.join(rootDir, "out", "extension.js");

async function build() {
  try {
    const diagramRendererSrc = path.join(
      rootDir,
      "..",
      "shared",
      "diagram-renderer",
      "src",
    );
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
      alias: {
        "@spec42/diagram-renderer/pipeline-export": path.join(
          diagramRendererSrc,
          "pipeline-export.ts",
        ),
        "@spec42/diagram-renderer/prepare": path.join(
          diagramRendererSrc,
          "prepare.ts",
        ),
        // pipeline-export pulls layout.ts; resolve via vscode's elkjs (npm ci in vscode/).
        "elkjs/lib/elk.bundled.js": path.join(
          rootDir,
          "node_modules",
          "elkjs",
          "lib",
          "elk.bundled.js",
        ),
      },
      logLevel: "info",
    });
    console.log(`Extension bundle written to ${outFile}`);
  } catch (error) {
    console.error("Extension bundle build failed:", error);
    process.exit(1);
  }
}

build();
