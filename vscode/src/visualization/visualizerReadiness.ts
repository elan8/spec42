import {
    getLifecycleMessage,
    getWorkspaceLifecycle,
} from '../activation/workspaceLifecycle';
import { getLastVisualizerRender } from './renderTracker';
import {
    evaluateClientVisualizationReadiness,
    type VisualizationReadiness,
} from './visualizationGate';

export type VisualizerPhase =
    | 'blockedServer'
    | 'blockedWorkspace'
    | 'fetching'
    | 'settledEmpty'
    | 'settledDiagram'
    | 'error';

export type VisualizerReadinessSnapshot = {
    phase: VisualizerPhase;
    fetchAllowed: boolean;
    loadingMessage?: string;
    suppressNotReadyFlash: boolean;
    suppressLoadingFlash: boolean;
};

let updateInFlight = false;
let bootstrapCompleted = false;

export function setVisualizerUpdateInFlight(inFlight: boolean): void {
    updateInFlight = inFlight;
}

export function setVisualizerBootstrapCompleted(completed: boolean): void {
    bootstrapCompleted = completed;
}

export function getVisualizerReadinessSnapshot(
    triggerSource = 'unknown',
): VisualizerReadinessSnapshot {
    const clientReadiness: VisualizationReadiness = evaluateClientVisualizationReadiness();
    const lastRender = getLastVisualizerRender();
    const lifecycle = getWorkspaceLifecycle();
    const isLifecycleTrigger = triggerSource === 'lifecycleChanged';
    const isSoftTrigger =
        isLifecycleTrigger || triggerSource === 'startupRetry';

    if (!clientReadiness.ready) {
        const blockedByServer =
            lifecycle.phase === 'serverStarting' ||
            lifecycle.phase === 'degraded';
        return {
            phase: blockedByServer ? 'blockedServer' : 'blockedWorkspace',
            fetchAllowed: false,
            loadingMessage: clientReadiness.message,
            suppressNotReadyFlash: bootstrapCompleted && isLifecycleTrigger,
            suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
        };
    }

    if (updateInFlight) {
        return {
            phase: 'fetching',
            fetchAllowed: true,
            suppressNotReadyFlash: bootstrapCompleted && isLifecycleTrigger,
            suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
        };
    }

    if (lastRender?.outcome === 'diagram') {
        return {
            phase: 'settledDiagram',
            fetchAllowed: true,
            suppressNotReadyFlash: true,
            suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
        };
    }

    if (lastRender?.outcome === 'empty') {
        return {
            phase: 'settledEmpty',
            fetchAllowed: true,
            suppressNotReadyFlash: true,
            suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
        };
    }

    if (lastRender?.outcome === 'error') {
        return {
            phase: 'error',
            fetchAllowed: true,
            suppressNotReadyFlash: bootstrapCompleted && isLifecycleTrigger,
            suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
        };
    }

    return {
        phase: 'fetching',
        fetchAllowed: true,
        loadingMessage: getLifecycleMessage('visualizer', lifecycle.phase) || undefined,
        suppressNotReadyFlash: bootstrapCompleted && isLifecycleTrigger,
        suppressLoadingFlash: bootstrapCompleted && isSoftTrigger,
    };
}

export function shouldFetchVisualization(triggerSource: string, forceUpdate: boolean): boolean {
    const snapshot = getVisualizerReadinessSnapshot(triggerSource);
    if (!snapshot.fetchAllowed) {
        return false;
    }
    if (forceUpdate) {
        return true;
    }
    if (snapshot.phase === 'fetching') {
        return !updateInFlight;
    }
    return true;
}
