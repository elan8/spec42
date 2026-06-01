export type VisualizationServerHealthState =
    | "starting"
    | "ready"
    | "indexing"
    | "degraded"
    | "restarting"
    | "crashed";

export type VisualizationReadiness = {
    ready: boolean;
    message?: string;
};

let languageClientReady = false;
let serverHealthState: VisualizationServerHealthState = "starting";

export function setVisualizationGateState(partial: {
    languageClientReady?: boolean;
    serverHealthState?: VisualizationServerHealthState;
}): void {
    if (partial.languageClientReady !== undefined) {
        languageClientReady = partial.languageClientReady;
    }
    if (partial.serverHealthState !== undefined) {
        serverHealthState = partial.serverHealthState;
    }
}

export function evaluateClientVisualizationReadiness(): VisualizationReadiness {
    if (!languageClientReady) {
        return {
            ready: false,
            message: "Starting SysML language server...",
        };
    }
    switch (serverHealthState) {
        case "starting":
            return {
                ready: false,
                message: "Starting SysML language server...",
            };
        case "indexing":
            return {
                ready: false,
                message: "Indexing SysML workspace...",
            };
        case "restarting":
            return {
                ready: false,
                message: "Restarting SysML language server...",
            };
        case "crashed":
            return {
                ready: false,
                message: "SysML language server is not available.",
            };
        case "degraded":
        case "ready":
            return { ready: true };
        default:
            return { ready: true };
    }
}
