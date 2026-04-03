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

export interface SysMLFeatureInspectorParams {
  textDocument: { uri: string };
  position: PositionDTO;
}

export interface SysMLFeatureInspectorElementRefDTO {
  id: string;
  name: string;
  qualifiedName: string;
  type: string;
  uri: string;
  range: RangeDTO;
}

export interface SysMLFeatureInspectorResolutionDTO {
  status: "resolved" | "unresolved" | "notApplicable";
  targets: SysMLFeatureInspectorElementRefDTO[];
}

export interface SysMLFeatureInspectorRelationshipDTO {
  type: string;
  peer: SysMLFeatureInspectorElementRefDTO;
  name?: string;
}

export interface SysMLFeatureInspectorElementDTO {
  id: string;
  name: string;
  qualifiedName: string;
  type: string;
  uri: string;
  range: RangeDTO;
  parent?: SysMLFeatureInspectorElementRefDTO;
  attributes: Record<string, unknown>;
  typing: SysMLFeatureInspectorResolutionDTO;
  specialization: SysMLFeatureInspectorResolutionDTO;
  incomingRelationships: SysMLFeatureInspectorRelationshipDTO[];
  outgoingRelationships: SysMLFeatureInspectorRelationshipDTO[];
}

export interface SysMLFeatureInspectorResult {
  version: number;
  sourceUri: string;
  requestedPosition: PositionDTO;
  element?: SysMLFeatureInspectorElementDTO | null;
}

export interface SysMLModelParams {
  textDocument: { uri: string };
  scope?: Array<"graph" | "ibd" | "stats" | "activityDiagrams" | "workspaceVisualization">;
}

export interface SysMLDiagramOptions {
  workspaceVisualization?: boolean;
  root?: string;
}

export interface SysMLDiagramParams {
  textDocument: { uri: string };
  kind: "general-view" | "interconnection-view";
  options?: SysMLDiagramOptions;
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

export interface SysMLModelResult {
  version: number;
  graph?: SysMLGraphDTO;
  generalViewGraph?: SysMLGraphDTO;
  activityDiagrams?: ActivityDiagramDTO[];
  ibd?: IbdDataDTO;
  stats?: SysMLModelStatsDTO;
}

export interface DiagramPointDTO {
  x: number;
  y: number;
}

export interface DiagramBoundsDTO {
  x: number;
  y: number;
  width: number;
  height: number;
}

export interface DiagramNodeCompartmentsDTO {
  stereotype: string;
  name: string;
  typedByName?: string;
  attributes: string[];
  parts: string[];
  ports: string[];
  other: string[];
}

export interface GeneralDiagramNodeDTO {
  id: string;
  name: string;
  qualifiedName: string;
  type: string;
  x: number;
  y: number;
  width: number;
  height: number;
  category: string;
  isDefinition: boolean;
  compartments: DiagramNodeCompartmentsDTO;
}

export interface GeneralDiagramEdgeDTO {
  id: string;
  source: string;
  target: string;
  type: string;
  label?: string;
  points: DiagramPointDTO[];
}

export interface GeneralDiagramSceneDTO {
  nodes: GeneralDiagramNodeDTO[];
  edges: GeneralDiagramEdgeDTO[];
  bounds: DiagramBoundsDTO;
}

export interface IbdScenePartDTO {
  id: string;
  name: string;
  qualifiedName: string;
  containerId?: string;
  type: string;
  x: number;
  y: number;
  width: number;
  height: number;
  isContainer: boolean;
  depth: number;
  attributes: Record<string, unknown>;
}

export interface IbdScenePortDTO {
  id: string;
  name: string;
  parentId: string;
  x: number;
  y: number;
  direction?: string;
  portType?: string;
  portSide?: string;
}

export interface IbdSceneConnectorDTO {
  id: string;
  source: string;
  target: string;
  sourceId: string;
  targetId: string;
  type: string;
  points: DiagramPointDTO[];
}

export interface IbdSceneRootDTO {
  name: string;
  parts: IbdScenePartDTO[];
  ports: IbdScenePortDTO[];
  connectors: IbdSceneConnectorDTO[];
  bounds: DiagramBoundsDTO;
}

export interface IbdDiagramSceneDTO {
  rootCandidates: string[];
  defaultRoot?: string;
  selectedRoot?: string;
  roots: Record<string, IbdSceneRootDTO>;
}

export interface DiagramSceneDTO {
  generalView?: GeneralDiagramSceneDTO;
  interconnectionView?: IbdDiagramSceneDTO;
}

export interface SysMLDiagramStatsDTO {
  nodeCount: number;
  edgeCount: number;
  buildTimeMs: number;
}

export interface SysMLDiagramResult {
  version: number;
  kind: string;
  sourceUri: string;
  scene: DiagramSceneDTO;
  warnings?: string[];
  stats?: SysMLDiagramStatsDTO;
}

// ---------------------------------------------------------------------------
// Activity Diagrams (optional - our server returns empty if not implemented)
// ---------------------------------------------------------------------------

export interface ActivityDiagramDTO {
  name: string;
  actions: ActivityActionDTO[];
  interface?: ActivityInterfaceDTO;
  decisions: DecisionNodeDTO[];
  flows: ControlFlowDTO[];
  states: ActivityStateDTO[];
  range: RangeDTO;
}

export interface ActivityInterfaceDTO {
  inputs: string[];
  outputs: string[];
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
