import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { log, logError } from "../logger";

export interface PositionDTO { line: number; character: number }
export interface SysMLServerStats {
  uptime: number;
  memory: { rss: number };
  caches: { documents: number; symbolTables: number; semanticTokens: number };
}
export interface SysMLClearCacheResult {
  documents: number;
  symbolTables: number;
  semanticTokens: number;
}
export interface LibrarySearchItem {
  name: string; kind: string; container?: string; uri: string;
  range: { start: PositionDTO; end: PositionDTO }; score: number;
  source: "standard" | "domain" | "custom"; path?: string;
}
export interface LibrarySearchPackage { name: string; path: string; source: "standard" | "domain" | "custom"; symbols: LibrarySearchItem[] }
export interface LibrarySearchSource { source: "standard" | "domain" | "custom"; packages: LibrarySearchPackage[] }
export interface SysMLLibrarySearchResult { sources: LibrarySearchSource[]; symbolTotal: number; total: number }
export interface SysMLKparLibraryStatus { id: string; displayName: string; pinnedVersion: string; installedVersion?: string; format: string; available: boolean; resolvedPath?: string; sourceKind: string; versionMatches: boolean; isInstalled: boolean; statusMessage?: string }
export interface SysMLStdlibLibraryStatus { pinnedVersion: string; installedVersion?: string; format: string; available: boolean; resolvedPath?: string; sourceKind: string; versionMatches: boolean; statusMessage?: string }
export interface SysMLLibraryStatusResult { stdlib: SysMLStdlibLibraryStatus; kparLibraries: SysMLKparLibraryStatus[] }

export interface FeatureInspectorElementRef { id: string; name: string; qualifiedName: string; type: string; uri: string; range: { start: PositionDTO; end: PositionDTO } }
export interface FeatureInspectorResolution { status: "resolved" | "partial" | "unresolved" | "ambiguous" | "unsupported" | "notApplicable"; targets: FeatureInspectorElementRef[]; candidates?: FeatureInspectorElementRef[] }
export interface FeatureInspectorRelationship { type: string; peer: FeatureInspectorElementRef; provenance: "authored" | "implied" }
export type FeatureInspectorEvaluation = { state: "notApplicable" } | { state: "notRun" } | { state: "literal"; value: unknown; unit?: string } | { state: "evaluated"; value: unknown; unit?: string } | { state: "nonConstant" } | { state: "cyclic" } | { state: "unsupported" } | { state: "failed"; reason: string };
export type FeatureInspectorAnalysis = { state: "notApplicable" } | { state: "notRun" } | { state: "verdict"; passed: boolean } | { state: "computed"; value: unknown; unit?: string } | { state: "unsettled"; evaluation: string };
export interface FeatureInspectorInheritedFeature { feature: FeatureInspectorElementRef; declaredIn: FeatureInspectorElementRef }
export interface FeatureInspectorElement extends FeatureInspectorElementRef { role: "definition" | "usage" | "relationship" | "namespace" | "other"; declaration: string; parent?: FeatureInspectorElementRef; documentation?: string; multiplicity?: string; direction?: string; modifiers?: string[]; evaluation: FeatureInspectorEvaluation; analysis: FeatureInspectorAnalysis; typing: FeatureInspectorResolution; effectiveTyping?: FeatureInspectorResolution; specialization: FeatureInspectorResolution; subsetting?: FeatureInspectorResolution; redefinition?: FeatureInspectorResolution; inheritedFeatures?: FeatureInspectorInheritedFeature[]; metadata?: FeatureInspectorElementRef[]; incomingRelationships: FeatureInspectorRelationship[]; outgoingRelationships: FeatureInspectorRelationship[] }
export type FeatureInspectorSelectionKind = "keyword" | "element" | "reference" | "value" | "unit" | "other";
export interface FeatureInspectorSelection { kind: FeatureInspectorSelectionKind; text?: string; range?: { start: PositionDTO; end: PositionDTO } }
export interface FeatureInspectorLanguageHelp { keyword: string; description: string; syntax?: string }
export type FeatureInspectorReference = { status: "none" } | { status: "resolved"; element: FeatureInspectorElement } | { status: "ambiguous"; candidates: FeatureInspectorElement[] } | { status: "unresolved" } | { status: "unsupported" } | { status: "incomplete" };
export interface FeatureInspectorResult { version: number; sourceUri: string; requestedPosition: PositionDTO; selection: FeatureInspectorSelection; languageHelp?: FeatureInspectorLanguageHelp; containingElement?: FeatureInspectorElement; referenced: FeatureInspectorReference }

export function resolvedReference(result: FeatureInspectorResult | null | undefined): FeatureInspectorElement | undefined {
  return result?.referenced?.status === "resolved" ? result.referenced.element : undefined;
}
export function hasWorkspaceFolder(): boolean { return (vscode.workspace.workspaceFolders?.length ?? 0) > 0; }
export function isClientNotRunningError(error: unknown): boolean {
  return error instanceof Error && /Client is not running/i.test(error.message);
}

/** Thin adapter for the custom RPCs still consumed by the extension. */
export class LspModelProvider {
  constructor(private readonly client: LanguageClient, private readonly whenReady: Promise<void> = Promise.resolve()) {}
  private sendRequest<R>(method: string, params: unknown, token?: vscode.CancellationToken): Promise<R> {
    return token === undefined ? this.client.sendRequest<R>(method, params) : this.client.sendRequest<R>(method, params, token);
  }
  async getFeatureInspector(uri: string, position: PositionDTO, token?: vscode.CancellationToken): Promise<FeatureInspectorResult> {
    await this.whenReady;
    try {
      return await this.sendRequest<FeatureInspectorResult>("sysml/featureInspector", { textDocument: { uri }, uri, position }, token);
    } catch (error) { logError("getFeatureInspector failed", error); throw error; }
  }
  async getServerStats(): Promise<SysMLServerStats | undefined> {
    try { return await this.client.sendRequest<SysMLServerStats>("sysml/serverStats"); }
    catch (error) { log("getServerStats failed", error); return undefined; }
  }
  async clearCache(): Promise<SysMLClearCacheResult | undefined> {
    try { return await this.client.sendRequest<SysMLClearCacheResult>("sysml/clearCache"); }
    catch (error) { log("clearCache failed", error); return undefined; }
  }
  async searchLibraries(query: string, limit = 100): Promise<SysMLLibrarySearchResult> {
    return await this.client.sendRequest<SysMLLibrarySearchResult>("sysml/librarySearch", { query, limit });
  }
  async getLibraryStatus(): Promise<SysMLLibraryStatusResult> {
    return await this.client.sendRequest<SysMLLibraryStatusResult>("sysml/libraryStatus", {});
  }
}
