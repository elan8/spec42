import * as vscode from "vscode";

export type WaitForDocumentDiagnosticsOptions = {
    debounceMs?: number;
    timeoutMs?: number;
};

/**
 * Waits until the language server publishes diagnostics for `uri`, or until timeout.
 * Used so the visualizer does not fetch mid-parse after a file change.
 */
export function waitForDocumentDiagnostics(
    uri: vscode.Uri,
    options: WaitForDocumentDiagnosticsOptions = {},
): Promise<void> {
    const debounceMs = options.debounceMs ?? 200;
    const timeoutMs = options.timeoutMs ?? 2500;
    const uriText = uri.toString();

    return new Promise((resolve) => {
        let debounceTimer: ReturnType<typeof setTimeout> | undefined;
        let timeoutTimer: ReturnType<typeof setTimeout> | undefined;
        let settled = false;

        const finish = () => {
            if (settled) {
                return;
            }
            settled = true;
            if (debounceTimer) {
                clearTimeout(debounceTimer);
            }
            if (timeoutTimer) {
                clearTimeout(timeoutTimer);
            }
            disposable.dispose();
            resolve();
        };

        const scheduleFinish = () => {
            if (debounceTimer) {
                clearTimeout(debounceTimer);
            }
            debounceTimer = setTimeout(finish, debounceMs);
        };

        const disposable = vscode.languages.onDidChangeDiagnostics((event) => {
            if (event.uris.some((changed) => changed.toString() === uriText)) {
                scheduleFinish();
            }
        });

        timeoutTimer = setTimeout(finish, timeoutMs);
    });
}
