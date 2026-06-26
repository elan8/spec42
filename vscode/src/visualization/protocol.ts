import type {
    IbdDataDTO,
    InterconnectionSceneDTO,
    SysMLElementDTO,
    SysMLGraphDTO,
    VisualizationViewCandidateDTO,
} from '../providers/sysmlModelTypes';
import type { RenderOutcome } from './renderContract';

export interface PreparedViewPayload {
    title: string;
    view: string;
    nodes: unknown[];
    edges: unknown[];
    meta?: unknown;
}

export interface WebviewRenderIdentity {
    contentHash: string;
    currentView: string;
    selectedView?: string;
    selectedViewName?: string;
    emptyStateMessage?: string;
    preparedView?: {
        title: string;
        view: string;
        nodeCount: number;
        edgeCount: number;
    };
    graph?: {
        nodeCount: number;
        edgeCount: number;
    };
}

export interface VisualizerUpdateMessage {
    command: 'update';
    updateId?: string;
    renderIdentity?: WebviewRenderIdentity;
    modelReady?: boolean;
    modelStatusMessage?: string;
    graph?: SysMLGraphDTO;
    elements?: SysMLElementDTO[];
    generalViewGraph?: SysMLGraphDTO;
    ibd?: IbdDataDTO;
    interconnectionScene?: InterconnectionSceneDTO;
    preparedView?: PreparedViewPayload;
    activityDiagrams: unknown[];
    sequenceDiagrams: unknown[];
    currentView: string;
    viewCandidates?: VisualizationViewCandidateDTO[];
    selectedView?: string;
    selectedViewName?: string;
    emptyStateMessage?: string;
}

export type HostToWebviewMessage =
    | { command: 'showLoading'; message?: string }
    | { command: 'hideLoading' }
    | { command: 'modelNotReady'; message?: string }
    | VisualizerUpdateMessage
    | { command: 'changeView'; view?: string }
    | { command: 'selectPackage'; packageName?: string }
    | { command: 'setRequirementsVisibleForTest' }
    | { command: 'export'; format?: 'png' | 'svg' | string; scale?: number }
    | { command: 'highlightElement'; elementName?: string; skipCentering?: boolean }
    | { command: 'requestCurrentView' }
    | { command: 'exportDiagramForTest' };

export interface RenderCompleteMessage {
    command: 'renderComplete';
    updateId?: string;
    view?: string;
    dataHash?: string;
    outcome?: RenderOutcome;
    graphNodes?: number;
    hasExportableSvg?: boolean;
}

export type WebviewToHostMessage =
    | { command: 'webviewLog'; level?: string; args?: unknown[] }
    | { command: 'webviewPerf'; event?: string; data?: Record<string, unknown> }
    | {
        command: 'jumpToElement';
        elementName?: string;
        skipCentering?: boolean;
        parentContext?: string;
        elementQualifiedName?: string;
        elementUri?: string;
        elementRange?: { start: { line: number; character: number }; end: { line: number; character: number } };
    }
    | { command: 'renameElement'; oldName?: string; newName?: string }
    | { command: 'export'; format?: string; data?: unknown }
    | { command: 'viewChanged'; view?: string }
    | { command: 'viewSelectionChanged'; viewId?: string; rendererView?: string }
    | { command: 'clearSelectedView' }
    | { command: 'openExternal'; url?: string }
    | { command: 'currentViewResponse'; view?: string }
    | { command: 'webviewReady' }
    | RenderCompleteMessage
    | { command: 'testDiagramExported'; viewId?: string; svgString?: string };

export function isRecord(value: unknown): value is Record<string, unknown> {
    return typeof value === 'object' && value !== null;
}

export function isHostToWebviewMessage(value: unknown): value is HostToWebviewMessage {
    return isRecord(value) && typeof value.command === 'string';
}

export function isWebviewToHostMessage(value: unknown): value is WebviewToHostMessage {
    return isRecord(value) && typeof value.command === 'string';
}
