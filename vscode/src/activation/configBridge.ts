import * as vscode from "vscode";
import { DOMAIN_LIBRARIES_DEFAULTS } from "../generated/domainLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "../generated/standardLibraryDefaults";

const CONFIG_SECTION = "spec42";
const LEGACY_CONFIG_SECTION = "sysml-language-server";

export type StartupWorkspaceIndexingMode = "lazy" | "background" | "eager";

export type StandardLibraryConfig = {
  enabled: boolean;
  version: string;
  repo: string;
  contentPath: string;
};

export type DomainLibrariesConfig = {
  version: string;
  repo: string;
  contentPath: string;
};

function getConfig() {
  return {
    primary: vscode.workspace.getConfiguration(CONFIG_SECTION),
    legacy: vscode.workspace.getConfiguration(LEGACY_CONFIG_SECTION),
  };
}

export function getConfigString(key: string): string | undefined {
  const { primary, legacy } = getConfig();
  return primary.get<string>(key) ?? legacy.get<string>(key) ?? undefined;
}

export function getConfigStringArray(key: string): string[] | undefined {
  const { primary, legacy } = getConfig();
  return primary.get<string[]>(key) ?? legacy.get<string[]>(key) ?? undefined;
}

export function getConfigBoolean(key: string, defaultValue: boolean): boolean {
  const { primary, legacy } = getConfig();
  return primary.get<boolean>(key) ?? legacy.get<boolean>(key) ?? defaultValue;
}

export function getConfigNumber(key: string, defaultValue: number): number {
  const { primary, legacy } = getConfig();
  return primary.get<number>(key) ?? legacy.get<number>(key) ?? defaultValue;
}

export function getStartupWorkspaceIndexingMode(): StartupWorkspaceIndexingMode {
  const configured = getConfigString("startup.workspaceIndexing");
  if (
    configured === "lazy" ||
    configured === "background" ||
    configured === "eager"
  ) {
    return configured;
  }
  return "background";
}

export function getStandardLibraryConfig(): StandardLibraryConfig {
  return {
    enabled: getConfigBoolean("standardLibrary.enabled", true),
    version:
      getConfigString("standardLibrary.version") ??
      STANDARD_LIBRARY_DEFAULTS.version,
    repo:
      getConfigString("standardLibrary.repo") ?? STANDARD_LIBRARY_DEFAULTS.repo,
    contentPath:
      getConfigString("standardLibrary.contentPath") ??
      STANDARD_LIBRARY_DEFAULTS.contentPath,
  };
}

export function getDomainLibrariesConfig(): DomainLibrariesConfig {
  return {
    version:
      getConfigString("domainLibraries.version") ??
      DOMAIN_LIBRARIES_DEFAULTS.version,
    repo:
      getConfigString("domainLibraries.repo") ?? DOMAIN_LIBRARIES_DEFAULTS.repo,
    contentPath: DOMAIN_LIBRARIES_DEFAULTS.contentPath,
  };
}

export function isDefaultServerPath(value: string): boolean {
  return value === "spec42" || value === "sysml-language-server";
}

export function isSysmlDoc(doc: vscode.TextDocument | undefined): boolean {
  if (!doc) return false;
  return doc.languageId === "sysml" || doc.languageId === "kerml";
}

export function activeSysmlDocument(
  doc?: vscode.TextDocument
): vscode.TextDocument | undefined {
  if (doc && isSysmlDoc(doc)) {
    return doc;
  }
  const active = vscode.window.activeTextEditor?.document;
  return isSysmlDoc(active) ? active : undefined;
}
