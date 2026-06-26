import type { LspModelProvider } from '../providers/lspModelProvider';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';
import type {
    SysMLElementDTO,
    SysMLGraphDTO,
    VisualizationViewCandidateDTO,
} from '../providers/sysmlModelTypes';
import type { VisualizerUpdateMessage } from './protocol';

export interface FetchModelParams {
    workspaceRootUri: string;
    lspModelProvider: LspModelProvider;
    currentView: string;
    selectedView?: string;
}

export type UpdateMessage = VisualizerUpdateMessage;

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
    const { workspaceRootUri, lspModelProvider, currentView, selectedView } = params;

    log(
        'fetchModelData:start',
        `workspaceRootUri=${workspaceRootUri}`,
        `currentView=${currentView}`,
        `selectedView=${selectedView ?? '(auto)'}`,
    );
    if (isVerboseLoggingEnabled()) {
        try {
            // eslint-disable-next-line no-console
            console.log(
                '[viz][fetchModelData:start]',
                JSON.stringify({
                    workspaceRootUri,
                    currentView,
                    selectedView: selectedView ?? null,
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
            selectedView,
        );
        const requestMs = Date.now() - requestStartedAt;

        const modelReady = result.modelReady !== false;
        const resultView = result.view ?? currentView;
        const msg: UpdateMessage = {
            command: 'update',
            modelReady,
            modelStatusMessage: modelReady
                ? undefined
                : result.emptyStateMessage ?? 'SysML model is not ready yet.',
            graph: result.graph ?? { nodes: [], edges: [] },
            elements: result.workspaceModel?.semantic,
            generalViewGraph: result.generalViewGraph ?? result.graph,
            preparedView: result.preparedView,
            activityDiagrams: result.activityDiagrams ?? [],
            sequenceDiagrams: result.sequenceDiagrams ?? [],
            currentView: resultView,
            viewCandidates: result.viewCandidates ?? [],
            selectedView: result.selectedView,
            selectedViewName: result.selectedViewName,
            emptyStateMessage: result.emptyStateMessage,
        };

        log(
            'fetchModelData:done',
            `graphNodes=${msg.graph?.nodes?.length || 0}`,
            `graphEdges=${msg.graph?.edges?.length || 0}`,
            `viewCandidates=${msg.viewCandidates?.length || 0}`,
            `selectedView=${msg.selectedViewName ?? '(auto)'}`,
        );
        logPerfEvent('visualizer:fetchModelData', {
            currentView,
            workspaceRootUri,
            selectedView: selectedView ?? null,
            requestMs,
            totalMs: Date.now() - startedAt,
            graphNodes: msg.graph?.nodes?.length || 0,
            graphEdges: msg.graph?.edges?.length || 0,
            viewCandidates: msg.viewCandidates?.length || 0,
        });
        return msg;
    } catch (error) {
        logError('fetchModelData failed', error);
        logPerfEvent('visualizer:fetchModelDataFailed', {
            currentView,
            workspaceRootUri,
            selectedView: selectedView ?? null,
            totalMs: Date.now() - startedAt,
            error: error instanceof Error ? error.message : String(error),
        });
        return null;
    }
}
