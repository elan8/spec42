import * as vscode from 'vscode';
import type { LspModelProvider } from '../providers/lspModelProvider';
import { isVerboseLoggingEnabled, log, logError } from '../logger';
import type {
    IbdDataDTO,
    SysMLModelResult,
    SysMLDiagramResult,
    SysMLGraphDTO,
    GraphNodeDTO,
    GraphEdgeDTO,
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
    diagramGeneral?: SysMLDiagramResult;
    diagramInterconnection?: SysMLDiagramResult;
    ibd?: IbdDataDTO;
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

export function mergeOptionalGraphs(graphs: Array<SysMLGraphDTO | undefined>): SysMLGraphDTO | undefined {
    const present = graphs.filter((graph): graph is SysMLGraphDTO => Boolean(graph));
    if (present.length === 0) {
        return undefined;
    }
    return mergeGraphs(present);
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

    const scopes: ('graph' | 'activityDiagrams' | 'stats' | 'ibd')[] =
        currentView === 'action-flow-view'
            ? ['graph', 'activityDiagrams', 'stats']
            : currentView === 'interconnection-view'
                ? ['graph', 'ibd', 'stats']
                : ['graph', 'stats'];
    const isWorkspaceVisualization = fileUris.length > 1;
    const requestScopes = isWorkspaceVisualization
        ? [...scopes, 'workspaceVisualization' as const]
        : scopes;
    const requestUris = fileUris.length > 0
        ? fileUris.map(u => u.toString())
        : [documentUri];
    log(
        'fetchModelData:start',
        `workspace=${isWorkspaceVisualization}`,
        `uris=${requestUris.length}`,
        `scopes=${requestScopes.join(',')}`,
        `currentView=${currentView}`,
        `pendingPackage=${pendingPackageName ?? '(none)'}`,
    );
    if (isVerboseLoggingEnabled()) {
        try {
            // eslint-disable-next-line no-console
            console.log(
                '[viz][fetchModelData:start]',
                JSON.stringify({
                    workspace: isWorkspaceVisualization,
                    uris: requestUris.length,
                    scopes: requestScopes,
                    currentView,
                    pendingPackage: pendingPackageName ?? null,
                })
            );
        } catch {
            // ignore
        }
    }

    const settledResults = await Promise.allSettled(
        requestUris.map(uri =>
            lspModelProvider.getModel(
                uri,
                requestScopes,
                undefined,
                `visualizer.fetchModelData:${currentView}`
            )
        ),
    );
    const [generalDiagramResult, interconnectionDiagramResult] = await Promise.allSettled([
        lspModelProvider.getDiagram(documentUri, 'general-view', {
            workspaceVisualization: isWorkspaceVisualization,
        }),
        lspModelProvider.getDiagram(documentUri, 'interconnection-view', {
            workspaceVisualization: isWorkspaceVisualization,
        }),
    ]);

    const results = settledResults
        .filter((result): result is PromiseFulfilledResult<SysMLModelResult> => result.status === 'fulfilled')
        .map((result) => result.value);
    const failures = settledResults.filter(
        (result): result is PromiseRejectedResult => result.status === 'rejected',
    );
    if (generalDiagramResult.status === 'rejected') {
        logError('fetchModelData: general-view diagram request failed', generalDiagramResult.reason);
    }
    if (interconnectionDiagramResult.status === 'rejected') {
        logError('fetchModelData: interconnection-view diagram request failed', interconnectionDiagramResult.reason);
    }

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
    const allGeneralViewGraphs: SysMLGraphDTO[] = [];
    const allActivityDiagrams: unknown[] = [];

    for (const result of results) {
        if (result.graph?.nodes?.length || result.graph?.edges?.length) {
            allGraphs.push(result.graph);
        }
        if (result.generalViewGraph?.nodes?.length || result.generalViewGraph?.edges?.length) {
            allGeneralViewGraphs.push(result.generalViewGraph);
        }
        if (result.activityDiagrams) allActivityDiagrams.push(...result.activityDiagrams);
    }

    const mergedGraph = mergeGraphs(allGraphs);
    const mergedGeneralViewGraph = mergeOptionalGraphs(allGeneralViewGraphs);
    const mergedPackageNames = (mergedGraph.nodes || [])
        .filter((n) => ((n.type || '') as string).toLowerCase().includes('package'))
        .map((n) => n.name)
        .filter((name): name is string => typeof name === 'string' && name.length > 0);
    const uniqueMergedPackageNames = [...new Set(mergedPackageNames)];

    const primaryResult = results.find(r => r.graph?.nodes?.length || r.graph?.edges?.length) ?? results[0];
    const generalDiagram = generalDiagramResult.status === 'fulfilled' ? generalDiagramResult.value : undefined;
    const interconnectionDiagram = interconnectionDiagramResult.status === 'fulfilled' ? interconnectionDiagramResult.value : undefined;
    const msg: UpdateMessage = {
        command: 'update',
        graph: mergedGraph,
        generalViewGraph: mergedGeneralViewGraph ?? primaryResult?.generalViewGraph,
        diagramGeneral: generalDiagram,
        diagramInterconnection: interconnectionDiagram,
        ibd: primaryResult?.ibd,
        activityDiagrams: allActivityDiagrams,
        currentView,
    };
    if (pendingPackageName) {
        msg.pendingPackageName = pendingPackageName;
    }
    log(
        'fetchModelData:done',
        `results=${results.length}`,
        `graphs=${allGraphs.length}`,
        `mergedNodes=${mergedGraph.nodes?.length || 0}`,
        `mergedEdges=${mergedGraph.edges?.length || 0}`,
        `mergedPackages=${uniqueMergedPackageNames.join('|') || '(none)'}`,
        `generalSceneNodes=${generalDiagram?.scene?.generalView?.nodes?.length || 0}`,
        `generalSceneEdges=${generalDiagram?.scene?.generalView?.edges?.length || 0}`,
    );
    if (isVerboseLoggingEnabled()) {
        try {
            // eslint-disable-next-line no-console
            console.log(
                '[viz][fetchModelData:done]',
                JSON.stringify({
                    results: results.length,
                    graphs: allGraphs.length,
                    mergedNodes: mergedGraph.nodes?.length || 0,
                    mergedEdges: mergedGraph.edges?.length || 0,
                    mergedPackages: uniqueMergedPackageNames,
                    generalSceneNodes: generalDiagram?.scene?.generalView?.nodes?.length || 0,
                    generalSceneEdges: generalDiagram?.scene?.generalView?.edges?.length || 0,
                })
            );
        } catch {
            // ignore
        }
    }
    return msg;
}
