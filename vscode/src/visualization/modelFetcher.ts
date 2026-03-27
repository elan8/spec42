import * as vscode from 'vscode';
import type { LspModelProvider } from '../providers/lspModelProvider';
import { log, logError } from '../logger';
import type {
    SysMLGraphDTO,
    GraphNodeDTO,
    GraphEdgeDTO,
    IbdDataDTO,
    RenderedDiagramsDTO,
} from '../providers/sysmlModelTypes';

export interface FetchModelParams {
    documentUri: string;
    fileUris: vscode.Uri[];
    lspModelProvider: LspModelProvider;
    currentView: string;
    pendingPackageName?: string;
}

export interface UpdateMessage {
    command: 'update';
    graph?: SysMLGraphDTO;
    generalViewGraph?: SysMLGraphDTO;
    ibd?: IbdDataDTO;
    renderedDiagrams?: RenderedDiagramsDTO;
    sequenceDiagrams: unknown[];
    activityDiagrams: unknown[];
    currentView: string;
    pendingPackageName?: string;
}

/**
 * Hash content for change detection. Used to skip re-parsing when document
 * content has not changed.
 */
export function hashContent(content: string): string {
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
        const char = content.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash;
    }
    return hash.toString(16);
}

/**
 * Merge graphs from multiple files. Nodes with same id (qualified name) are merged;
 * packages merge attributes and children; edges are deduplicated.
 */
export function mergeGraphs(graphs: SysMLGraphDTO[]): SysMLGraphDTO {
    const nodeMap = new Map<string, GraphNodeDTO>();
    const edgeKeys = new Set<string>();
    const edges: GraphEdgeDTO[] = [];

    for (const g of graphs) {
        for (const node of g.nodes ?? []) {
            const existing = nodeMap.get(node.id);
            if (existing && node.type === 'package') {
                existing.attributes = { ...(existing.attributes ?? {}), ...(node.attributes ?? {}) };
            } else if (!existing) {
                nodeMap.set(node.id, { ...node, attributes: { ...(node.attributes ?? {}) } });
            }
        }
        for (const edge of g.edges ?? []) {
            const edgeType = edge.type || edge.rel_type || '';
            const key = `${edgeType}::${edge.source}::${edge.target}`;
            if (!edgeKeys.has(key)) {
                edgeKeys.add(key);
                edges.push(edge);
            }
        }
    }

    return {
        nodes: Array.from(nodeMap.values()),
        edges,
    };
}

/**
 * Fetch model data from the LSP provider and convert it to the webview update message format.
 */
export async function fetchModelData(params: FetchModelParams): Promise<UpdateMessage | null> {
    const {
        documentUri,
        fileUris,
        lspModelProvider,
        currentView,
        pendingPackageName,
    } = params;

    const scopes: ('graph' | 'ibd' | 'sequenceDiagrams' | 'activityDiagrams' | 'stats')[] =
        ['graph', 'ibd', 'sequenceDiagrams', 'activityDiagrams', 'stats'];
    const isWorkspaceVisualization = fileUris.length > 1;
    const requestScopes = isWorkspaceVisualization
        ? [...scopes, 'workspaceVisualization' as const]
        : scopes;
    const requestUris = isWorkspaceVisualization
        ? [documentUri]
        : (fileUris.length > 0 ? fileUris.map(u => u.toString()) : [documentUri]);

    const settledResults = await Promise.allSettled(
        requestUris.map(uri => lspModelProvider.getModel(uri, requestScopes)),
    );

    const results = settledResults
        .filter((result): result is PromiseFulfilledResult<Awaited<ReturnType<LspModelProvider["getModel"]>>> => result.status === 'fulfilled')
        .map((result) => result.value);
    const failures = settledResults.filter(
        (result): result is PromiseRejectedResult => result.status === 'rejected',
    );

    if (failures.length > 0) {
        for (const failure of failures) {
            logError('fetchModelData: getModel failed for one of the requested URIs', failure.reason);
        }
        log(
            'fetchModelData: partial model fetch',
            `${results.length} succeeded`,
            `${failures.length} failed`,
        );
    }

    if (results.length === 0) {
        log('fetchModelData: no successful model responses, returning null');
        return null;
    }

    const allGraphs: SysMLGraphDTO[] = [];
    const allSequenceDiagrams: unknown[] = [];
    const allActivityDiagrams: unknown[] = [];

    for (const result of results) {
        if (result.graph?.nodes?.length || result.graph?.edges?.length) {
            allGraphs.push(result.graph);
        }
        if (result.sequenceDiagrams) allSequenceDiagrams.push(...result.sequenceDiagrams);
        if (result.activityDiagrams) allActivityDiagrams.push(...result.activityDiagrams);
    }

    const mergedGraph = mergeGraphs(allGraphs);

    const primaryResult = results.find(r => r.graph?.nodes?.length || r.graph?.edges?.length) ?? results[0];
    const primaryGeneralViewGraph = primaryResult?.generalViewGraph;
    const ibd = primaryResult?.ibd;
    // renderedDiagrams are intentionally not requested by default (backend SVG/layout is expensive).
    const renderedDiagrams = requestUris.length === 1 ? primaryResult?.renderedDiagrams : undefined;

    const msg: UpdateMessage = {
        command: 'update',
        graph: mergedGraph,
        generalViewGraph: primaryGeneralViewGraph,
        ibd,
        renderedDiagrams,
        sequenceDiagrams: allSequenceDiagrams,
        activityDiagrams: allActivityDiagrams,
        currentView,
    };
    if (pendingPackageName) {
        msg.pendingPackageName = pendingPackageName;
    }
    return msg;
}
