import type { InterconnectionLayoutDto, PreparedNode } from "../prepare/types";
import type { DiagramThemeOverrides } from "../theme";
import { collectCompartments } from "../sysml-node-builder";

/**
 * Expansion is renderer-owned presentation state. The projection carries the resulting state on
 * each node (`disclosure`, `hiddenRelationshipCount`, `compartmentSectionState`); these actions are
 * how the drawn controls hand a toggle back to the owner.
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

/** Identifies which diagram product and which of its views a layout request is for. The server
 * has no independent notion of this; it only echoes the value back, so the caller can tell a
 * response for a since-superseded product/view apart from a current one. */
export interface DiagramProductIdentity {
  modelDigest: string;
  viewHandle: string;
}

/** Requests layout for `graph` (an ELK JSON graph, e.g. from `buildGeneralElkGraph`) from
 * wherever the host wires this to -- normally the Rust server's `spec42/layout` LSP request via
 * the VS Code extension host. `presentationRevision` and `signal` exist for the host's own
 * cancellation/staleness bookkeeping; this function does not interpret them.
 *
 * Returns `null` when the host declines or fails to answer (offline, cancelled, server error),
 * which the caller must treat as "compute layout locally instead", not as an empty layout. Not
 * every render host can provide this -- headless SVG export and the browser preview harness have
 * no server to call, so `RenderOptions.requestLayout` is optional and its absence just means
 * "always lay out in this process". */
export type RequestServerLayout = (
  graph: Record<string, unknown>,
  identity: DiagramProductIdentity,
  presentationRevision: number,
  signal: AbortSignal,
) => Promise<Record<string, unknown> | null>;

export interface RenderOptions {
  onNodeClick?: (node: PreparedNode) => void;
  disclosure?: DisclosureActions;
  disclosureState?: DisclosureState;
  selectedNodeId?: string | null;
  theme?: DiagramThemeOverrides;
  delegateZoom?: boolean;
  onPerformance?: (event: string, data: Record<string, unknown>) => void;
  /** Identity of the diagram product being rendered, required for `requestLayout` to be used. */
  productIdentity?: DiagramProductIdentity;
  requestLayout?: RequestServerLayout;
}

export const nodeWidth = 200;
export const nodeHeight = 70;
export const ibdNodeWidth = 280;
export const ibdNodeHeight = 140;

export interface EdgeSection {
  startPoint?: { x: number; y: number };
  bendPoints?: Array<{ x: number; y: number }>;
  endPoint?: { x: number; y: number };
}

export interface LaidOutNode extends PreparedNode {
  x?: number;
  y?: number;
  width?: number;
  height?: number;
  compartments?: ReturnType<typeof collectCompartments>;
}

export interface LaidOutEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  edgeKind?: string;
  attributes?: Record<string, unknown>;
  sourceNode?: LaidOutNode;
  targetNode?: LaidOutNode;
  layout?: {
    sections?: EdgeSection[];
    edgeOwnerOffset?: { x: number; y: number };
    lcaOffset?: { x: number; y: number };
  };
}

export interface LayoutResult {
  nodes: LaidOutNode[];
  edges: LaidOutEdge[];
  interconnectionLayout?: InterconnectionLayoutDto;
}

export interface ContentBounds {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface ContentExtents {
  minX: number;
  minY: number;
  maxX: number;
  maxY: number;
}

export interface PreparedPort {
  id?: string;
  name: string;
  direction?: string;
  /** Whether the port's authored typing conjugates the definition it names (`port p : ~PD;`). */
  conjugated?: boolean;
  semanticId?: string;
  multiplicity?: string;
  portType?: string;
  portSide?: string;
  attributes?: Record<string, unknown>;
}

export type PortUsage = { sourceCount: number; targetCount: number };

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

export function compareIbdPorts(
  node: PreparedNode,
  a: PreparedPort,
  b: PreparedPort,
  usageForPort: (node: PreparedNode, port: PreparedPort) => PortUsage,
): number {
  const usageA = usageForPort(node, a);
  const usageB = usageForPort(node, b);
  const degreeA = usageA.sourceCount + usageA.targetCount;
  const degreeB = usageB.sourceCount + usageB.targetCount;
  if (degreeB !== degreeA) return degreeB - degreeA;
  return a.name.localeCompare(b.name);
}

export function splitIbdPortsBySide(
  node: PreparedNode,
  ports: PreparedPort[],
  sideForPort: (port: PreparedPort, node: PreparedNode) => "WEST" | "EAST",
  usageForPort: (node: PreparedNode, port: PreparedPort) => PortUsage,
): { west: PreparedPort[]; east: PreparedPort[] } {
  const west: PreparedPort[] = [];
  const east: PreparedPort[] = [];
  for (const port of ports) {
    (sideForPort(port, node) === "WEST" ? west : east).push(port);
  }
  const compare = (a: PreparedPort, b: PreparedPort) => compareIbdPorts(node, a, b, usageForPort);
  west.sort(compare);
  east.sort(compare);
  return { west, east };
}

export function computeIbdLeafHeight(node: PreparedNode, ports: PreparedPort[], portRows: number): number {
  const attrs = (node.attributes ?? {}) as Record<string, unknown>;
  const headerHeight = attrs.partType ? 50 : 38;
  const children = Array.isArray(attrs.children) ? attrs.children : [];
  const contentLineCount = children.filter(
    (child) => child && typeof child === "object" && String((child as Record<string, unknown>).name || ""),
  ).length;
  const contentHeight = Math.min(contentLineCount, 8) * 12 + 10;
  const portSpacing = 26;
  const portsHeight = ports.length > 0 ? portRows * portSpacing + 22 : 0;
  return Math.min(340, Math.max(ibdNodeHeight, headerHeight + contentHeight + portsHeight));
}
