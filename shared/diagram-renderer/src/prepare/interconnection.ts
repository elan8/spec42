import { prepareInterconnectionLegacy } from "./interconnection-legacy";
import { prepareInterconnectionScene } from "./interconnection-scene";
import type { PreparedView, VisualizationPayload } from "./types";

export { prepareInterconnectionLegacy } from "./interconnection-legacy";
export { prepareInterconnectionScene } from "./interconnection-scene";

export function prepareInterconnection(visualization: VisualizationPayload): PreparedView {
  const scene = visualization.interconnectionScene;
  if (scene && scene.schemaVersion >= 1) {
    return prepareInterconnectionScene(scene, visualization);
  }
  return prepareInterconnectionLegacy(visualization);
}
