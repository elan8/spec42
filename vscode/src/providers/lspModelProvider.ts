import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { log, logError, logPerfEvent } from "../logger";
import type {
  GraphNodeDTO,
  SysMLDiagramParams,
  SysMLDiagramResult,
  SysMLGraphDTO,
  PositionDTO,
  SysMLElementDTO,
  SysMLModelParams,
  SysMLModelResult,
} from "./sysmlModelTypes";

function logPerf(event: string, extra?: Record<string, unknown>): void {
  logPerfEvent(event, extra);
}

function isCancellationError(error: unknown): boolean {
  return error instanceof vscode.CancellationError;
}

function isClientNotRunningError(error: unknown): boolean {
  return error instanceof Error && /Client is not running/i.test(error.message);
}

/** Convert GraphNodeDTO to SysMLElementDTO for findElement compatibility. */
function graphNodeToElementDTO(
  node: GraphNodeDTO,
  graph: SysMLGraphDTO,
  visited: Set<string> = new Set()
): SysMLElementDTO {
  if (visited.has(node.id)) {
    return { id: node.id, type: node.type, name: node.name, range: node.range, children: [], attributes: node.attributes || {}, relationships: [] };
  }
  visited.add(node.id);
  const children = (graph.nodes || []).filter((n) => n.parentId === node.id);
  const childDTOs = children.map((c) => graphNodeToElementDTO(c, graph, visited));
  const edgeType = (e: { type?: string; rel_type?: string }) => e.type || e.rel_type || '';
  const relationships = (graph.edges || [])
    .filter((e) => e.source === node.id && edgeType(e).toLowerCase() !== 'contains')
    .map((e) => ({ source: e.source, target: e.target, type: edgeType(e), name: e.name }));
  return {
    id: node.id,
    type: node.type,
    name: node.name,
    range: node.range,
    children: childDTOs,
    attributes: node.attributes || {},
    relationships,
  };
}

/** Convert LSP PositionDTO to vscode.Position. */
function toVscodePosition(p: PositionDTO): vscode.Position {
  return new vscode.Position(p.line, p.character);
}

/** Convert LSP RangeDTO to vscode.Range. */
export function toVscodeRange(r: { start: PositionDTO; end: PositionDTO }): vscode.Range {
  return new vscode.Range(toVscodePosition(r.start), toVscodePosition(r.end));
}

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
  name: string;
  kind: string;
  container?: string;
  uri: string;
  range: { start: PositionDTO; end: PositionDTO };
  score: number;
  source: "standard" | "custom";
  path?: string;
}

export interface LibrarySearchPackage {
  name: string;
  path: string;
  source: "standard" | "custom";
  symbols: LibrarySearchItem[];
}

export interface LibrarySearchSource {
  source: "standard" | "custom";
  packages: LibrarySearchPackage[];
}

export interface SysMLLibrarySearchResult {
  sources: LibrarySearchSource[];
  symbolTotal: number;
  total: number;
}

type NormalizedScope = NonNullable<SysMLModelParams["scope"]>[number];

type CachedModelResult = {
  key: string;
  uri: string;
  scopes: NormalizedScope[];
  storedAt: number;
  result: SysMLModelResult;
};

type InFlightModelRequest = {
  key: string;
  uri: string;
  scopes: NormalizedScope[];
  requestId: string;
  caller: string;
  promise: Promise<SysMLModelResult>;
};

const MODEL_CACHE_TTL_MS = 5000;

function normalizeScopes(scopes?: SysMLModelParams["scope"]): NormalizedScope[] {
  return [...new Set((scopes ?? []).slice().sort())] as NormalizedScope[];
}

function scopesKey(scopes: NormalizedScope[]): string {
  return scopes.length > 0 ? scopes.join("|") : "(default)";
}

function buildRequestKey(uri: string, scopes: NormalizedScope[]): string {
  return `${uri}::${scopesKey(scopes)}`;
}

function canReuseCachedScopes(
  requested: NormalizedScope[],
  available: NormalizedScope[]
): boolean {
  if (requested.length === 0 || available.length === 0) {
    return (
      requested.length === available.length &&
      requested.every((scope, index) => scope === available[index])
    );
  }
  if (
    requested.includes("workspaceVisualization") !==
    available.includes("workspaceVisualization")
  ) {
    return false;
  }
  const availableSet = new Set(available);
  return requested.every((scope) => availableSet.has(scope));
}

export class LspModelProvider {
  private readonly inFlightModelRequests = new Map<string, InFlightModelRequest>();
  private readonly modelResultCache = new Map<string, CachedModelResult>();
  private nextModelRequestId = 0;

  constructor(
    private readonly client: LanguageClient,
    /** Resolves when the LSP client is ready. Prevents getModel before didOpen is processed. */
    private readonly whenReady: Promise<void> = Promise.resolve()
  ) {}

  private nextRequestId(): string {
    this.nextModelRequestId += 1;
    return `model-${this.nextModelRequestId}`;
  }

  private findReusableCacheEntry(
    uri: string,
    scopes: NormalizedScope[]
  ): { entry: CachedModelResult; reuseType: "exact" | "superset" } | undefined {
    const now = Date.now();
    let supersetEntry: CachedModelResult | undefined;
    for (const entry of this.modelResultCache.values()) {
      if (entry.uri !== uri) {
        continue;
      }
      if (now - entry.storedAt > MODEL_CACHE_TTL_MS) {
        this.modelResultCache.delete(entry.key);
        continue;
      }
      if (!canReuseCachedScopes(scopes, entry.scopes)) {
        continue;
      }
      if (
        entry.scopes.length === scopes.length &&
        entry.scopes.every((scope, index) => scope === scopes[index])
      ) {
        return { entry, reuseType: "exact" };
      }
      if (!supersetEntry) {
        supersetEntry = entry;
      }
    }
    return supersetEntry
      ? { entry: supersetEntry, reuseType: "superset" }
      : undefined;
  }

  private findReusableInFlightRequest(
    uri: string,
    scopes: NormalizedScope[]
  ): { request: InFlightModelRequest; reuseType: "exact" | "superset" } | undefined {
    let supersetRequest: InFlightModelRequest | undefined;
    for (const request of this.inFlightModelRequests.values()) {
      if (request.uri !== uri) {
        continue;
      }
      if (!canReuseCachedScopes(scopes, request.scopes)) {
        continue;
      }
      if (
        request.scopes.length === scopes.length &&
        request.scopes.every((scope, index) => scope === scopes[index])
      ) {
        return { request, reuseType: "exact" };
      }
      if (!supersetRequest) {
        supersetRequest = request;
      }
    }
    return supersetRequest
      ? { request: supersetRequest, reuseType: "superset" }
      : undefined;
  }

  private storeModelResult(
    uri: string,
    scopes: NormalizedScope[],
    result: SysMLModelResult
  ): void {
    const key = buildRequestKey(uri, scopes);
    this.modelResultCache.set(key, {
      key,
      uri,
      scopes,
      storedAt: Date.now(),
      result,
    });
  }

  private async awaitWithCancellation<T>(
    promise: Promise<T>,
    token?: vscode.CancellationToken
  ): Promise<T> {
    if (!token) {
      return await promise;
    }
    if (token.isCancellationRequested) {
      throw new vscode.CancellationError();
    }
    return await new Promise<T>((resolve, reject) => {
      let settled = false;
      const subscription = token.onCancellationRequested(() => {
        if (settled) {
          return;
        }
        settled = true;
        subscription.dispose();
        reject(new vscode.CancellationError());
      });
      promise.then(
        (value) => {
          if (settled) {
            return;
          }
          settled = true;
          subscription.dispose();
          resolve(value);
        },
        (error) => {
          if (settled) {
            return;
          }
          settled = true;
          subscription.dispose();
          reject(error);
        }
      );
    });
  }

  invalidateModelCache(uri?: string | vscode.Uri): void {
    if (!uri) {
      this.modelResultCache.clear();
      this.inFlightModelRequests.clear();
      return;
    }
    const target = typeof uri === "string" ? uri.trim() : uri.toString();
    for (const [key, entry] of this.modelResultCache.entries()) {
      if (entry.uri === target) {
        this.modelResultCache.delete(key);
      }
    }
    for (const [key, request] of this.inFlightModelRequests.entries()) {
      if (request.uri === target) {
        this.inFlightModelRequests.delete(key);
      }
    }
  }

  clearModelCache(): void {
    this.modelResultCache.clear();
    this.inFlightModelRequests.clear();
  }

  async getModel(
    uri: string,
    scopes?: SysMLModelParams["scope"],
    token?: vscode.CancellationToken,
    caller = "unknown"
  ): Promise<SysMLModelResult> {
    const totalStartedAt = Date.now();
    const trimmed = (uri || "").trim();
    const normalizedScopes = normalizeScopes(scopes);
    const requestId = this.nextRequestId();
    if (!trimmed) {
      log("getModel: empty URI, returning empty model");
      return {
        version: 0,
        graph: { nodes: [], edges: [] },
      };
    }
    log("getModel: uri (full)=", trimmed, "scopes:", normalizedScopes);
    const readyWaitStartedAt = Date.now();
    await this.whenReady;
    const readyWaitMs = Date.now() - readyWaitStartedAt;
    const cached = this.findReusableCacheEntry(trimmed, normalizedScopes);
    if (cached) {
      logPerf(
        cached.reuseType === "exact"
          ? "lspModelProvider:getModelCacheHit"
          : "lspModelProvider:getModelSupersetCacheHit",
        {
          requestId,
          caller,
          uri: trimmed,
          scopes: normalizedScopes,
          cacheScopes: cached.entry.scopes,
          readyWaitMs,
          cacheAgeMs: Date.now() - cached.entry.storedAt,
          totalMs: Date.now() - totalStartedAt,
          nodeCount: cached.entry.result.graph?.nodes?.length ?? 0,
          edgeCount: cached.entry.result.graph?.edges?.length ?? 0,
        }
      );
      return cached.entry.result;
    }
    const joinedRequest = this.findReusableInFlightRequest(trimmed, normalizedScopes);
    if (joinedRequest) {
      const result = await this.awaitWithCancellation(
        joinedRequest.request.promise,
        token
      );
      logPerf(
        joinedRequest.reuseType === "exact"
          ? "lspModelProvider:getModelInFlightJoin"
          : "lspModelProvider:getModelSupersetInFlightJoin",
        {
          requestId,
          caller,
          joinedRequestId: joinedRequest.request.requestId,
          joinedCaller: joinedRequest.request.caller,
          uri: trimmed,
          scopes: normalizedScopes,
          joinedScopes: joinedRequest.request.scopes,
          readyWaitMs,
          totalMs: Date.now() - totalStartedAt,
          nodeCount: result.graph?.nodes?.length ?? 0,
          edgeCount: result.graph?.edges?.length ?? 0,
        }
      );
      return result;
    }
    const params: SysMLModelParams = {
      textDocument: { uri: trimmed },
      scope: normalizedScopes.length > 0 ? normalizedScopes : undefined,
    };
    const requestKey = buildRequestKey(trimmed, normalizedScopes);
    const doRequest = () =>
      this.client.sendRequest<SysMLModelResult>("sysml/model", params);

    try {
      logPerf("lspModelProvider:getModelRequestStart", {
        requestId,
        caller,
        uri: trimmed,
        scopes: normalizedScopes,
        readyWaitMs,
        dedupe: "newRequest",
      });
      const requestStartedAt = Date.now();
      const sharedPromise = (async () => {
        let requestAttempts = 1;
        let result = await doRequest();
        let nodeCount = result.graph?.nodes?.length ?? 0;
        let edgeCount = result.graph?.edges?.length ?? 0;

        if (
          nodeCount === 0 &&
          edgeCount === 0 &&
          normalizedScopes.includes("graph")
        ) {
          log(
            "getModel: 0 nodes/edges for uri=",
            trimmed,
            ", retrying after 300ms"
          );
          await new Promise((r) => setTimeout(r, 300));
          requestAttempts += 1;
          result = await doRequest();
          nodeCount = result.graph?.nodes?.length ?? 0;
          edgeCount = result.graph?.edges?.length ?? 0;
        }

        this.storeModelResult(trimmed, normalizedScopes, result);
        const requestMs = Date.now() - requestStartedAt;
        const containsCount = result.graph?.edges?.filter(
          (e: { type?: string; rel_type?: string }) =>
            (e.type || e.rel_type || "").toLowerCase() === "contains"
        ).length ?? 0;
        log(
          "getModel result:",
          result.graph?.nodes?.length ?? 0,
          "nodes,",
          result.graph?.edges?.length ?? 0,
          "edges,",
          containsCount,
          "contains"
        );
        logPerf("lspModelProvider:getModel", {
          requestId,
          caller,
          uri: trimmed,
          scopes: normalizedScopes,
          readyWaitMs,
          requestMs,
          totalMs: Date.now() - totalStartedAt,
          requestAttempts,
          nodeCount: result.graph?.nodes?.length ?? 0,
          edgeCount: result.graph?.edges?.length ?? 0,
          containsCount,
          parseTimeMs: result.stats?.parseTimeMs,
          modelBuildTimeMs: result.stats?.modelBuildTimeMs,
          dedupe: "newRequest",
        });
        return result;
      })();
      const trackedPromise = sharedPromise.finally(() => {
        const current = this.inFlightModelRequests.get(requestKey);
        if (current?.requestId === requestId) {
          this.inFlightModelRequests.delete(requestKey);
        }
      });
      this.inFlightModelRequests.set(requestKey, {
        key: requestKey,
        uri: trimmed,
        scopes: normalizedScopes,
        requestId,
        caller,
        promise: trackedPromise,
      });
      const result = await this.awaitWithCancellation(trackedPromise, token);
      return result;
    } catch (e) {
      if (isCancellationError(e)) {
        log("getModel cancelled for uri=", trimmed);
        logPerf("lspModelProvider:getModelCancelled", {
          requestId,
          caller,
          uri: trimmed,
          scopes: normalizedScopes,
          readyWaitMs,
          totalMs: Date.now() - totalStartedAt,
        });
        throw e;
      }
      if (isClientNotRunningError(e)) {
        logError(`getModel failed because the language client is not running for ${trimmed}`, e);
        logPerf("lspModelProvider:getModelClientNotRunning", {
          requestId,
          caller,
          uri: trimmed,
          scopes: normalizedScopes,
          readyWaitMs,
          totalMs: Date.now() - totalStartedAt,
        });
        throw e;
      }
      logError("getModel failed", e);
      logPerf("lspModelProvider:getModelFailed", {
        requestId,
        caller,
        uri: trimmed,
        scopes: normalizedScopes,
        readyWaitMs,
        totalMs: Date.now() - totalStartedAt,
        error: e instanceof Error ? e.message : String(e),
      });
      throw e;
    }
  }

  async getDiagram(
    uri: string,
    kind: SysMLDiagramParams["kind"],
    options?: SysMLDiagramParams["options"],
    token?: vscode.CancellationToken
  ): Promise<SysMLDiagramResult> {
    const startedAt = Date.now();
    const trimmed = (uri || "").trim();
    if (!trimmed) {
      throw new Error("getDiagram requires a non-empty URI");
    }
    await this.whenReady;
    const params: SysMLDiagramParams = {
      textDocument: { uri: trimmed },
      kind,
      options,
    };
    const result = await this.client.sendRequest<SysMLDiagramResult>("sysml/diagram", params, token);
    logPerf("lspModelProvider:getDiagram", {
      uri: trimmed,
      kind,
      workspaceVisualization: options?.workspaceVisualization === true,
      totalMs: Date.now() - startedAt,
      nodeCount: result.scene?.generalView?.nodes?.length ?? 0,
      edgeCount: result.scene?.generalView?.edges?.length ?? 0,
    });
    return result;
  }

  async getServerStats(): Promise<SysMLServerStats | undefined> {
    try {
      return await this.client.sendRequest<SysMLServerStats>("sysml/serverStats");
    } catch (e) {
      log("getServerStats failed", e);
      return undefined;
    }
  }

  async clearCache(): Promise<SysMLClearCacheResult | undefined> {
    try {
      return await this.client.sendRequest<SysMLClearCacheResult>("sysml/clearCache");
    } catch (e) {
      log("clearCache failed", e);
      return undefined;
    }
  }

  async searchLibraries(
    query: string,
    limit = 100
  ): Promise<SysMLLibrarySearchResult> {
    return await this.client.sendRequest<SysMLLibrarySearchResult>(
      "sysml/librarySearch",
      { query, limit }
    );
  }

  /**
   * Find an element by name in the model. When elementQualifiedName is provided,
   * looks up by id directly (disambiguates package vs part def with same name).
   * Otherwise searches by name and optionally scopes by parentContext.
   */
  async findElement(
    uri: string,
    elementName: string,
    parentContext?: string,
    elementQualifiedName?: string,
    token?: vscode.CancellationToken
  ): Promise<SysMLElementDTO | undefined> {
    const result = await this.getModel(uri, ["graph"], token, "findElement");
    if (!result.graph?.nodes?.length) {
      return undefined;
    }
    const nodes = result.graph.nodes;

    if (elementQualifiedName) {
      const byId = nodes.find((n) => (n.id || "").toLowerCase() === elementQualifiedName.toLowerCase());
      if (byId) {
        log("findElement: found by id", elementQualifiedName);
        return graphNodeToElementDTO(byId, result.graph);
      }
      const matchingByName = nodes.filter((n) => (n.name || "").toLowerCase() === (elementName || "").toLowerCase());
      log(
        "findElement: no match for id",
        JSON.stringify(elementQualifiedName),
        "graph has",
        nodes.length,
        "nodes;",
        matchingByName.length,
        "with name",
        elementName,
        "-> ids:",
        matchingByName.slice(0, 5).map((n) => n.id)
      );
    }

    const byName = new Map<string, typeof nodes>();
    for (const n of nodes) {
      const key = (n.name || "").toLowerCase();
      if (!byName.has(key)) byName.set(key, []);
      byName.get(key)!.push(n);
    }
    const candidates = byName.get(elementName.toLowerCase()) ?? [];
    if (parentContext) {
      const parentKey = parentContext.toLowerCase();
      const parentIds = new Set(nodes.filter((n) => (n.name || "").toLowerCase() === parentKey || (n.id || "").toLowerCase() === parentKey).map((n) => n.id));
      const scoped = candidates.filter((c) => c.parentId && parentIds.has(c.parentId));
      if (scoped.length > 0) {
        return graphNodeToElementDTO(scoped[0], result.graph);
      }
    }
    if (candidates.length > 0) {
      return graphNodeToElementDTO(candidates[0], result.graph);
    }
    return undefined;
  }
}
