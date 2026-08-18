import type { ServerHealthState } from "../statusBar/statusBarViewModel";

export type WorkspaceLifecyclePhase =
  | "serverStarting"
  | "validatingFiles"
  | "buildingWorkspaceModel"
  | "workspaceReady"
  | "degraded";

export type WorkspaceLoadState =
  | "idle"
  | "pending"
  | "indexing"
  | "ready"
  | "degraded";

export type WorkspaceLifecycleInput = {
  languageClientReady: boolean;
  serverHealthState: ServerHealthState;
  hasWorkspaceFolder: boolean;
  semanticIndexReady: boolean;
  workspaceLoadState: WorkspaceLoadState;
  hasWorkspaceData: boolean;
  workspaceLoadFailures?: number;
  workspaceLoadCancelled?: boolean;
  workspaceLoadTruncated?: boolean;
};

export type WorkspaceLifecycle = {
  phase: WorkspaceLifecyclePhase;
  detail: string;
  progress?: string;
};

let snapshotProvider: (() => WorkspaceLifecycleInput) | undefined;

export function registerWorkspaceLifecycleSnapshotProvider(
  provider: () => WorkspaceLifecycleInput
): void {
  snapshotProvider = provider;
}

export function resetWorkspaceLifecycleSnapshotProvider(): void {
  snapshotProvider = undefined;
}

function defaultSnapshot(): WorkspaceLifecycleInput {
  return {
    languageClientReady: false,
    serverHealthState: "starting",
    hasWorkspaceFolder: false,
    semanticIndexReady: false,
    workspaceLoadState: "idle",
    hasWorkspaceData: false,
  };
}

export function getWorkspaceLifecycleInput(): WorkspaceLifecycleInput {
  return snapshotProvider?.() ?? defaultSnapshot();
}

export function deriveWorkspaceLifecycle(
  input: WorkspaceLifecycleInput
): WorkspaceLifecycle {
  const {
    languageClientReady,
    serverHealthState,
    hasWorkspaceFolder,
    semanticIndexReady,
    workspaceLoadState,
    hasWorkspaceData,
    workspaceLoadFailures = 0,
    workspaceLoadCancelled = false,
    workspaceLoadTruncated = false,
  } = input;

  if (
    !languageClientReady ||
    serverHealthState === "starting" ||
    serverHealthState === "restarting" ||
    serverHealthState === "crashed"
  ) {
    const detail =
      serverHealthState === "crashed"
        ? "SysML language server is not available."
        : serverHealthState === "restarting"
          ? "Restarting SysML language server."
          : "Starting SysML language server.";
    return { phase: "serverStarting", detail };
  }

  if (
    serverHealthState === "degraded" ||
    workspaceLoadState === "degraded" ||
    workspaceLoadFailures > 0 ||
    workspaceLoadCancelled ||
    workspaceLoadTruncated
  ) {
    const detail =
      workspaceLoadCancelled
        ? "Workspace indexing was cancelled."
        : workspaceLoadFailures > 0
          ? `Workspace model loaded with ${workspaceLoadFailures} failure(s).`
          : workspaceLoadTruncated
            ? "Workspace discovery limit reached."
            : "Workspace model may be incomplete.";
    return { phase: "degraded", detail };
  }

  if (
    serverHealthState === "indexing" ||
    workspaceLoadState === "pending" ||
    workspaceLoadState === "indexing" ||
    (hasWorkspaceFolder &&
      semanticIndexReady &&
      !hasWorkspaceData &&
      workspaceLoadState === "idle")
  ) {
    return {
      phase: "buildingWorkspaceModel",
      detail: "Building the cross-file workspace model.",
    };
  }

  if (
    hasWorkspaceFolder &&
    !semanticIndexReady &&
    !hasWorkspaceData
  ) {
    return {
      phase: "validatingFiles",
      detail: "Per-file validation is active; workspace model not built yet.",
    };
  }

  if (hasWorkspaceData && workspaceLoadState === "ready") {
    return { phase: "workspaceReady", detail: "" };
  }

  if (hasWorkspaceData) {
    return { phase: "workspaceReady", detail: "" };
  }

  if (!hasWorkspaceFolder) {
    return { phase: "workspaceReady", detail: "" };
  }

  return { phase: "validatingFiles", detail: "" };
}

export function getWorkspaceLifecycle(): WorkspaceLifecycle {
  return deriveWorkspaceLifecycle(getWorkspaceLifecycleInput());
}

type WorkspaceLifecycleListener = () => void;
const lifecycleListeners: WorkspaceLifecycleListener[] = [];

export function onWorkspaceLifecycleChanged(
  listener: WorkspaceLifecycleListener
): void {
  lifecycleListeners.push(listener);
}

export function notifyWorkspaceLifecycleChanged(): void {
  for (const listener of lifecycleListeners) {
    listener();
  }
}
