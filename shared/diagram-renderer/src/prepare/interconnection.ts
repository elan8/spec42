import { prepareInterconnectionScene } from "./interconnection-scene";
import type { PreparedView, VisualizationPayload } from "./types";

export { prepareInterconnectionScene } from "./interconnection-scene";

export function prepareInterconnection(visualization: VisualizationPayload): PreparedView {
  const scene = visualization.interconnectionScene;
  if (scene && scene.schemaVersion >= 1) {
    return prepareInterconnectionScene(scene, visualization);
  }
  return {
    title: String(visualization.selectedViewName || "Interconnection View"),
    view: "interconnection-view",
    nodes: [],
    edges: [],
    meta: {
      diagnostics: [
        {
          severity: "error",
          code: "missing_interconnection_scene",
          message: "Interconnection view requires interconnectionScene from the language server.",
        },
      ],
    },
  };
}
