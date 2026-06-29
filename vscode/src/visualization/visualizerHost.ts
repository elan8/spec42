import * as vscode from 'vscode';

/**
 * Minimal abstraction over WebviewPanel and WebviewView so the controller
 * can drive either without knowing which it holds.
 */
export interface VisualizerHost {
    readonly webview: vscode.Webview;
    readonly visible: boolean;
    onDidDispose(handler: () => void, thisArg: unknown, disposables: vscode.Disposable[]): vscode.Disposable;
    onVisibilityChange(handler: () => void, disposables: vscode.Disposable[]): void;
    reveal(): void;
    dispose(): void;
    title: string;
}

export function createWebviewViewHost(view: vscode.WebviewView): VisualizerHost {
    return {
        get webview() { return view.webview; },
        get visible() { return view.visible; },
        onDidDispose: (handler, thisArg, disposables) => view.onDidDispose(handler, thisArg, disposables),
        onVisibilityChange: (handler, disposables) => {
            view.onDidChangeVisibility(() => handler(), null, disposables);
        },
        reveal: () => { void vscode.commands.executeCommand('sysmlVisualizerView.focus'); },
        dispose: () => { /* VS Code manages WebviewView lifecycle */ },
        get title() { return view.title ?? 'SysML Visualizer'; },
        set title(v: string) { view.title = v; },
    };
}
