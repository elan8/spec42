export type { PreparedEdge, PreparedNode, PreparedView } from "./prepare";

export type ViewCandidateSource = "predefined" | "user-defined";

export interface SharedViewCandidate {
  id: string;
  name: string;
  supported: boolean;
  rendererView?: string;
  viewType?: string;
  description?: string;
  source?: ViewCandidateSource;
}

/** Scoped interconnection (IBD) payload from semantic_core. */
export interface SharedIbdPayload {
  parts?: unknown[];
  ports?: unknown[];
  connectors?: unknown[];
  containerGroups?: unknown[];
  packageContainerGroups?: unknown[];
  rootCandidates?: string[];
  defaultRoot?: string;
  rootViews?: Record<
    string,
    {
      parts?: unknown[];
      ports?: unknown[];
      connectors?: unknown[];
      containerGroups?: unknown[];
      packageContainerGroups?: unknown[];
    }
  >;
}

/** LSP visualization DTO consumed by prepareViewData. */
export interface SharedVisualizationPayload {
  view: string;
  selectedView?: string;
  selectedViewName?: string;
  emptyStateMessage?: string;
  viewCandidates?: SharedViewCandidate[];
  graph?: {
    nodes?: unknown[];
    edges?: unknown[];
  };
  generalViewGraph?: unknown;
  ibd?: SharedIbdPayload;
  /** Normalized behavior diagrams (extension prepare or server DTO). */
  diagrams?: unknown[];
  activityDiagrams?: unknown[];
  sequenceDiagrams?: unknown[];
  stateMachines?: unknown[];
  stateDiagrams?: unknown[];
  synthesizeInitialState?: boolean;
  activityLayoutDirection?: string;
  stateLayoutDirection?: string;
}
