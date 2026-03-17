/**
 * Webview entry point. Bootstraps the visualizer via orchestrator.
 * Config (elkWorkerUrl) must be set via window.__VIZ_INIT by a minimal inline script
 * in the HTML before this bundle loads.
 */
declare const acquireVsCodeApi: () => { postMessage: (msg: unknown) => void };

const vscode = acquireVsCodeApi();
import { initializeOrchestrator } from './orchestrator';

initializeOrchestrator(vscode);
