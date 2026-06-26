import type { ActivityDiagramDTO } from "../generated/backend/ActivityDiagramDTO";
import type { GraphEdgeDTO } from "../generated/backend/GraphEdgeDTO";
import type { GraphNodeDTO } from "../generated/backend/GraphNodeDTO";
import type { IbdDataDTO } from "../generated/backend/IbdDataDTO";
import type { InterconnectionSceneDTO } from "../generated/backend/InterconnectionSceneDTO";
import type { SequenceDiagramDTO } from "../generated/backend/SequenceDiagramDTO";
import type { SysMLElementDTO } from "../generated/backend/SysMLElementDTO";
import type { SysMLGraphDTO } from "../generated/backend/SysMLGraphDTO";
import type { SysMLModelStatsDTO } from "../generated/backend/SysMLModelStatsDTO";
import type { SysMLVisualizationResult } from "../generated/backend/SysMLVisualizationResult";
import type { WorkspaceModelDTO } from "../generated/backend/WorkspaceModelDTO";

export type { ActivityActionDTO } from "../generated/backend/ActivityActionDTO";
export type { ActivityDiagramDTO } from "../generated/backend/ActivityDiagramDTO";
export type { ActivityInterfaceDTO } from "../generated/backend/ActivityInterfaceDTO";
export type { ActivityStateDTO } from "../generated/backend/ActivityStateDTO";
export type { BehaviorPositionDTO } from "../generated/backend/BehaviorPositionDTO";
export type { BehaviorRangeDTO } from "../generated/backend/BehaviorRangeDTO";
export type { BranchDTO } from "../generated/backend/BranchDTO";
export type { ControlFlowDTO } from "../generated/backend/ControlFlowDTO";
export type { DecisionNodeDTO } from "../generated/backend/DecisionNodeDTO";
export type { GraphEdgeDTO } from "../generated/backend/GraphEdgeDTO";
export type { GraphNodeDTO } from "../generated/backend/GraphNodeDTO";
export type { IbdConnectorDTO } from "../generated/backend/IbdConnectorDTO";
export type { IbdContainerGroupDTO } from "../generated/backend/IbdContainerGroupDTO";
export type { IbdDataDTO } from "../generated/backend/IbdDataDTO";
export type { IbdPackageContainerGroupDTO } from "../generated/backend/IbdPackageContainerGroupDTO";
export type { IbdPartDTO } from "../generated/backend/IbdPartDTO";
export type { IbdPortDTO } from "../generated/backend/IbdPortDTO";
export type { IbdRootViewDTO } from "../generated/backend/IbdRootViewDTO";
export type { InterconnectionSceneContainerDTO } from "../generated/backend/InterconnectionSceneContainerDTO";
export type { InterconnectionSceneDiagnosticDTO } from "../generated/backend/InterconnectionSceneDiagnosticDTO";
export type { InterconnectionSceneDTO } from "../generated/backend/InterconnectionSceneDTO";
export type { InterconnectionSceneEdgeDTO } from "../generated/backend/InterconnectionSceneEdgeDTO";
export type { InterconnectionSceneNodeDTO } from "../generated/backend/InterconnectionSceneNodeDTO";
export type { InterconnectionScenePortDTO } from "../generated/backend/InterconnectionScenePortDTO";
export type { InterconnectionSceneViewDTO } from "../generated/backend/InterconnectionSceneViewDTO";
export type { PositionDTO } from "../generated/backend/PositionDTO";
export type { PreparedEdgeDTO } from "../generated/backend/PreparedEdgeDTO";
export type { PreparedNodeDTO } from "../generated/backend/PreparedNodeDTO";
export type { PreparedViewDTO } from "../generated/backend/PreparedViewDTO";
export type { RangeDTO } from "../generated/backend/RangeDTO";
export type { RegionDTO } from "../generated/backend/RegionDTO";
export type { RelationshipDTO } from "../generated/backend/RelationshipDTO";
export type { SequenceActivationDTO } from "../generated/backend/SequenceActivationDTO";
export type { SequenceDiagramDTO } from "../generated/backend/SequenceDiagramDTO";
export type { SequenceFragmentDTO } from "../generated/backend/SequenceFragmentDTO";
export type { SequenceLifelineDTO } from "../generated/backend/SequenceLifelineDTO";
export type { SequenceMessageDTO } from "../generated/backend/SequenceMessageDTO";
export type { SequenceOperandDTO } from "../generated/backend/SequenceOperandDTO";
export type { StateMachineDTO } from "../generated/backend/StateMachineDTO";
export type { StateNodeDTO } from "../generated/backend/StateNodeDTO";
export type { StateNodeElementDTO } from "../generated/backend/StateNodeElementDTO";
export type { StateTransitionDTO } from "../generated/backend/StateTransitionDTO";
export type { SysMLElementDTO } from "../generated/backend/SysMLElementDTO";
export type { SysMLGraphDTO } from "../generated/backend/SysMLGraphDTO";
export type { SysMLModelStatsDTO } from "../generated/backend/SysMLModelStatsDTO";
export type { SysMLVisualizationResult } from "../generated/backend/SysMLVisualizationResult";
export type { VisualizationViewCandidateDTO } from "../generated/backend/VisualizationViewCandidateDTO";
export type { WorkspaceFileModelDTO } from "../generated/backend/WorkspaceFileModelDTO";
export type { WorkspaceModelDTO } from "../generated/backend/WorkspaceModelDTO";
export type { WorkspaceModelSummaryDTO } from "../generated/backend/WorkspaceModelSummaryDTO";

/** LSP notification `spec42/semanticIndexReady` payload. */
export interface SemanticIndexReadyParams {
  lifecycle: string;
  semanticStateVersion: number;
  workspaceFileCount: number;
}

export interface SysMLModelParams {
  textDocument: { uri: string };
  scope?: Array<"graph" | "ibd" | "stats" | "activityDiagrams" | "sequenceDiagrams" | "workspaceVisualization">;
}

export interface SysMLVisualizationParams {
  workspaceRootUri: string;
  view: string;
  selectedView?: string;
}

export interface SysMLModelResult {
  version: number;
  graph?: SysMLGraphDTO;
  generalViewGraph?: SysMLGraphDTO;
  workspaceModel?: WorkspaceModelDTO;
  activityDiagrams?: ActivityDiagramDTO[];
  sequenceDiagrams?: SequenceDiagramDTO[];
  ibd?: IbdDataDTO;
  interconnectionScene?: InterconnectionSceneDTO;
  preparedView?: SysMLVisualizationResult["preparedView"];
  stats?: SysMLModelStatsDTO;
}
