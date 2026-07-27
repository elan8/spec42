/**
 * Canonical fixture payloads for the CLI-vs-webview golden parity tests.
 * Both `renderer.golden-parity.test.ts` (real jsdom DOM, the webview-equivalent path) and
 * `headless-export.golden-parity.test.ts` (the virtual DOM used by `spec42 diagrams export`
 * and `POST /v1/diagrams/export`) render these exact payloads and must produce identical
 * structural markers -- see `svg-markers.ts` and `../../docs/engineering/DIAGRAM-EXPORT-QUALITY-ANALYSIS.md`.
 */
import type { UnknownRecord } from "../prepare/types";

const basePayload = {
  version: 1,
  workspaceRootUri: "file:///demo",
  modelReady: true,
  viewCandidates: [],
  selectedView: null,
  emptyStateMessage: null,
  packageGroups: null,
  workspaceModel: null,
  ibd: null,
  interconnectionScene: null,
  stats: null,
  projectionHints: null,
  graph: null,
  generalViewGraph: null,
  activityDiagrams: null,
  activityDiagramCandidates: null,
  sequenceDiagrams: null,
  sequenceDiagramCandidates: null,
  stateMachines: null,
  stateMachineCandidates: null,
};

export const generalViewGoldenPayload: UnknownRecord = {
  ...basePayload,
  view: "general-view",
  selectedViewName: "General",
  graph: {
    nodes: [
      { id: "P::Vehicle", name: "Vehicle", type: "part def", attributes: { attributes: ["mass"] } },
      { id: "P::vehicle", name: "vehicle", type: "part", attributes: { partType: "Vehicle" } },
    ],
    edges: [{ id: "typed", source: "P::vehicle", target: "P::Vehicle", type: "typing", name: "typing" }],
  },
};

export const interconnectionViewGoldenPayload: UnknownRecord = {
  ...basePayload,
  view: "interconnection-view",
  selectedViewName: "Connections",
  interconnectionScene: {
    schemaVersion: 2,
    view: { id: "v", name: "Connections", type: "InterconnectionView", rootIds: ["a", "b"] },
    nodes: [
      { id: "a", name: "a", kind: "part", qualifiedName: "a", semanticId: "a", definitionId: "A", typeName: "A" },
      { id: "b", name: "b", kind: "part", qualifiedName: "b", semanticId: "b", definitionId: "B", typeName: "B" },
    ],
    ports: [
      { id: "a.p", ownerNodeId: "a", name: "p", direction: "out", typeName: "Power", sideHint: "east" },
      { id: "b.p", ownerNodeId: "b", name: "p", direction: "in", typeName: "Power", sideHint: "west" },
    ],
    edges: [
      { id: "e", sourceNodeId: "a", targetNodeId: "b", sourcePortId: "a.p", targetPortId: "b.p", kind: "flow", label: "flow", semanticId: "e" },
    ],
    containers: [],
    diagnostics: [],
  },
};
