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
  activeSummary?: StatusSummary
): Spec42StatusBarViewModel {
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
  const workspaceDiagTooltip = workspaceSummary
    ? `${workspaceSummary.errors} error(s), ${workspaceSummary.warnings} warning(s) across ${workspaceSummary.totalFiles ?? 0} workspace file(s) (${workspaceSummary.filesWithIssues ?? 0} with issues)`
    : undefined;
  const activeDiagTooltip = activeSummary
    ? `Active file: ${activeSummary.errors} error(s), ${activeSummary.warnings} warning(s)`
    : undefined;
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
