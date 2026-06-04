import * as vscode from "vscode";
import {
  buildCheckArgv,
  buildDoctorArgv,
  buildExplainDiagnosticArgv,
  buildModelSummaryArgv,
  Spec42CliContext,
} from "./spec42CliArgs";
import { resolveTargetPath, runSpec42Json } from "./spec42Cli";

type CheckInput = {
  path?: string;
  workspace_root?: string;
};

type ModelSummaryInput = {
  path?: string;
  workspace_root?: string;
  max_nodes?: number;
};

type ExplainInput = {
  code: string;
  path?: string;
  workspace_root?: string;
  line?: number;
};

function asRecord(value: unknown): Record<string, unknown> | undefined {
  return value && typeof value === "object" && !Array.isArray(value)
    ? (value as Record<string, unknown>)
    : undefined;
}

function formatCheckResult(data: unknown): string {
  const root = asRecord(data);
  const summary = asRecord(root?.summary);
  const documents = Array.isArray(root?.documents) ? root.documents : [];
  const errors = Number(summary?.error_count ?? 0);
  const warnings = Number(summary?.warning_count ?? 0);
  const lines: string[] = [
    `Spec42 check: ${errors} error(s), ${warnings} warning(s) across ${documents.length} document(s).`,
  ];
  const codes: string[] = [];
  for (const doc of documents) {
    const d = asRecord(doc);
    const diags = Array.isArray(d?.diagnostics) ? d.diagnostics : [];
    for (const diag of diags) {
      const entry = asRecord(diag);
      const code = entry?.code;
      if (typeof code === "string") {
        codes.push(code);
      } else if (code && typeof code === "object") {
        const c = asRecord(code);
        if (typeof c?.value === "string") {
          codes.push(c.value);
        }
      }
      if (codes.length >= 5) {
        break;
      }
    }
    if (codes.length >= 5) {
      break;
    }
  }
  if (codes.length > 0) {
    lines.push(`Top diagnostic codes: ${codes.join(", ")}`);
  }
  const advice = Array.isArray(root?.advice) ? root.advice : [];
  if (advice.length > 0 && typeof advice[0] === "string") {
    lines.push(`Advice: ${advice[0]}`);
  }
  return lines.join("\n");
}

function formatDoctorResult(data: unknown): string {
  const root = asRecord(data);
  const libs = Array.isArray(root?.library_paths) ? root.library_paths : [];
  const missing = libs.filter((p) => {
    const entry = asRecord(p);
    return entry?.exists === false;
  }).length;
  return [
    `Spec42 doctor (${root?.version ?? "unknown"}): stdlib ${root?.resolved_stdlib_path ? "resolved" : "not resolved"}, ${libs.length} library path(s) (${missing} missing).`,
    root?.sysand && typeof root.sysand === "object"
      ? `Sysand: ${asRecord(root.sysand)?.installed ? "installed" : "not installed"}.`
      : "",
  ]
    .filter(Boolean)
    .join("\n");
}

function formatModelSummaryResult(data: unknown): string {
  const root = asRecord(data);
  const summary = asRecord(root?.summary);
  const trunc = asRecord(root?.truncation);
  const nodes = Array.isArray(root?.nodes) ? root.nodes : [];
  const lines: string[] = [
    `Model summary: ${summary?.error_count ?? 0} error(s), ${summary?.warning_count ?? 0} warning(s).`,
    `Nodes ${trunc?.nodes_returned ?? nodes.length}/${trunc?.nodes_total ?? "?"}, relationships ${trunc?.relationships_returned ?? 0}/${trunc?.relationships_total ?? "?"}.`,
  ];
  const names = nodes
    .slice(0, 5)
    .map((n) => asRecord(n)?.qualified_name)
    .filter((n): n is string => typeof n === "string");
  if (names.length > 0) {
    lines.push(`Sample nodes: ${names.join(", ")}`);
  }
  return lines.join("\n");
}

function formatExplainResult(data: unknown): string {
  const root = asRecord(data);
  const catalog = asRecord(root?.catalog);
  const instances = Array.isArray(root?.instances) ? root.instances : [];
  const lines: string[] = [`Diagnostic code: ${root?.code ?? "(unknown)"}`];
  if (catalog) {
    lines.push(
      `${catalog.severity}: ${catalog.meaning}`,
      `Typical fix: ${catalog.typical_fix}`
    );
  } else {
    lines.push("(No catalog entry for this code.)");
  }
  if (instances.length > 0) {
    lines.push(`Instances (${instances.length}):`);
    for (const inst of instances.slice(0, 5)) {
      const i = asRecord(inst);
      lines.push(`  ${i?.uri}:${i?.line} — ${i?.message}`);
    }
  }
  return lines.join("\n");
}

function lmResult(text: string): vscode.LanguageModelToolResult {
  return new vscode.LanguageModelToolResult([new vscode.LanguageModelTextPart(text)]);
}

class Spec42CheckLmTool implements vscode.LanguageModelTool<CheckInput> {
  constructor(private readonly ctx: Spec42CliContext) {}

  async prepareInvocation(
    options: vscode.LanguageModelToolInvocationPrepareOptions<CheckInput>
  ) {
    const target = options.input.path ?? defaultLabel(this.ctx);
    return { invocationMessage: `Validating SysML model at ${target}…` };
  }

  async invoke(
    options: vscode.LanguageModelToolInvocationOptions<CheckInput>,
    _token: vscode.CancellationToken
  ): Promise<vscode.LanguageModelToolResult> {
    const targetPath = resolveTargetPath(options.input.path, this.ctx.workspaceRoot);
    const data = await runSpec42Json(
      this.ctx.serverCommand,
      buildCheckArgv(this.ctx, targetPath, options.input.workspace_root),
      this.ctx.workspaceRoot
    );
    return lmResult(formatCheckResult(data));
  }
}

class Spec42DoctorLmTool implements vscode.LanguageModelTool<Record<string, never>> {
  constructor(private readonly ctx: Spec42CliContext) {}

  async prepareInvocation() {
    return { invocationMessage: "Checking Spec42 environment (doctor)…" };
  }

  async invoke(
    _options: vscode.LanguageModelToolInvocationOptions<Record<string, never>>,
    _token: vscode.CancellationToken
  ): Promise<vscode.LanguageModelToolResult> {
    const data = await runSpec42Json(
      this.ctx.serverCommand,
      buildDoctorArgv(this.ctx),
      this.ctx.workspaceRoot
    );
    return lmResult(formatDoctorResult(data));
  }
}

class Spec42ModelSummaryLmTool implements vscode.LanguageModelTool<ModelSummaryInput> {
  constructor(private readonly ctx: Spec42CliContext) {}

  async prepareInvocation(
    options: vscode.LanguageModelToolInvocationPrepareOptions<ModelSummaryInput>
  ) {
    const target = options.input.path ?? defaultLabel(this.ctx);
    return { invocationMessage: `Building semantic model summary for ${target}…` };
  }

  async invoke(
    options: vscode.LanguageModelToolInvocationOptions<ModelSummaryInput>,
    _token: vscode.CancellationToken
  ): Promise<vscode.LanguageModelToolResult> {
    const targetPath = resolveTargetPath(options.input.path, this.ctx.workspaceRoot);
    const maxNodes = options.input.max_nodes ?? 500;
    const data = await runSpec42Json(
      this.ctx.serverCommand,
      buildModelSummaryArgv(this.ctx, targetPath, maxNodes, options.input.workspace_root),
      this.ctx.workspaceRoot
    );
    return lmResult(formatModelSummaryResult(data));
  }
}

class Spec42ExplainDiagnosticLmTool implements vscode.LanguageModelTool<ExplainInput> {
  constructor(private readonly ctx: Spec42CliContext) {}

  async prepareInvocation(
    options: vscode.LanguageModelToolInvocationPrepareOptions<ExplainInput>
  ) {
    return {
      invocationMessage: `Explaining diagnostic code ${options.input.code}…`,
    };
  }

  async invoke(
    options: vscode.LanguageModelToolInvocationOptions<ExplainInput>,
    _token: vscode.CancellationToken
  ): Promise<vscode.LanguageModelToolResult> {
    if (!options.input.code?.trim()) {
      throw new Error("Parameter `code` is required.");
    }
    const targetPath = options.input.path
      ? resolveTargetPath(options.input.path, this.ctx.workspaceRoot)
      : undefined;
    const data = await runSpec42Json(
      this.ctx.serverCommand,
      buildExplainDiagnosticArgv(this.ctx, options.input.code, {
        path: targetPath,
        line: options.input.line,
        workspaceRoot: options.input.workspace_root,
      }),
      this.ctx.workspaceRoot
    );
    return lmResult(formatExplainResult(data));
  }
}

function defaultLabel(ctx: Spec42CliContext): string {
  return ctx.workspaceRoot || "active SysML file";
}

export function registerSpec42LmTools(
  context: vscode.ExtensionContext,
  cliContext: Spec42CliContext
): void {
  const lm = (vscode as { lm?: { registerTool?: unknown } }).lm;
  if (!lm || typeof lm.registerTool !== "function") {
    return;
  }

  const register = lm.registerTool as (
    name: string,
    tool: vscode.LanguageModelTool<unknown>
  ) => vscode.Disposable;

  context.subscriptions.push(
    register("spec42_check", new Spec42CheckLmTool(cliContext)),
    register("spec42_doctor", new Spec42DoctorLmTool(cliContext)),
    register("spec42_model_summary", new Spec42ModelSummaryLmTool(cliContext)),
    register("spec42_explain_diagnostic", new Spec42ExplainDiagnosticLmTool(cliContext))
  );
}
