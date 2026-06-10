import fs from "node:fs";

const srcPath = new URL("../src/renderer.ts", import.meta.url);
const lines = fs.readFileSync(srcPath, "utf8").split(/\r?\n/);

const typesContent = `import type { ReturnType } from "./types-internal";
import type { PreparedNode } from "../prepare";
import { collectCompartments } from "../sysml-node-builder";

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
`;

const layoutHeader = `import ELK from "elkjs/lib/elk.bundled.js";
import { normalizeEdgeKind } from "../graph-normalization";
import { collectCompartments, computeNodeHeight } from "../sysml-node-builder";
import type { PreparedNode, PreparedView } from "../prepare";
import {
  computeIbdLeafHeight,
  ibdNodeHeight,
  ibdNodeWidth,
  nodeHeight,
  nodeWidth,
  type LaidOutEdge,
  type LaidOutNode,
  type LayoutResult,
  type PreparedPort,
} from "./types";

const elk = new ELK();

`;

const exportHeader = `import * as d3 from "d3";
import type { DiagramTheme } from "../theme";
import type { ContentBounds, LayoutResult } from "./types";
import { nodeHeight, nodeWidth } from "./types";

`;

const exportBody = lines.slice(1552, 1607).join("\n");

const layoutBody = lines
  .slice(288, 687)
  .join("\n")
  .replace(/^async function layoutPrepared/gm, "export async function layoutPrepared")
  .replace(/^async function layoutInterconnectionPrepared/gm, "export async function layoutInterconnectionPrepared");

const drawingHeader = `import * as d3 from "d3";
import { normalizeEdgeKind } from "../graph-normalization";
import { resolveNodeChrome } from "../node-notation";
import { collectCompartments, computeNodeHeight, renderSysMLNode } from "../sysml-node-builder";
import { strokeColorForEdge, strokeColorForNode, type DiagramTheme } from "../theme";
import type { PreparedNode, PreparedView } from "../prepare";
import type { RenderOptions } from "../renderer";
import {
  ibdNodeWidth,
  nodeWidth,
  type ContentBounds,
  type LaidOutEdge,
  type LaidOutNode,
  type LayoutResult,
  type PreparedPort,
} from "./types";

`;

const drawingBody = lines
  .slice(688, 1551)
  .join("\n");

const rendererHeader = `import * as d3 from "d3";
import { resolveDiagramTheme } from "./theme";
import type { PreparedView } from "./prepare";
import { addActionFlowMarkers, renderActionFlowView } from "./views/action-flow";
import { renderSequenceView, addSequenceMarkers } from "./views/sequence";
import { addStateTransitionMarkers, renderStateTransitionView } from "./views/state-transition";
import { renderBrowserView, renderGeometryView, renderGridView } from "./views/standard-views";
import {
  addMarkers,
  applyFit,
  contentBounds,
  exportSvg,
} from "./render/export";
import {
  drawEdges,
  drawGeneralPackageContainers,
  drawIbdViewFrame,
  drawInterconnectionContainers,
  drawNodes,
  shouldDrawIbdViewFrame,
} from "./render/drawing";
import { layoutPrepared } from "./render/layout";
import { contentBoundsFromExtents } from "./render/types";

export interface RenderOptions {
  onNodeClick?: (node: PreparedNode) => void;
  selectedNodeId?: string | null;
  theme?: import("./theme").DiagramThemeOverrides;
  delegateZoom?: boolean;
}

export interface RenderController {
  reset: () => void;
  exportSvg: () => string;
  destroy: () => void;
  getFitTransform: () => d3.ZoomTransform;
}

import type { PreparedNode } from "./prepare";

`;

const rendererBody = lines.slice(156, 287).join("\n");

const outDir = new URL("../src/render/", import.meta.url);
fs.mkdirSync(outDir, { recursive: true });
fs.writeFileSync(new URL("types.ts", outDir), typesContent.replace('import type { ReturnType } from "./types-internal";\n', ""));
fs.writeFileSync(new URL("layout.ts", outDir), layoutHeader + layoutBody);
fs.writeFileSync(new URL("export.ts", outDir), exportHeader + exportBody.replace(/^function /gm, "export function "));
fs.writeFileSync(new URL("drawing.ts", outDir), drawingHeader + drawingBody);

const newRenderer = rendererHeader + rendererBody;
fs.writeFileSync(srcPath, newRenderer);
console.log("Split complete");
