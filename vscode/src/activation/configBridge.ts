import * as vscode from "vscode";
import { DOMAIN_LIBRARIES_DEFAULTS } from "../generated/domainLibrariesDefaults";
import { STANDARD_LIBRARY_DEFAULTS } from "../generated/standardLibraryDefaults";

const CONFIG_SECTION = "spec42";
const LEGACY_CONFIG_SECTION = "sysml-language-server";

export type StandardLibraryConfig = {
  version: string;
  format: string;
  contentPath: string;
};

export type DomainLibrariesConfig = {
  version: string;
  format: string;
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

export function getStandardLibraryConfig(): StandardLibraryConfig {
  return {
    version: STANDARD_LIBRARY_DEFAULTS.version,
    format: STANDARD_LIBRARY_DEFAULTS.format,
    contentPath: STANDARD_LIBRARY_DEFAULTS.contentPath,
  };
}

export function getDomainLibrariesConfig(): DomainLibrariesConfig {
  return {
    version: DOMAIN_LIBRARIES_DEFAULTS.version,
    format: DOMAIN_LIBRARIES_DEFAULTS.format,
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
