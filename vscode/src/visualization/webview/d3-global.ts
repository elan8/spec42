/**
 * Webview loads d3 via <script> before this bundle. Shared diagram-renderer imports
 * "d3" and must use the same instance as the orchestrator for zoom/pan transforms.
 */
declare global {
    interface Window {
        d3: typeof import("d3");
    }
}

const lib = typeof window !== "undefined" ? window.d3 : undefined;
if (!lib) {
    throw new Error("d3 global is required; load the d3 script before visualizer.js");
}

export default lib;
