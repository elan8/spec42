import * as vscode from "vscode";
import { LibraryWebviewViewProvider } from "../../library/libraryWebviewViewProvider";
import { KPAR_LIBRARIES_DEFAULTS, kparLibraryDefaults } from "../../generated/kparLibrariesDefaults";
import { classifyKparLibraryStatus } from "../../library/libraryStatusViewModel";
import {
  getDisabledLibraries,
  getKparLibraryPathOverrides,
  getStandardLibraryConfig,
} from "../configBridge";
import type { LspClientHandles } from "../lspClient";

const CONFIG_SECTION = "spec42";

function configurationTarget(): vscode.ConfigurationTarget {
  return (vscode.workspace.workspaceFolders?.length ?? 0) > 0
    ? vscode.ConfigurationTarget.Workspace
    : vscode.ConfigurationTarget.Global;
}

async function updateDisabledLibraries(ids: string[]): Promise<void> {
  await vscode.workspace
    .getConfiguration(CONFIG_SECTION)
    .update("disabledLibraries", ids, configurationTarget());
}

async function updateKparLibraryPaths(paths: Record<string, string>): Promise<void> {
  await vscode.workspace
    .getConfiguration(CONFIG_SECTION)
    .update("kparLibraryPaths", paths, configurationTarget());
}

async function promptRestartToApplyLibraryChange(message: string): Promise<void> {
  const selection = await vscode.window.showInformationMessage(message, "Restart Server");
  if (selection === "Restart Server") {
    await vscode.commands.executeCommand("sysml.restartServer");
  }
}

export function registerLibraryCommands(
  context: vscode.ExtensionContext,
  libraryWebviewProvider: LibraryWebviewViewProvider,
  handles: Pick<
    LspClientHandles,
    "readSysandStatus" | "lspModelProvider"
  >
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
        `The SysML standard library is bundled with the Spec42 language server as ${cfg.format.toUpperCase()} (release ${cfg.version}). Add extra library roots with spec42.libraryPaths if needed.`
      );
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.library.showKparLibraryStatus",
      async (libraryId?: string) => {
        const id = String(libraryId ?? "domain");
        const defaults = kparLibraryDefaults(id);
        try {
          const status = await handles.lspModelProvider.getLibraryStatus();
          const library =
            status.kparLibraries.find((entry) => entry.id === id) ??
            (defaults
              ? {
                  id,
                  displayName: defaults.displayName,
                  pinnedVersion: defaults.version,
                  format: defaults.format,
                  available: false,
                  sourceKind: "none",
                  versionMatches: false,
                  isInstalled: false,
                }
              : undefined);
          if (!library) {
            void vscode.window.showWarningMessage(`Unknown managed library '${id}'.`);
            return;
          }
          const version = library.installedVersion
            ? `${library.pinnedVersion} (installed ${library.installedVersion})`
            : library.pinnedVersion;
          if (library.sourceKind === "disabled") {
            void vscode.window.showInformationMessage(
              `${library.displayName} is disabled (spec42.disabledLibraries). Enable it to resume indexing.`
            );
            return;
          }
          const { label } = classifyKparLibraryStatus(library);
          const sourceDescription =
            library.sourceKind === "override" || library.sourceKind === "custom"
              ? label.toLowerCase()
              : `source ${library.sourceKind}`;
          void vscode.window.showInformationMessage(
            `${library.displayName} are bundled with Spec42 as ${library.format.toUpperCase()} (revision ${version}; ${sourceDescription}).`
          );
        } catch (error) {
          if (defaults) {
            void vscode.window.showInformationMessage(
              `${defaults.displayName} are bundled with Spec42 as ${defaults.format.toUpperCase()} (revision ${defaults.version}).`
            );
            return;
          }
          void vscode.window.showErrorMessage(
            `Unable to read library status: ${error instanceof Error ? error.message : String(error)}`
          );
        }
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand("sysml.library.showDomainLibrariesStatus", async () => {
      await vscode.commands.executeCommand("sysml.library.showKparLibraryStatus", "domain");
    })
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.library.toggleLibrary",
      async (libraryId?: string) => {
        const disabled = getDisabledLibraries();
        let id = libraryId;
        if (!id) {
          const knownIds = new Set<string>([
            ...KPAR_LIBRARIES_DEFAULTS.map((library) => library.id),
            ...Object.keys(getKparLibraryPathOverrides()),
            ...disabled,
          ]);
          const picked = await vscode.window.showQuickPick(
            Array.from(knownIds).map((candidateId) => ({
              label: candidateId,
              description: disabled.includes(candidateId) ? "Disabled" : "Enabled",
            })),
            { placeHolder: "Select a library to enable/disable" }
          );
          id = picked?.label;
        }
        if (!id) {
          return;
        }
        const nextDisabled = disabled.includes(id)
          ? disabled.filter((entry) => entry !== id)
          : [...disabled, id];
        await updateDisabledLibraries(nextDisabled);
        libraryWebviewProvider.refresh();
        await promptRestartToApplyLibraryChange(
          `${id} is now ${nextDisabled.includes(id) ? "disabled" : "enabled"}. Restart the SysML language server to apply this change.`
        );
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.library.setLocalLibraryPath",
      async (libraryId?: string) => {
        let id = libraryId;
        if (!id) {
          const knownIds = KPAR_LIBRARIES_DEFAULTS.map((library) => library.id);
          const customEntry = "$(add) Add a custom library id...";
          const picked = await vscode.window.showQuickPick([...knownIds, customEntry], {
            placeHolder: "Select a library id, or add a custom one",
          });
          if (!picked) {
            return;
          }
          if (picked === customEntry) {
            id = (
              await vscode.window.showInputBox({
                prompt: "Custom KPAR library id (letters, numbers, hyphens)",
                validateInput: (value) =>
                  /^[a-zA-Z0-9-]+$/.test(value.trim())
                    ? undefined
                    : "Use letters, numbers, and hyphens only.",
              })
            )?.trim();
          } else {
            id = picked;
          }
        }
        if (!id) {
          return;
        }
        const selection = await vscode.window.showOpenDialog({
          canSelectFiles: true,
          canSelectFolders: true,
          canSelectMany: false,
          openLabel: `Use as local "${id}" library`,
          title: `Select a directory or .kpar file for library "${id}"`,
        });
        const picked = selection?.[0];
        if (!picked) {
          return;
        }
        const overrides = { ...getKparLibraryPathOverrides(), [id]: picked.fsPath };
        await updateKparLibraryPaths(overrides);
        libraryWebviewProvider.refresh();
        await promptRestartToApplyLibraryChange(
          `Library "${id}" will use ${picked.fsPath}. Restart the SysML language server to apply this change.`
        );
      }
    )
  );

  context.subscriptions.push(
    vscode.commands.registerCommand(
      "sysml.library.removeLocalLibraryPath",
      async (libraryId?: string) => {
        const overrides = getKparLibraryPathOverrides();
        let id = libraryId;
        if (!id) {
          const ids = Object.keys(overrides);
          if (ids.length === 0) {
            void vscode.window.showInformationMessage(
              "No local or custom library paths are configured."
            );
            return;
          }
          id = await vscode.window.showQuickPick(ids, {
            placeHolder: "Select a local/custom library path to remove",
          });
        }
        if (!id || !(id in overrides)) {
          return;
        }
        const next = { ...overrides };
        delete next[id];
        await updateKparLibraryPaths(next);
        libraryWebviewProvider.refresh();
        await promptRestartToApplyLibraryChange(
          `Removed the local/custom path for "${id}". Restart the SysML language server to apply this change.`
        );
      }
    )
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
