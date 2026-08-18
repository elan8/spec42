import { readFileSync, writeFileSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const scriptDir = dirname(fileURLToPath(import.meta.url));
const scenePath = process.argv[2] ?? join(scriptDir, "../test-fixtures/interconnection/grid-system-context-scene.json");

if (!existsSync(scenePath)) {
  console.error(`Scene file not found: ${scenePath}`);
  console.error("Generate it with: cargo test -p semantic_core export_powersystems_system_context_scene -- --nocapture");
  process.exit(1);
}

const { exportInterconnectionPipeline } = await import("../src/pipeline-export.ts");
const scene = JSON.parse(readFileSync(scenePath, "utf8"));
const payload = {
  view: "interconnection-view",
  selectedView: "systemContext",
  selectedViewName: "systemContext",
  interconnectionScene: scene,
};

const report = await exportInterconnectionPipeline(payload);
const outPath = join(scriptDir, "../test-fixtures/interconnection/grid-system-context-pipeline.json");
writeFileSync(outPath, JSON.stringify(report, null, 2));
console.log(JSON.stringify({
  sceneNodes: scene.nodes?.length ?? 0,
  scenePorts: scene.ports?.length ?? 0,
  sceneEdges: scene.edges?.length ?? 0,
  preparedNodes: report.preparedScene?.nodeCount ?? 0,
  preparedEdges: report.preparedScene?.edgeCount ?? 0,
  routePassed: report.routeSummary?.passed ?? false,
  violations: report.routeSummary?.violations ?? [],
}, null, 2));
