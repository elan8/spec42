import type { LspModelProvider } from '../providers/lspModelProvider';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';
import type {
    IbdDataDTO,
    SysMLElementDTO,
    SysMLGraphDTO,
    VisualizationPackageCandidateDTO,
} from '../providers/sysmlModelTypes';

export interface FetchModelParams {
    workspaceRootUri: string;
    lspModelProvider: LspModelProvider;
    currentView: string;
    selectedPackage?: string;
}

export interface UpdateMessage {
    command: 'update';
    graph?: SysMLGraphDTO;
    elements?: SysMLElementDTO[];
    generalViewGraph?: SysMLGraphDTO;
    ibd?: IbdDataDTO;
    activityDiagrams: unknown[];
    currentView: string;
    packageCandidates?: VisualizationPackageCandidateDTO[];
    selectedPackage?: string;
    selectedPackageName?: string;
}

export function hashContent(content: string): string {
    let hash = 0;
    for (let i = 0; i < content.length; i++) {
        const char = content.charCodeAt(i);
        hash = ((hash << 5) - hash) + char;
        hash = hash & hash;
    }
    return hash.toString(16);
}

export async function fetchModelData(params: FetchModelParams): Promise<UpdateMessage | null> {
    const startedAt = Date.now();
    const { workspaceRootUri, lspModelProvider, currentView, selectedPackage } = params;

    log(
        'fetchModelData:start',
        `workspaceRootUri=${workspaceRootUri}`,
        `currentView=${currentView}`,
        `selectedPackage=${selectedPackage ?? '(all)'}`,
    );
    if (isVerboseLoggingEnabled()) {
        try {
            // eslint-disable-next-line no-console
            console.log(
                '[viz][fetchModelData:start]',
                JSON.stringify({
                    workspaceRootUri,
                    currentView,
                    selectedPackage: selectedPackage ?? null,
                })
            );
        } catch {
            // ignore
        }
    }

    try {
        const requestStartedAt = Date.now();
        const result = await lspModelProvider.getVisualization(
            workspaceRootUri,
            currentView,
            selectedPackage
                ? { kind: 'package', package: selectedPackage }
                : { kind: 'all' },
        );
        const requestMs = Date.now() - requestStartedAt;

        const msg: UpdateMessage = {
            command: 'update',
            graph: result.graph ?? { nodes: [], edges: [] },
            elements: result.workspaceModel?.semantic,
            generalViewGraph: result.generalViewGraph ?? result.graph,
            ibd: result.ibd,
            activityDiagrams: result.activityDiagrams ?? [],
            currentView,
            packageCandidates: result.packageCandidates ?? [],
            selectedPackage: result.selectedPackage,
            selectedPackageName: result.selectedPackageName,
        };

        log(
            'fetchModelData:done',
            `graphNodes=${msg.graph?.nodes?.length || 0}`,
            `graphEdges=${msg.graph?.edges?.length || 0}`,
            `packageCandidates=${msg.packageCandidates?.length || 0}`,
            `selectedPackage=${msg.selectedPackageName ?? '(all)'}`,
        );
        logPerfEvent('visualizer:fetchModelData', {
            currentView,
            workspaceRootUri,
            selectedPackage: selectedPackage ?? null,
            requestMs,
            totalMs: Date.now() - startedAt,
            graphNodes: msg.graph?.nodes?.length || 0,
            graphEdges: msg.graph?.edges?.length || 0,
            packageCandidates: msg.packageCandidates?.length || 0,
        });
        return msg;
    } catch (error) {
        logError('fetchModelData failed', error);
        logPerfEvent('visualizer:fetchModelDataFailed', {
            currentView,
            workspaceRootUri,
            selectedPackage: selectedPackage ?? null,
            totalMs: Date.now() - startedAt,
            error: error instanceof Error ? error.message : String(error),
        });
        return null;
    }
}
