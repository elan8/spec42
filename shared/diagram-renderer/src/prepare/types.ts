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

export interface InterconnectionLayoutDto {
  nodes: InterconnectionLayoutNodeDto[];
  edges: InterconnectionLayoutEdgeDto[];
  diagnostics: string[];
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
