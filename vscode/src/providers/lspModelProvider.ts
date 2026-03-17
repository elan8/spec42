import * as vscode from "vscode";
import type { LanguageClient } from "vscode-languageclient/node";
import { log, logError } from "../logger";
import type {
  GraphNodeDTO,
  SysMLGraphDTO,
  PositionDTO,
  SysMLElementDTO,
  SysMLModelParams,
  SysMLModelResult,
} from "./sysmlModelTypes";

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
    return { type: node.type, name: node.name, range: node.range, children: [], attributes: node.attributes || {}, relationships: [] };
  }
  visited.add(node.id);
  const children = (graph.nodes || []).filter((n) => n.parentId === node.id);
  const childDTOs = children.map((c) => graphNodeToElementDTO(c, graph, visited));
  const edgeType = (e: { type?: string; rel_type?: string }) => e.type || e.rel_type || '';
  const relationships = (graph.edges || [])
    .filter((e) => e.source === node.id && edgeType(e).toLowerCase() !== 'contains')
    .map((e) => ({ source: e.source, target: e.target, type: edgeType(e), name: e.name }));
  return {
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

export class LspModelProvider {
  constructor(
    private readonly client: LanguageClient,
    /** Resolves when the LSP client is ready. Prevents getModel before didOpen is processed. */
    private readonly whenReady: Promise<void> = Promise.resolve()
  ) {}

  async getModel(
    uri: string,
    scopes?: SysMLModelParams["scope"],
    token?: vscode.CancellationToken
  ): Promise<SysMLModelResult> {
    const trimmed = (uri || "").trim();
    if (!trimmed) {
      log("getModel: empty URI, returning empty model");
      return {
        version: 0,
        graph: { nodes: [], edges: [] },
      };
    }
    log("getModel: uri (full)=", trimmed, "scopes:", scopes);
    await this.whenReady;
    const params: SysMLModelParams = {
      textDocument: { uri: trimmed },
      scope: scopes,
    };
    const doRequest = () =>
      this.client.sendRequest<SysMLModelResult>("sysml/model", params, token);

    try {
      let result = await doRequest();
      const nodeCount = result.graph?.nodes?.length ?? 0;
      const edgeCount = result.graph?.edges?.length ?? 0;

      // Retry once if empty: server may not have processed didOpen/workspace scan yet
      if (nodeCount === 0 && edgeCount === 0 && scopes?.includes("graph")) {
        log("getModel: 0 nodes/edges for uri=", trimmed, ", retrying after 300ms");
        await new Promise((r) => setTimeout(r, 300));
        result = await doRequest();
      }

      const containsCount = result.graph?.edges?.filter(
        (e: { type?: string; rel_type?: string }) => (e.type || e.rel_type || "").toLowerCase() === "contains"
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
      return result;
    } catch (e) {
      if (isCancellationError(e)) {
        log("getModel cancelled for uri=", trimmed);
        throw e;
      }
      if (isClientNotRunningError(e)) {
        logError(`getModel failed because the language client is not running for ${trimmed}`, e);
        throw e;
      }
      logError("getModel failed", e);
      throw e;
    }
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
    const result = await this.getModel(uri, ["graph"], token);
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
