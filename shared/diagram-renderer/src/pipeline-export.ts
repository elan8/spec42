import { prepareViewData } from "./prepare";
import { buildInterconnectionElkGraphInput, layoutInterconnectionScene } from "./render/interconnection-layout";
import { summarizeRoutes } from "./render/route-quality";
import type { UnknownRecord } from "./prepare/types";

export interface InterconnectionPipelineExport {
  rawPayload: UnknownRecord;
  preparedScene: UnknownRecord;
  elkInput: UnknownRecord;
  elkOutput: UnknownRecord | null;
  routeSummary: Record<string, unknown>;
}

export async function exportInterconnectionPipeline(
  payload: UnknownRecord,
): Promise<InterconnectionPipelineExport> {
  const prepared = prepareViewData({ ...payload, view: "interconnection-view" });
  const elkInput = buildInterconnectionElkGraphInput(prepared);
  let elkOutput: UnknownRecord | null = null;
  let routeSummary: Record<string, unknown> = { passed: false, violations: ["layout not run"] };
  try {
    const { layout, layoutDto } = await layoutInterconnectionScene(prepared);
    elkOutput = {
      nodes: layoutDto.nodes,
      edges: layoutDto.edges,
      diagnostics: layoutDto.diagnostics,
    };
    routeSummary = summarizeRoutes(layout.edges, layout.nodes);
  } catch (error) {
    routeSummary = {
      passed: false,
      violations: [error instanceof Error ? error.message : "layout failed"],
    };
  }
  return {
    rawPayload: payload,
    preparedScene: {
      title: prepared.title,
      view: prepared.view,
      nodeCount: prepared.nodes.length,
      edgeCount: prepared.edges.length,
      meta: prepared.meta ?? {},
      nodes: prepared.nodes,
      edges: prepared.edges,
    },
    elkInput,
    elkOutput,
    routeSummary,
  };
}
