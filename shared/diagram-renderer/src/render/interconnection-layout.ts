import type { InterconnectionLayoutDto, PreparedView } from "../prepare/types";
import { buildInterconnectionElkGraph, layoutInterconnectionPrepared } from "./layout";
import type { LayoutResult } from "./types";

export { buildInterconnectionElkGraph };

export async function layoutInterconnectionScene(
  prepared: PreparedView,
): Promise<{ layout: LayoutResult; layoutDto: InterconnectionLayoutDto }> {
  const layout = await layoutInterconnectionPrepared(prepared);
  const layoutDto = layout.interconnectionLayout ?? {
    nodes: [],
    edges: [],
    diagnostics: ["interconnectionLayout missing for canonical scene"],
  };
  return { layout, layoutDto };
}
