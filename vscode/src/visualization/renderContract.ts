/** Shared extension ↔ webview visualizer render contract. */

export type RenderOutcome = 'diagram' | 'empty' | 'error' | 'cancelled';

export interface RenderCompleteMessage {
    command: 'renderComplete';
    updateId?: string;
    view: string;
    dataHash: string;
    outcome: RenderOutcome;
    graphNodes: number;
    hasExportableSvg: boolean;
}

export interface RenderCompleteEvent {
    updateId?: string;
    view: string;
    dataHash: string;
    outcome: RenderOutcome;
    graphNodes: number;
    hasExportableSvg: boolean;
    timestampMs: number;
}

export function createUpdateId(): string {
    return `upd-${Date.now()}-${Math.random().toString(36).slice(2, 9)}`;
}
