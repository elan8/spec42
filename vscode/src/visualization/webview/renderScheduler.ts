export type PendingRenderRequest = {
    view: string;
    preserveZoomOverride: unknown;
    allowDuringResize: boolean;
};

export type WebviewPerfFn = (event: string, data?: Record<string, unknown>) => void;

export type ActiveRender = {
    requestId: number;
    abortController: AbortController;
    isStale: () => boolean;
};

export type FinishedRender = {
    supersededByNewerRequest: boolean;
    nextRequest: PendingRenderRequest | null;
};

export class RenderScheduler {
    dataHash = '';
    isRendering = false;

    private pendingRenderRequest: PendingRenderRequest | null = null;
    private pendingViewRenderTimeout: ReturnType<typeof setTimeout> | null = null;
    private activeRenderAbortController: AbortController | null = null;
    private activeRenderRequestId = 0;

    constructor(private readonly webviewPerf: WebviewPerfFn) {}

    cancelOutstandingRenderRequests(reason = 'view-switch'): void {
        this.pendingRenderRequest = null;
        this.clearPendingViewRenderTimeout();
        if (this.activeRenderAbortController) {
            try {
                this.activeRenderAbortController.abort();
            } catch {
                // Ignore abort races.
            }
            this.activeRenderAbortController = null;
        }
        this.isRendering = false;
        this.webviewPerf('visualizer:webviewRenderCancelled', { reason });
    }

    setPendingViewRenderTimeout(timeout: ReturnType<typeof setTimeout>): void {
        this.clearPendingViewRenderTimeout();
        this.pendingViewRenderTimeout = timeout;
    }

    clearPendingViewRenderTimeout(): void {
        if (this.pendingViewRenderTimeout) {
            clearTimeout(this.pendingViewRenderTimeout);
            this.pendingViewRenderTimeout = null;
        }
    }

    beginRender(): ActiveRender {
        const requestId = ++this.activeRenderRequestId;
        const abortController = new AbortController();
        if (this.activeRenderAbortController) {
            try {
                this.activeRenderAbortController.abort();
            } catch {
                // Ignore abort races.
            }
        }
        this.activeRenderAbortController = abortController;

        return {
            requestId,
            abortController,
            isStale: () => abortController.signal.aborted || requestId !== this.activeRenderRequestId,
        };
    }

    queueRenderRequest(request: PendingRenderRequest): void {
        this.pendingRenderRequest = request;
    }

    markRendering(): void {
        this.isRendering = true;
    }

    finishRender(abortController: AbortController, requestId: number): FinishedRender {
        const supersededByNewerRequest = requestId !== this.activeRenderRequestId;
        this.isRendering = false;
        if (this.activeRenderAbortController === abortController) {
            this.activeRenderAbortController = null;
        }
        const nextRequest = this.pendingRenderRequest;
        this.pendingRenderRequest = null;

        return {
            supersededByNewerRequest,
            nextRequest,
        };
    }
}
