import * as vscode from "vscode";

let outputChannel: vscode.OutputChannel | undefined;

function getChannel(): vscode.OutputChannel {
  if (!outputChannel) {
    outputChannel = vscode.window.createOutputChannel("SysML");
  }
  return outputChannel;
}

function isDebugEnabled(): boolean {
  return (
    vscode.workspace.getConfiguration("spec42").get<boolean>("debug") ??
    vscode.workspace.getConfiguration("sysml-language-server").get<boolean>("debug") ??
    false
  );
}

export function isVerboseLoggingEnabled(): boolean {
  return (
    isDebugEnabled() ||
    vscode.workspace.getConfiguration("spec42").get<boolean>("logging.verbose") === true
  );
}

export function isPerformanceLoggingEnabled(): boolean {
  return (
    vscode.workspace.getConfiguration("spec42").get<boolean>("performanceLogging.enabled") ??
    vscode.workspace.getConfiguration("sysml-language-server").get<boolean>("performanceLogging.enabled") ??
    false
  );
}

function timestamp(): string {
  return new Date().toISOString();
}

function appendStructuredLine(prefix: string, payload: Record<string, unknown>): void {
  const channel = getChannel();
  try {
    channel.appendLine(`${prefix} ${JSON.stringify(payload)}`);
  } catch {
    channel.appendLine(`${prefix} {"serializationError":true}`);
  }
}

/**
 * Log a debug message to the SysML output channel (only when debug is enabled).
 */
export function log(msg: string, ...args: unknown[]): void {
  if (!isDebugEnabled()) return;
  const channel = getChannel();
  const extra = args.length > 0 ? " " + args.map((a) => JSON.stringify(a)).join(" ") : "";
  channel.appendLine(`[${timestamp()}] ${msg}${extra}`);
}

export function logPerfEvent(event: string, extra?: Record<string, unknown>): void {
  if (!isPerformanceLoggingEnabled()) return;
  appendStructuredLine("[SysML][perf]", { event, ...(extra ?? {}) });
}

export function logStartupEvent(phase: string, extra?: Record<string, unknown>): void {
  if (!isPerformanceLoggingEnabled()) return;
  appendStructuredLine("[SysML][startup]", { phase, ...(extra ?? {}) });
}

/**
 * Log an error (always, regardless of debug setting).
 */
export function logError(msg: string, err?: unknown): void {
  const channel = getChannel();
  const errStr = err instanceof Error ? err.message : String(err ?? "");
  channel.appendLine(`[${timestamp()}] ERROR: ${msg}${errStr ? ` — ${errStr}` : ""}`);
  if (err instanceof Error && err.stack) {
    channel.appendLine(err.stack);
  }
  // Mirror errors to the extension host console so CI logs capture root causes.
  try {
    // eslint-disable-next-line no-console
    console.error(`[SysML] ERROR: ${msg}`, err ?? "");
  } catch {
    // ignore
  }
}

/**
 * Show the SysML output channel (e.g. when user wants to see logs).
 */
export function showChannel(): void {
  getChannel().show();
}

/**
 * Get the SysML output channel for appending lines (used by visualization panel).
 */
export function getOutputChannel(): vscode.OutputChannel {
  return getChannel();
}
