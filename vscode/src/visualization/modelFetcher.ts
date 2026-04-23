import type { LspModelProvider } from '../providers/lspModelProvider';
import { isVerboseLoggingEnabled, log, logError, logPerfEvent } from '../logger';
import type {
    IbdDataDTO,
    SoftwareWorkspaceModelDTO,
    SoftwareVisualizationViewCandidateDTO,
    SysMLElementDTO,
    SysMLGraphDTO,
    VisualizationViewCandidateDTO,
} from '../providers/sysmlModelTypes';
import { projectSoftwareWorkspaceModel } from './softwareViewProjection';

export interface FetchModelParams {
    workspaceRootUri: string;
    lspModelProvider: LspModelProvider;
    currentView: string;
    selectedView?: string;
}

export interface UpdateMessage {
    command: 'update';
    graph?: SysMLGraphDTO;
    elements?: SysMLElementDTO[];
    generalViewGraph?: SysMLGraphDTO;
    ibd?: IbdDataDTO;
    activityDiagrams: unknown[];
    currentView: string;
    viewCandidates?: VisualizationViewCandidateDTO[];
    selectedView?: string;
    selectedViewName?: string;
    emptyStateMessage?: string;
}

function toVisualizationCandidates(
    candidates: SoftwareVisualizationViewCandidateDTO[]
): VisualizationViewCandidateDTO[] {
    return candidates.map((candidate) => ({
        id: candidate.id,
        name: candidate.name,
        supported: candidate.supported,
        description: candidate.description,
        rendererView: candidate.id,
        viewType: 'SoftwareView',
    }));
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

        const msg: UpdateMessage = {
            command: 'update',
            graph: result.graph ?? { nodes: [], edges: [] },
            elements: result.workspaceModel?.semantic,
            generalViewGraph: result.generalViewGraph ?? result.graph,
            ibd: result.ibd,
            activityDiagrams: result.activityDiagrams ?? [],
            currentView: result.view ?? currentView,
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

export async function fetchSoftwareModelData(params: Omit<FetchModelParams, 'selectedView'>): Promise<UpdateMessage | null> {
    const startedAt = Date.now();
    const { workspaceRootUri, lspModelProvider, currentView } = params;

    log(
        'fetchSoftwareModelData:start',
        `workspaceRootUri=${workspaceRootUri}`,
        `currentView=${currentView}`,
    );

    try {
        const requestStartedAt = Date.now();
        const result = await lspModelProvider.getSoftwareVisualization(
            workspaceRootUri,
            currentView,
        );
        const requestMs = Date.now() - requestStartedAt;

        const msg: UpdateMessage = {
            command: 'update',
            graph: result.graph ?? { nodes: [], edges: [] },
            elements: result.workspaceModel?.semantic,
            generalViewGraph: result.graph ?? { nodes: [], edges: [] },
            ibd: undefined,
            activityDiagrams: [],
            currentView: result.view ?? currentView,
            viewCandidates: toVisualizationCandidates(result.views ?? []),
            selectedView: result.view,
            selectedViewName: result.views?.find((candidate) => candidate.id === result.view)?.name,
            emptyStateMessage: result.emptyStateMessage,
        };

        logPerfEvent('softwareVisualizer:fetchModelData', {
            currentView,
            workspaceRootUri,
            requestMs,
            totalMs: Date.now() - startedAt,
            graphNodes: msg.graph?.nodes?.length || 0,
            graphEdges: msg.graph?.edges?.length || 0,
            viewCandidates: msg.viewCandidates?.length || 0,
        });
        return msg;
    } catch (error) {
        logError('fetchSoftwareModelData failed', error);
        logPerfEvent('softwareVisualizer:fetchModelDataFailed', {
            currentView,
            workspaceRootUri,
            totalMs: Date.now() - startedAt,
            error: error instanceof Error ? error.message : String(error),
        });
        return null;
    }
}

export function buildSoftwareUpdateMessage(
    workspaceRootUri: string,
    currentView: string,
    model?: SoftwareWorkspaceModelDTO,
): UpdateMessage {
    if (!model) {
        return {
            command: 'update',
            graph: { nodes: [], edges: [] },
            elements: [],
            generalViewGraph: { nodes: [], edges: [] },
            ibd: undefined,
            activityDiagrams: [],
            currentView,
            viewCandidates: toVisualizationCandidates([
                { id: 'software-module-view', name: 'Rust Module View', supported: true, description: 'Shows crates and modules.' },
                { id: 'software-dependency-view', name: 'Rust Dependency View', supported: true, description: 'Shows module dependencies.' },
            ]),
            selectedView: currentView,
            selectedViewName: currentView === 'software-dependency-view' ? 'Rust Dependency View' : 'Rust Module View',
            emptyStateMessage: 'Run analysis from the Spec42 Add-ons view before opening the software architecture visualizer.',
        };
    }

    const projection = projectSoftwareWorkspaceModel(
        model,
        currentView === 'software-dependency-view' ? 'software-dependency-view' : 'software-module-view',
    );
    return {
        command: 'update',
        graph: projection.graph,
        elements: projection.workspaceModel.semantic,
        generalViewGraph: projection.graph,
        ibd: undefined,
        activityDiagrams: [],
        currentView,
        viewCandidates: toVisualizationCandidates([
            { id: 'software-module-view', name: 'Rust Module View', supported: true, description: 'Shows crates and modules.' },
            { id: 'software-dependency-view', name: 'Rust Dependency View', supported: true, description: 'Shows module dependencies.' },
        ]),
        selectedView: currentView,
        selectedViewName: currentView === 'software-dependency-view' ? 'Rust Dependency View' : 'Rust Module View',
        emptyStateMessage: undefined,
    };
}
