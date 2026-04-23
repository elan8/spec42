import * as vscode from "vscode";
import type { LspModelProvider } from "../providers/lspModelProvider";
import type { SoftwareWorkspaceModelDTO } from "../providers/sysmlModelTypes";

export type SoftwareAnalysisStatus = "idle" | "running" | "ready" | "failed";

export interface SoftwareAnalysisEntry {
  workspaceRootUri: string;
  status: SoftwareAnalysisStatus;
  model?: SoftwareWorkspaceModelDTO;
  startedAt?: number;
  finishedAt?: number;
  durationMs?: number;
  errorMessage?: string;
}

type ProgressStep = {
  increment: number;
  message: string;
};

const ANALYSIS_PROGRESS_STEPS: ProgressStep[] = [
  { increment: 10, message: "Discovering Rust workspace..." },
  { increment: 25, message: "Extracting crates and modules..." },
  { increment: 30, message: "Resolving dependencies..." },
  { increment: 35, message: "Building software workspace model..." },
];

export class SoftwareAnalysisStore {
  private readonly entries = new Map<string, SoftwareAnalysisEntry>();
  private readonly inFlight = new Map<string, Promise<SoftwareAnalysisEntry>>();
  private readonly changeEmitter = new vscode.EventEmitter<SoftwareAnalysisEntry>();

  readonly onDidChange = this.changeEmitter.event;

  get(workspaceRootUri: string): SoftwareAnalysisEntry {
    return (
      this.entries.get(workspaceRootUri) ?? {
        workspaceRootUri,
        status: "idle",
      }
    );
  }

  set(entry: SoftwareAnalysisEntry): void {
    this.entries.set(entry.workspaceRootUri, entry);
    this.changeEmitter.fire(entry);
  }

  clear(workspaceRootUri: string): void {
    this.entries.delete(workspaceRootUri);
    this.changeEmitter.fire({ workspaceRootUri, status: "idle" });
  }

  async runAnalysis(
    workspaceRootUri: string,
    lspModelProvider: LspModelProvider
  ): Promise<SoftwareAnalysisEntry> {
    const existing = this.inFlight.get(workspaceRootUri);
    if (existing) {
      return await existing;
    }

    const startedAt = Date.now();
    const runningEntry: SoftwareAnalysisEntry = {
      workspaceRootUri,
      status: "running",
      startedAt,
    };
    this.set(runningEntry);

    const promise = (async () => {
      try {
        return await vscode.window.withProgress(
          {
            location: vscode.ProgressLocation.Notification,
            title: "Spec42: Software workspace analysis",
            cancellable: false,
          },
          async (progress) => {
            for (const step of ANALYSIS_PROGRESS_STEPS.slice(0, 3)) {
              progress.report(step);
              await new Promise((resolve) => setTimeout(resolve, 10));
            }
            const result = await lspModelProvider.analyzeSoftwareWorkspace(workspaceRootUri);
            progress.report(ANALYSIS_PROGRESS_STEPS[3]);
            const finishedAt = Date.now();
            const readyEntry: SoftwareAnalysisEntry = {
              workspaceRootUri,
              status: "ready",
              model: result.workspaceModel,
              startedAt,
              finishedAt,
              durationMs: finishedAt - startedAt,
            };
            this.set(readyEntry);
            return readyEntry;
          }
        );
      } catch (error) {
        const failedEntry: SoftwareAnalysisEntry = {
          workspaceRootUri,
          status: "failed",
          startedAt,
          finishedAt: Date.now(),
          durationMs: Date.now() - startedAt,
          errorMessage: error instanceof Error ? error.message : String(error),
        };
        this.set(failedEntry);
        throw error;
      } finally {
        this.inFlight.delete(workspaceRootUri);
      }
    })();

    this.inFlight.set(workspaceRootUri, promise);
    return await promise;
  }

  dispose(): void {
    this.changeEmitter.dispose();
  }
}
