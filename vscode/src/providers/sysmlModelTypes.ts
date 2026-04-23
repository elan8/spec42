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
  uri?: string;
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

export interface SourceAnchorDTO {
  filePath: string;
  range?: RangeDTO;
}

export interface SoftwareComponentDTO {
  id: string;
  name: string;
  kind: string;
  parentId?: string;
  crateName: string;
  modulePath: string;
  anchors: SourceAnchorDTO[];
  isExternal: boolean;
}

export interface SoftwareDependencyDTO {
  from: string;
  to: string;
  kind: string;
  sourceAnchor?: SourceAnchorDTO;
}

export interface SoftwareArchitectureModelDTO {
  components: SoftwareComponentDTO[];
  dependencies: SoftwareDependencyDTO[];
}

export interface SoftwareAnalysisSummaryDTO {
  crateCount: number;
  moduleCount: number;
  dependencyCount: number;
}

export interface SoftwareWorkspaceModelDTO {
  workspaceRoot: string;
  architecture: SoftwareArchitectureModelDTO;
  summary: SoftwareAnalysisSummaryDTO;
}

export interface SoftwareVisualizationViewCandidateDTO {
  id: string;
  name: string;
  supported: boolean;
  description?: string;
}

export interface SysMLElementDTO {
  id?: string;
  type: string;
  name: string;
  uri?: string;
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

export interface WorkspaceFileModelDTO {
  uri: string;
  elements: SysMLElementDTO[];
}

export interface WorkspaceModelSummaryDTO {
  scannedFiles: number;
  loadedFiles: number;
  failures: number;
  truncated: boolean;
}

export interface WorkspaceModelDTO {
  files: WorkspaceFileModelDTO[];
  semantic: SysMLElementDTO[];
  summary: WorkspaceModelSummaryDTO;
}

export interface SysMLModelParams {
  textDocument: { uri: string };
  scope?: Array<"graph" | "ibd" | "stats" | "activityDiagrams" | "workspaceVisualization">;
}

export interface IbdPartDTO {
  id: string;
  name: string;
  qualifiedName: string;
  uri?: string;
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

export interface IbdContainerGroupDTO {
  id: string;
  label: string;
  depth: number;
  qualifiedName: string;
  parentId?: string;
  memberPartIds: string[];
}

export interface IbdPackageContainerGroupDTO {
  id: string;
  label: string;
  qualifiedPackage: string;
  parentId?: string;
  memberPartIds: string[];
}

export interface IbdRootViewDTO {
  parts: IbdPartDTO[];
  ports: IbdPortDTO[];
  connectors: IbdConnectorDTO[];
  containerGroups?: IbdContainerGroupDTO[];
  packageContainerGroups?: IbdPackageContainerGroupDTO[];
}

export interface IbdDataDTO {
  parts: IbdPartDTO[];
  ports: IbdPortDTO[];
  connectors: IbdConnectorDTO[];
  containerGroups?: IbdContainerGroupDTO[];
  packageContainerGroups?: IbdPackageContainerGroupDTO[];
  rootCandidates: string[];
  rootViews?: Record<string, IbdRootViewDTO>;
  defaultRoot?: string;
}

export interface SysMLModelResult {
  version: number;
  graph?: SysMLGraphDTO;
  softwareArchitecture?: SoftwareArchitectureModelDTO;
  generalViewGraph?: SysMLGraphDTO;
  workspaceModel?: WorkspaceModelDTO;
  activityDiagrams?: ActivityDiagramDTO[];
  ibd?: IbdDataDTO;
  stats?: SysMLModelStatsDTO;
}

export interface VisualizationViewCandidateDTO {
  id: string;
  name: string;
  rendererView?: string;
  supported: boolean;
  viewType?: string;
  description?: string;
}

export interface SysMLVisualizationParams {
  workspaceRootUri: string;
  view: string;
  selectedView?: string;
}

export interface SoftwareVisualizationParams {
  workspaceRootUri: string;
  view: string;
}

export interface SoftwareAnalyzeWorkspaceParams {
  workspaceRootUri: string;
}

export interface SoftwareProjectViewParams {
  workspaceRootUri: string;
  view: string;
  workspaceModel: SoftwareWorkspaceModelDTO;
}

export interface SysMLVisualizationResult {
  version: number;
  view: string;
  workspaceRootUri: string;
  viewCandidates: VisualizationViewCandidateDTO[];
  selectedView?: string;
  selectedViewName?: string;
  emptyStateMessage?: string;
  graph?: SysMLGraphDTO;
  softwareArchitecture?: SoftwareArchitectureModelDTO;
  generalViewGraph?: SysMLGraphDTO;
  workspaceModel?: WorkspaceModelDTO;
  activityDiagrams?: ActivityDiagramDTO[];
  ibd?: IbdDataDTO;
  stats?: SysMLModelStatsDTO;
}

export interface SoftwareVisualizationResult {
  version: number;
  view: string;
  workspaceRootUri: string;
  views: SoftwareVisualizationViewCandidateDTO[];
  emptyStateMessage?: string;
  graph: SysMLGraphDTO;
  softwareArchitecture: SoftwareArchitectureModelDTO;
  workspaceModel: WorkspaceModelDTO;
  stats: SysMLModelStatsDTO;
}

export interface SoftwareAnalyzeWorkspaceResult {
  version: number;
  workspaceModel: SoftwareWorkspaceModelDTO;
}

// ---------------------------------------------------------------------------
// Activity Diagrams (optional - our server returns empty if not implemented)
// ---------------------------------------------------------------------------

export interface ActivityDiagramDTO {
  id?: string;
  name: string;
  packagePath?: string;
  sourceKind?: "actionDef" | "performer" | string;
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
