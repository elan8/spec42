import type { PreparedView, VisualizationPayload } from "../types";
import { diagramToPrepared, selectNamedDiagram } from "../diagram-select";
import { asArray, asRecord, asString } from "../util";
import { prepareGraph } from "../graph";

export function prepareSequence(visualization: VisualizationPayload): PreparedView {
  const selected = selectNamedDiagram(
    visualization?.sequenceDiagrams,
    visualization?.selectedViewName,
    visualization?.selectedView,
  );
  const fallbackDiagram = asArray(visualization?.sequenceDiagrams).map(asRecord)[0] ?? null;
  const effective = selected ?? fallbackDiagram;
  if (effective) {
    const prepared = diagramToPrepared(effective, "sequence-view", "Sequence View");
    return {
      ...prepared,
      meta: {
        selectedDiagramName: asString(asRecord(effective).name),
        sequenceDiagram: effective,
        parentContext: asString(asRecord(effective).name),
      },
    };
  }
  return prepareGraph(visualization?.graph, visualization);
}
