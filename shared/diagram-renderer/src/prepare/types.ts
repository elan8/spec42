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

export type UnknownRecord = Record<string, unknown>;
export type UnknownArray = UnknownRecord[];

export interface VisualizationPayload extends UnknownRecord {
  view?: string;
  selectedViewName?: string;
  selectedView?: string;
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
