import * as path from 'path';
import * as vscode from 'vscode';
import type { LspModelProvider } from '../providers/lspModelProvider';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';
import type {
    IbdDataDTO,
    SysMLElementDTO,
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
    elements?: SysMLElementDTO[];
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

type DiagramFetchResult = {
    generalDiagram?: SysMLDiagramResult;
    interconnectionDiagram?: SysMLDiagramResult;
};

function getWorkspaceModelRequestUri(fileUris: vscode.Uri[], documentUri: string): string {
    if (fileUris.length === 0) {
        return documentUri;
    }

    const directoryPaths = fileUris
        .map((uri) => uri.fsPath)
        .filter((fsPath): fsPath is string => typeof fsPath === 'string' && fsPath.length > 0)
        .map((fsPath) => path.dirname(path.resolve(fsPath)));

    if (directoryPaths.length === 0) {
        return fileUris[0].toString();
    }

    const roots = directoryPaths.map((dirPath) => path.parse(dirPath).root.toLowerCase());
    if (new Set(roots).size !== 1) {
        return fileUris[0].toString();
    }

    const segmentLists = directoryPaths.map((dirPath) =>
        dirPath
            .slice(path.parse(dirPath).root.length)
            .split(path.sep)
            .filter((segment) => segment.length > 0)
    );

    const commonSegments: string[] = [];
    const shortestLength = Math.min(...segmentLists.map((segments) => segments.length));
    for (let index = 0; index < shortestLength; index += 1) {
        const candidate = segmentLists[0][index];
        const matches = segmentLists.every((segments) =>
            (process.platform === 'win32'
                ? segments[index].toLowerCase() === candidate.toLowerCase()
                : segments[index] === candidate)
        );
        if (!matches) {
            break;
        }
        commonSegments.push(candidate);
    }

    const root = path.parse(directoryPaths[0]).root;
    const commonDirectory = commonSegments.length > 0
        ? path.join(root, ...commonSegments)
        : root;
    return vscode.Uri.file(commonDirectory).toString();
}

async function fetchDiagramsForCurrentView(
    lspModelProvider: LspModelProvider,
    documentUri: string,
    currentView: string,
    isWorkspaceVisualization: boolean,
): Promise<DiagramFetchResult> {
    if (currentView === 'general-view') {
        return {
            generalDiagram: await lspModelProvider.getDiagram(documentUri, 'general-view', {
                workspaceVisualization: isWorkspaceVisualization,
            }),
        };
    }

    if (currentView === 'interconnection-view') {
        return {
            interconnectionDiagram: await lspModelProvider.getDiagram(documentUri, 'interconnection-view', {
                workspaceVisualization: isWorkspaceVisualization,
            }),
        };
    }

    return {};
}

/**
 * Fetch model data from the LSP provider and convert it to the webview update message format.
 */
export async function fetchModelData(params: FetchModelParams): Promise<UpdateMessage | null> {
    const startedAt = Date.now();
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
    const diagramRequestUri = fileUris.length > 0
        ? fileUris[0].toString()
        : documentUri;
    const modelRequestUri = isWorkspaceVisualization
        ? getWorkspaceModelRequestUri(fileUris, documentUri)
        : diagramRequestUri;
    log(
        'fetchModelData:start',
        `workspace=${isWorkspaceVisualization}`,
        `diagramUri=${diagramRequestUri}`,
        `modelUri=${modelRequestUri}`,
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
                    diagramUri: diagramRequestUri,
                    modelUri: modelRequestUri,
                    scopes: requestScopes,
                    currentView,
                    pendingPackage: pendingPackageName ?? null,
                })
            );
        } catch {
            // ignore
        }
    }

    const modelRequestsStartedAt = Date.now();
    const settledResults = await Promise.allSettled([
        lspModelProvider.getModel(
            modelRequestUri,
            requestScopes,
            undefined,
            `visualizer.fetchModelData:${currentView}`
        ),
    ]);
    const modelRequestsMs = Date.now() - modelRequestsStartedAt;
    const diagramRequestsStartedAt = Date.now();
    const diagramFetchResult = await Promise.allSettled([
        fetchDiagramsForCurrentView(
            lspModelProvider,
            diagramRequestUri,
            currentView,
            isWorkspaceVisualization,
        ),
    ]);
    const diagramRequestsMs = Date.now() - diagramRequestsStartedAt;

    const results = settledResults
        .filter((result): result is PromiseFulfilledResult<SysMLModelResult> => result.status === 'fulfilled')
        .map((result) => result.value);
    const failures = settledResults.filter(
        (result): result is PromiseRejectedResult => result.status === 'rejected',
    );
    const diagramFailure = diagramFetchResult.find(
        (result): result is PromiseRejectedResult => result.status === 'rejected',
    );
    if (diagramFailure) {
        logError(`fetchModelData: ${currentView} diagram request failed`, diagramFailure.reason);
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
        logPerfEvent('visualizer:fetchModelData', {
            currentView,
            workspaceVisualization: isWorkspaceVisualization,
            requestUriCount: 1,
            requestScopes,
            results: 0,
            failures: failures.length,
            modelRequestsMs,
            diagramRequestsMs,
            totalMs: Date.now() - startedAt,
        });
        return null;
    }

    const primaryResult = results[0];
    const mergedGraph = primaryResult.graph ?? { nodes: [], edges: [] };
    const mergedGeneralViewGraph = primaryResult.generalViewGraph;
    const allActivityDiagrams = primaryResult.activityDiagrams ?? [];
    const mergedPackageNames = (mergedGraph.nodes || [])
        .filter((n) => ((n.type || '') as string).toLowerCase().includes('package'))
        .map((n) => n.name)
        .filter((name): name is string => typeof name === 'string' && name.length > 0);
    const uniqueMergedPackageNames = [...new Set(mergedPackageNames)];
    const diagrams = diagramFetchResult[0]?.status === 'fulfilled' ? diagramFetchResult[0].value : {};
    const generalDiagram = diagrams.generalDiagram;
    const interconnectionDiagram = diagrams.interconnectionDiagram;
    const msg: UpdateMessage = {
        command: 'update',
        graph: mergedGraph,
        elements: primaryResult.workspaceModel?.semantic,
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
        `graphs=${primaryResult.graph ? 1 : 0}`,
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
                    graphs: primaryResult.graph ? 1 : 0,
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
    logPerfEvent('visualizer:fetchModelData', {
        currentView,
        workspaceVisualization: isWorkspaceVisualization,
        requestUriCount: 1,
        requestScopes,
        results: results.length,
        failures: failures.length,
        graphCount: primaryResult.graph ? 1 : 0,
        mergedNodes: mergedGraph.nodes?.length || 0,
        mergedEdges: mergedGraph.edges?.length || 0,
        generalDiagramNodes: generalDiagram?.scene?.generalView?.nodes?.length || 0,
        generalDiagramEdges: generalDiagram?.scene?.generalView?.edges?.length || 0,
        interconnectionDiagramNodes: interconnectionDiagram?.scene?.generalView?.nodes?.length || 0,
        interconnectionDiagramEdges: interconnectionDiagram?.scene?.generalView?.edges?.length || 0,
        modelRequestsMs,
        diagramRequestsMs,
        totalMs: Date.now() - startedAt,
    });
    return msg;
}
