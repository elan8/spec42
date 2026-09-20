import type { Selection } from "d3";
import type { PreparedView } from "../prepare";
import type { RenderOptions } from "../render/types";
import type { DiagramTheme } from "../theme";

export interface BehaviorSceneContext {
  root: Selection<SVGGElement, unknown, null, undefined>;
  prepared: PreparedView;
  theme: DiagramTheme;
  width: number;
  height: number;
  options?: RenderOptions;
}

export function truncateLabel(text: string, max: number): string {
  const trimmed = text.trim();
  return trimmed.length > max ? `${trimmed.slice(0, max - 2)}..` : trimmed;
}
