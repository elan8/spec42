import type { RenderCompleteEvent, RenderOutcome } from './renderContract';

export type WaitForRenderOptions = {
    view?: string;
    outcome?: RenderOutcome | RenderOutcome[];
    minGraphNodes?: number;
    updateId?: string;
    timeoutMs?: number;
};

type RenderWaiter = {
    options: WaitForRenderOptions;
    resolve: (event: RenderCompleteEvent) => void;
    reject: (error: Error) => void;
    timer: ReturnType<typeof setTimeout>;
};

let lastRender: RenderCompleteEvent | null = null;
const waiters: RenderWaiter[] = [];

function matchesOptions(event: RenderCompleteEvent, options: WaitForRenderOptions): boolean {
    if (options.updateId !== undefined && event.updateId !== options.updateId) {
        return false;
    }
    if (options.view !== undefined && event.view !== options.view) {
        return false;
    }
    if (options.outcome !== undefined) {
        const allowed = Array.isArray(options.outcome) ? options.outcome : [options.outcome];
        if (!allowed.includes(event.outcome)) {
            return false;
        }
    }
    if (options.minGraphNodes !== undefined && event.graphNodes < options.minGraphNodes) {
        return false;
    }
    if (event.outcome === 'cancelled') {
        return false;
    }
    return true;
}

function resolveMatchingWaiters(): void {
    for (let index = waiters.length - 1; index >= 0; index -= 1) {
        const waiter = waiters[index];
        if (!lastRender || !matchesOptions(lastRender, waiter.options)) {
            continue;
        }
        clearTimeout(waiter.timer);
        waiters.splice(index, 1);
        waiter.resolve(lastRender);
    }
}

export function onRenderComplete(event: Omit<RenderCompleteEvent, 'timestampMs'>): void {
    lastRender = {
        ...event,
        timestampMs: Date.now(),
    };
    resolveMatchingWaiters();
}

export function getLastVisualizerRender(): RenderCompleteEvent | null {
    return lastRender;
}

export function resetVisualizerRenderTracker(): void {
    lastRender = null;
    while (waiters.length > 0) {
        const waiter = waiters.pop();
        if (waiter) {
            clearTimeout(waiter.timer);
            waiter.reject(new Error('Visualizer render tracker reset'));
        }
    }
}

export function waitForVisualizerRender(options: WaitForRenderOptions = {}): Promise<RenderCompleteEvent> {
    const timeoutMs = options.timeoutMs ?? 20000;
    if (lastRender && matchesOptions(lastRender, options)) {
        return Promise.resolve(lastRender);
    }
    return new Promise<RenderCompleteEvent>((resolve, reject) => {
        const timer = setTimeout(() => {
            const index = waiters.findIndex((waiter) => waiter.resolve === resolve);
            if (index >= 0) {
                waiters.splice(index, 1);
            }
            const detail = lastRender
                ? ` Last render: view=${lastRender.view} outcome=${lastRender.outcome} graphNodes=${lastRender.graphNodes} hasExportableSvg=${lastRender.hasExportableSvg}${lastRender.updateId ? ` updateId=${lastRender.updateId}` : ''}.`
                : ' No renderComplete received yet.';
            const filterParts: string[] = [];
            if (options.view !== undefined) {
                filterParts.push(`view=${options.view}`);
            }
            if (options.outcome !== undefined) {
                const allowed = Array.isArray(options.outcome) ? options.outcome : [options.outcome];
                filterParts.push(`outcome in [${allowed.join(', ')}]`);
            }
            if (options.minGraphNodes !== undefined) {
                filterParts.push(`minGraphNodes=${options.minGraphNodes}`);
            }
            const filterDetail =
                filterParts.length > 0 ? ` Waiting for ${filterParts.join(', ')}.` : '';
            reject(
                new Error(
                    `Visualizer render did not settle within ${timeoutMs}ms.${filterDetail}${detail}`
                )
            );
        }, timeoutMs);
        waiters.push({ options, resolve, reject, timer });
        if (lastRender && matchesOptions(lastRender, options)) {
            clearTimeout(timer);
            const index = waiters.findIndex((waiter) => waiter.resolve === resolve);
            if (index >= 0) {
                waiters.splice(index, 1);
            }
            resolve(lastRender);
        }
    });
}
