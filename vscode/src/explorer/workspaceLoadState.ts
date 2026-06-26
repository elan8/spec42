import * as vscode from "vscode";

export type WorkspaceLoadStatus = {
  state: "idle" | "pending" | "indexing" | "ready" | "degraded";
  scannedFiles: number;
  loadedFiles: number;
  perPatternLimit?: number;
  truncated: boolean;
  cancelled: boolean;
  failures: number;
};

export type WorkspaceLoadOptions = {
  runId: string;
  token?: vscode.CancellationToken;
};

export type WorkspaceLoadResult = {
  runId: string;
  fileCount: number;
  loadedFiles: number;
  failures: number;
  cancelled: number;
  committed: boolean;
  stale: boolean;
  totalMs: number;
};

export type InFlightWorkspaceLoad = {
  runId: string;
  promise: Promise<WorkspaceLoadResult>;
};
