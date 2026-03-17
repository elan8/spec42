export interface PositionDTO {
  line: number;
  character: number;
}

export interface RangeDTO {
  start: PositionDTO;
  end: PositionDTO;
}

export interface RelationshipDTO {
  type: string;
  source: string;
  target: string;
  name?: string;
}

export interface GraphNodeDTO {
  id: string;
  type: string;
  name: string;
  parentId?: string;
  range: RangeDTO;
  attributes: Record<string, unknown>;
}

export interface GraphEdgeDTO {
  source: string;
  target: string;
  /** Preferred; server may send rel_type instead of type */
  type?: string;
  rel_type?: string;
  name?: string;
}

export interface SysMLGraphDTO {
  nodes: GraphNodeDTO[];
  edges: GraphEdgeDTO[];
}

export interface SysMLElementDTO {
  type: string;
  name: string;
  range: RangeDTO;
  children: SysMLElementDTO[];
  attributes: Record<string, unknown>;
  relationships: RelationshipDTO[];
  errors?: string[];
}

export interface SysMLModelStatsDTO {
  totalElements: number;
  resolvedElements: number;
  unresolvedElements: number;
  parseTimeMs: number;
  modelBuildTimeMs: number;
  parseCached: boolean;
}

export interface SysMLModelParams {
  textDocument: { uri: string };
  scope?: Array<"graph" | "stats" | "sequenceDiagrams" | "activityDiagrams">;
}

export interface IbdPartDTO {
  id: string;
  name: string;
  qualifiedName: string;
  containerId?: string;
  type: string;
  attributes?: Record<string, unknown>;
}

export interface IbdPortDTO {
  id: string;
  name: string;
  parentId: string;
  direction?: string;
  portType?: string;
  portSide?: string;
}

export interface IbdConnectorDTO {
  source: string;
  target: string;
  sourceId: string;
  targetId: string;
  type: string;
}

export interface IbdDataDTO {
  parts: IbdPartDTO[];
  ports: IbdPortDTO[];
  connectors: IbdConnectorDTO[];
  rootCandidates: string[];
  defaultRoot?: string;
}

export interface DiagramBoundsDTO {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface HitRegionDTO {
  id: string;
  kind: string;
  elementId: string;
  qualifiedName?: string;
  bounds: DiagramBoundsDTO;
}

export interface DiagramMetricsDTO {
  nodeCount: number;
  edgeCount: number;
  overlapCount: number;
  overlapArea: number;
  edgeCrossingCount: number;
  edgeNodeIntrusionCount: number;
  totalEdgeLength: number;
  bendCount: number;
  orthogonalViolationCount: number;
  minimumNodeClearance: number;
  canvasArea: number;
  aspectRatio: number;
  compactness: number;
}

export interface DiagramViewStateDTO {
  view: string;
  selection?: string;
}

export interface RenderedDiagramDTO {
  svg: string;
  hitMap: HitRegionDTO[];
  bounds: DiagramBoundsDTO;
  metrics: DiagramMetricsDTO;
  warnings: string[];
  viewState: DiagramViewStateDTO;
}

export interface RenderedDiagramsDTO {
  generalView?: RenderedDiagramDTO;
  interconnectionView?: RenderedDiagramDTO;
}

export interface SysMLModelResult {
  version: number;
  graph?: SysMLGraphDTO;
  sequenceDiagrams?: SequenceDiagramDTO[];
  activityDiagrams?: ActivityDiagramDTO[];
  ibd?: IbdDataDTO;
  renderedDiagrams?: RenderedDiagramsDTO;
  stats?: SysMLModelStatsDTO;
}

// ---------------------------------------------------------------------------
// Sequence Diagrams (optional - our server returns empty if not implemented)
// ---------------------------------------------------------------------------

export interface SequenceDiagramDTO {
  name: string;
  participants: ParticipantDTO[];
  messages: MessageDTO[];
  range: RangeDTO;
}

export interface ParticipantDTO {
  name: string;
  type: string;
  range: RangeDTO;
}

export interface MessageDTO {
  name: string;
  from: string;
  to: string;
  payload: string;
  occurrence: number;
  range: RangeDTO;
}

// ---------------------------------------------------------------------------
// Activity Diagrams (optional - our server returns empty if not implemented)
// ---------------------------------------------------------------------------

export interface ActivityDiagramDTO {
  name: string;
  actions: ActivityActionDTO[];
  decisions: DecisionNodeDTO[];
  flows: ControlFlowDTO[];
  states: ActivityStateDTO[];
  range: RangeDTO;
}

export interface ActivityActionDTO {
  name: string;
  type: string;
  kind?: string;
  inputs?: string[];
  outputs?: string[];
  condition?: string;
  subActions?: ActivityActionDTO[];
  isDefinition?: boolean;
  range?: RangeDTO;
  parent?: string;
  children?: string[];
}

export interface DecisionNodeDTO {
  name: string;
  condition: string;
  branches: { condition: string; target: string }[];
  range: RangeDTO;
}

export interface ControlFlowDTO {
  from: string;
  to: string;
  condition?: string;
  guard?: string;
  range: RangeDTO;
}

export interface ActivityStateDTO {
  name: string;
  type: "initial" | "final" | "intermediate";
  entryActions?: string[];
  exitActions?: string[];
  doActivity?: string;
  range: RangeDTO;
}
