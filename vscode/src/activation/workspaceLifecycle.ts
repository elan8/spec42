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

type LifecycleSurface = "statusBar" | "explorer" | "visualizer";

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
      detail: "Building cross-file workspace model for Model Explorer and diagrams.",
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

export function getLifecycleMessage(
  surface: LifecycleSurface,
  phase: WorkspaceLifecyclePhase,
  _detail?: string
): string {
  switch (surface) {
    case "statusBar":
      switch (phase) {
        case "serverStarting":
          return "Starting SysML server";
        case "validatingFiles":
          return "Validating files";
        case "buildingWorkspaceModel":
          return "Building workspace model";
        case "degraded":
          return "Workspace degraded";
        case "workspaceReady":
          return "";
      }
      break;
    case "explorer":
      switch (phase) {
        case "validatingFiles":
          return "Validating files — workspace model not built yet";
        case "buildingWorkspaceModel":
          return "Building workspace model";
        case "degraded":
          return "Workspace results may be incomplete";
        default:
          return "";
      }
      break;
    case "visualizer":
      switch (phase) {
        case "serverStarting":
          return "Starting SysML language server...";
        case "validatingFiles":
          return "Validating SysML files...";
        case "buildingWorkspaceModel":
          return "Building workspace model...";
        case "workspaceReady":
          return "Preparing diagram...";
        case "degraded":
          return "Workspace model may be incomplete...";
      }
      break;
  }
  return "";
}

export function getVisualizerLoadingMessage(): string {
  const lifecycle = getWorkspaceLifecycle();
  const message = getLifecycleMessage("visualizer", lifecycle.phase, lifecycle.detail);
  return message || "Loading workspace model...";
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
