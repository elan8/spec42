export interface PreparedNode {
  id: string;
  label: string;
  kind: string;
  sourcePath?: string | null;
  uri?: string | null;
  range?: { start?: { line?: number; character?: number }; end?: { line?: number; character?: number } } | null;
  attributes?: Record<string, unknown>;
}

export interface PreparedEdge {
  id: string;
  source: string;
  target: string;
  label: string;
  edgeKind?: string;
  attributes?: Record<string, unknown>;
}

export interface PreparedView {
  title: string;
  view: string;
  nodes: PreparedNode[];
  edges: PreparedEdge[];
  meta?: Record<string, unknown>;
}

export interface InterconnectionSceneViewDto {
  id: string;
  name: string;
  type: string;
  rootIds: string[];
}

export interface InterconnectionSceneNodeDto {
  id: string;
  semanticId: string;
  qualifiedName: string;
  name: string;
  kind: string;
  typeName?: string;
  parentId?: string;
}

export interface InterconnectionScenePortDto {
  id: string;
  semanticId: string;
  ownerNodeId: string;
  name: string;
  typeName?: string;
  direction?: string;
  sideHint: string;
}

export interface InterconnectionSceneEdgeDto {
  id: string;
  kind: string;
  sourcePortId: string;
  targetPortId: string;
  sourceNodeId: string;
  targetNodeId: string;
  semanticId?: string;
  label?: string;
}

export interface InterconnectionSceneContainerDto {
  id: string;
  label: string;
  parentId?: string;
  memberNodeIds: string[];
  depth: number;
}

export interface InterconnectionSceneDiagnosticDto {
  severity: string;
  code: string;
  message: string;
  connectorId?: string;
}

export interface InterconnectionSceneDto {
  schemaVersion: number;
  view: InterconnectionSceneViewDto;
  nodes: InterconnectionSceneNodeDto[];
  ports: InterconnectionScenePortDto[];
  edges: InterconnectionSceneEdgeDto[];
  containers: InterconnectionSceneContainerDto[];
  diagnostics: InterconnectionSceneDiagnosticDto[];
}

export interface InterconnectionLayoutPortAnchor {
  x: number;
  y: number;
  side: string;
}

export interface InterconnectionLayoutPortDrawOrder {
  west: string[];
  east: string[];
}

export interface InterconnectionLayoutNodeDto {
  id: string;
  x: number;
  y: number;
  width: number;
  height: number;
  portAnchors: Record<string, InterconnectionLayoutPortAnchor>;
  portDrawOrder?: InterconnectionLayoutPortDrawOrder;
}

export interface InterconnectionLayoutEdgeDto {
  id: string;
  routePoints: Array<{ x: number; y: number }>;
  sourcePortId?: string;
  targetPortId?: string;
}

export interface InterconnectionLayoutContainerDto {
  id: string;
  label: string;
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface InterconnectionLayoutDto {
  nodes: InterconnectionLayoutNodeDto[];
  edges: InterconnectionLayoutEdgeDto[];
  containers: InterconnectionLayoutContainerDto[];
  diagnostics: string[];
}

export interface InterconnectionPreparedPortDetail {
  id: string;
  name: string;
  direction?: string;
  portType?: string;
  portSide?: string;
  attributes: {
    parentId: string;
    scenePortId: string;
    sideHint: string;
  };
}

export interface InterconnectionPreparedNodeAttributes {
  containerId?: string | null;
  qualifiedName?: string;
  semanticId?: string;
  partType?: string;
  ports?: string[];
  portDetails?: InterconnectionPreparedPortDetail[];
  isDefinition?: boolean;
  isReference?: boolean;
  sceneNodeId?: string;
  isSyntheticContainer?: boolean;
  isPackageContainer?: boolean;
  memberNodeIds?: string[];
  layoutDepth?: number;
  _isLayoutContainer?: boolean;
  _layoutDepth?: number;
}

export interface InterconnectionPreparedNode extends Omit<PreparedNode, "attributes"> {
  attributes: InterconnectionPreparedNodeAttributes;
}

export interface InterconnectionPreparedEdgeAttributes {
  sourceId?: string;
  targetId?: string;
  sourcePortId?: string;
  targetPortId?: string;
  sourceNodeId?: string;
  targetNodeId?: string;
  semanticId?: string;
  relationType?: string;
  canonicalScene?: boolean;
}

export interface InterconnectionPreparedEdge extends Omit<PreparedEdge, "attributes"> {
  attributes: InterconnectionPreparedEdgeAttributes;
}

export interface InterconnectionPreparedMeta {
  canonicalScene: boolean;
  schemaVersion?: number;
  selectedRoot?: string | null;
  rootCandidates?: string[];
  diagnostics?: InterconnectionSceneDiagnosticDto[];
  packageContainerGroups?: unknown[];
}

export interface InterconnectionPreparedView extends Omit<PreparedView, "nodes" | "edges" | "meta"> {
  nodes: InterconnectionPreparedNode[];
  edges: InterconnectionPreparedEdge[];
  meta: InterconnectionPreparedMeta;
}

export function isInterconnectionPreparedView(
  prepared: PreparedView,
): prepared is InterconnectionPreparedView {
  return prepared.view === "interconnection-view" && Boolean(prepared.meta?.canonicalScene);
}

export function asInterconnectionPrepared(prepared: PreparedView): InterconnectionPreparedView {
  if (!isInterconnectionPreparedView(prepared)) {
    throw new Error("Expected canonical interconnection prepared view");
  }
  return prepared;
}

/** Layout/drawing path: canonical scene or legacy test fixtures with open attributes. */
export function interconnectionPreparedForLayout(prepared: PreparedView): InterconnectionPreparedView {
  return prepared as InterconnectionPreparedView;
}

export type UnknownRecord = Record<string, unknown>;
export type UnknownArray = UnknownRecord[];

export interface VisualizationPayload extends UnknownRecord {
  view?: string;
  selectedViewName?: string;
  selectedView?: string;
  interconnectionScene?: InterconnectionSceneDto;
  graph?: UnknownRecord;
  generalViewGraph?: UnknownRecord;
  ibd?: UnknownRecord;
  diagrams?: UnknownArray;
  activityDiagrams?: UnknownArray;
  sequenceDiagrams?: UnknownArray;
  stateMachines?: UnknownArray;
  stateDiagrams?: UnknownArray;
  synthesizeInitialState?: boolean;
  activityLayoutDirection?: string;
  stateLayoutDirection?: string;
}
