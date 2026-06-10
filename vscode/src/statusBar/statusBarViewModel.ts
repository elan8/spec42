import type { WorkspaceLifecyclePhase } from "../activation/workspaceLifecycle";

export type ServerHealthState =
  | "starting"
  | "ready"
  | "indexing"
  | "degraded"
  | "restarting"
  | "crashed";

export type StatusSummary = {
  errors: number;
  warnings: number;
  filesWithIssues?: number;
  totalFiles?: number;
};

export type Spec42StatusBarViewModel = {
  text: string;
  baseTooltip: string;
};

export function formatSpec42StatusBar(
  healthState: ServerHealthState,
  healthDetail: string,
  errors: number,
  warnings: number,
  workspaceSummary?: StatusSummary,
  activeSummary?: StatusSummary,
  lifecyclePhase?: WorkspaceLifecyclePhase
): Spec42StatusBarViewModel {
  const workspaceDiagTooltip = workspaceSummary
    ? `${workspaceSummary.errors} error(s), ${workspaceSummary.warnings} warning(s) across ${workspaceSummary.totalFiles ?? 0} workspace file(s) (${workspaceSummary.filesWithIssues ?? 0} with issues)`
    : undefined;
  const activeDiagTooltip = activeSummary
    ? `Active file: ${activeSummary.errors} error(s), ${activeSummary.warnings} warning(s)`
    : undefined;
  const diagnosticsSuffix =
    errors > 0 || warnings > 0 ? ` · ${errors}E ${warnings}W` : "";

  if (
    lifecyclePhase === "validatingFiles" ||
    lifecyclePhase === "buildingWorkspaceModel"
  ) {
    const activityLabel =
      lifecyclePhase === "validatingFiles"
        ? "Validating files"
        : "Building workspace model";
    return {
      text: `$(sync~spin) ${activityLabel}${diagnosticsSuffix}`,
      baseTooltip: [
        lifecyclePhase === "validatingFiles"
          ? "Problems reflect open files. Workspace model not built yet."
          : "Building the cross-file workspace model.",
        "Model Explorer and diagrams update when workspace indexing completes.",
        workspaceDiagTooltip,
        activeDiagTooltip,
        "Click for Spec42 actions.",
      ]
        .filter(Boolean)
        .join("\n"),
    };
  }

  const healthText =
    healthState === "starting"
      ? "$(sync~spin) SysML: Starting"
      : healthState === "indexing"
        ? "$(sync~spin) SysML: Indexing"
        : healthState === "restarting"
          ? "$(sync~spin) SysML: Restarting"
          : healthState === "degraded"
            ? "$(warning) SysML: Degraded"
            : healthState === "crashed"
              ? "$(error) SysML: Server stopped"
              : undefined;
  const diagnosticsText = (() => {
    const icon = errors > 0 ? "$(error)" : warnings > 0 ? "$(warning)" : "$(check)";
    return `${icon} SysML: ${errors}E ${warnings}W`;
  })();
  return {
    text: healthText ?? diagnosticsText,
    baseTooltip: healthText
      ? `Server state: ${healthState}${healthDetail ? `\n${healthDetail}` : ""}`
      : [
          workspaceDiagTooltip,
          activeDiagTooltip,
          "Click for Spec42 actions.",
        ]
          .filter(Boolean)
          .join("\n"),
  };
}
