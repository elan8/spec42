import * as vscode from "vscode";
import { LibraryWebviewViewProvider } from "../../library/libraryWebviewViewProvider";
import { getStandardLibraryConfig } from "../configBridge";
import type { LspClientHandles } from "../lspClient";

export function registerLibraryCommands(
  context: vscode.ExtensionContext,
  libraryWebviewProvider: LibraryWebviewViewProvider,
  handles: Pick<LspClientHandles, "readSysandStatus">
): void {
  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.refresh", () => {
      libraryWebviewProvider.refresh();
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.managePaths", async () => {
      await vscode.commands.executeCommand(
        "workbench.action.openSettings",
        "spec42.libraryPaths"
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.search", async (query?: string) => {
      await libraryWebviewProvider.searchAndReveal(String(query ?? ""));
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.showStdLibStatus", async () => {
      const cfg = getStandardLibraryConfig();
      void vscode.window.showInformationMessage(
        `The SysML standard library is bundled with the Spec42 language server (release ${cfg.version}). Add extra library roots with spec42.libraryPaths if needed.`
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.sysand.showStatus", async () => {
      try {
        const status = await handles.readSysandStatus();
        libraryWebviewProvider.refresh();
        const roots = status.dependencyRoots.length;
        const warnings = status.warnings;
        const detail = [
          status.installed ? "installed" : "not installed",
          status.projectRoot ? `project: ${status.projectRoot}` : "no project manifest",
          `${roots} dependency root(s)`,
        ].join("; ");
        if (warnings.length > 0) {
          void vscode.window.showWarningMessage(`Sysand ${detail}. ${warnings[0]}`);
        } else {
          void vscode.window.showInformationMessage(`Sysand ${detail}.`);
        }
      } catch (error) {
        void vscode.window.showErrorMessage(
          `Unable to read Sysand status: ${error instanceof Error ? error.message : String(error)}`
        );
      }
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.sysand.refreshDependencies", async () => {
      await vscode.commands.executeCommand("sysml.sysand.showStatus");
      await vscode.commands.executeCommand("sysml.restartServer");
      libraryWebviewProvider.refresh();
    })
  );
}
