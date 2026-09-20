import type { PreparedNode } from "../prepare/types";
import type { DiagramThemeOverrides } from "../theme";

/**
 * Expansion is renderer-owned presentation state. The server applies it while drawing; these
 * actions are how the mounted SVG controls hand a toggle back to the owner.
 */
export interface DisclosureActions {
  toggleNode: (nodeId: string) => void;
  /** `currentlyExpanded` is the drawn state, so the owner never re-derives the default. */
  toggleSection: (nodeId: string, sectionKey: string, currentlyExpanded: boolean) => void;
}

/** Serializable disclosure state for transferring the current General view to another renderer. */
export interface DisclosureState {
  expandedNodeIds: string[];
  sectionStates: Array<{ nodeId: string; sectionKey: string; expanded: boolean }>;
}

/** Identifies which diagram product and which of its views a draw request is for. The server
 * has no independent notion of this; it only echoes the value back, so the caller can tell a
 * response for a since-superseded product/view apart from a current one. */
export interface DiagramProductIdentity {
  modelDigest: string;
  viewHandle: string;
}

/** Requests a native SVG from wherever the host wires this to -- normally `spec42/draw` via the
 * VS Code extension host. Returns `null` when the host declines or fails to answer (offline,
 * cancelled, server error). There is no client drawing fallback for the five shipped views. */
export type RequestServerDraw = (
  identity: DiagramProductIdentity,
  presentationRevision: number,
  request: {
    product: unknown;
    width: number;
    height: number;
    colorScheme: "light" | "dark";
    disclosure: DisclosureState;
  },
  signal: AbortSignal,
) => Promise<string | null>;

export interface RenderOptions {
  onNodeClick?: (node: PreparedNode) => void;
  disclosure?: DisclosureActions;
  disclosureState?: DisclosureState;
  selectedNodeId?: string | null;
  theme?: DiagramThemeOverrides;
  delegateZoom?: boolean;
  onPerformance?: (event: string, data: Record<string, unknown>) => void;
  productIdentity?: DiagramProductIdentity;
  /** Schema-5 product JSON (or legacy visualization payload) for `requestDraw`. */
  product?: unknown;
  requestDraw?: RequestServerDraw;
}

export const NATIVE_DIAGRAM_VIEWS = new Set([
  "general-view",
  "interconnection-view",
  "sequence-view",
  "action-flow-view",
  "state-transition-view",
]);

export function isNativeDiagramView(view: string): boolean {
  return NATIVE_DIAGRAM_VIEWS.has(view);
}

export interface ContentBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface PreparedPort {
  id?: string;
  name: string;
  direction?: string;
  conjugated?: boolean;
  semanticId?: string;
  multiplicity?: string;
  portType?: string;
  portSide?: string;
  attributes?: Record<string, unknown>;
}

export interface ContentExtents {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export function contentBoundsFromExtents(extents: ContentExtents): ContentBounds {
  const width = extents.maxX - extents.minX;
  const height = extents.maxY - extents.minY;
  return {
    x: extents.minX,
    y: extents.minY,
    width: width > 0 ? width : 1,
    height: height > 0 ? height : 1,
  };
}
