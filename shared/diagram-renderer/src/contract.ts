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
  ibd?: unknown;
  activityDiagrams?: unknown[];
  sequenceDiagrams?: unknown[];
}
