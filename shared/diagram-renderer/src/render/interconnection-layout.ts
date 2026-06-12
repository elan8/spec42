import type { InterconnectionLayoutDto, PreparedView } from "../prepare/types";
import { buildInterconnectionElkGraph } from "./layout";
import { buildInterconnectionElkGraphInput } from "./interconnection-elk-input";
import { layoutInterconnectionPrepared } from "./layout";
import type { LayoutResult } from "./types";

export { buildInterconnectionElkGraph, buildInterconnectionElkGraphInput };

export async function layoutInterconnectionScene(
  prepared: PreparedView,
): Promise<{ layout: LayoutResult; layoutDto: InterconnectionLayoutDto }> {
  const layout = await layoutInterconnectionPrepared(prepared);
  const layoutDto = layout.interconnectionLayout ?? {
    nodes: [],
    edges: [],
    containers: [],
    diagnostics: ["interconnectionLayout missing for canonical scene"],
  };
  return { layout, layoutDto };
}
