export interface InterconnectionPipelineExport {
  rawPayload: Record<string, unknown>;
  preparedScene: Record<string, unknown>;
  elkInput: Record<string, unknown>;
  elkOutput: Record<string, unknown> | null;
  routeSummary: Record<string, unknown>;
}

export declare function exportInterconnectionPipeline(
  payload: Record<string, unknown>,
): Promise<InterconnectionPipelineExport>;
