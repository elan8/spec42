#!/usr/bin/env node
// Reads a Spec42 performance report JSON and fails if any budget is exceeded.
// Usage: node scripts/check-perf-budgets.mjs <report-path>
import fs from "node:fs";
import path from "node:path";
import process from "node:process";

const reportPath = process.argv[2];
if (!reportPath) {
  console.error("Usage: check-perf-budgets.mjs <report-path>");
  process.exit(1);
}

if (!fs.existsSync(reportPath)) {
  console.error(`Performance report not found: ${reportPath}`);
  console.error("Run the performance test first to generate the report.");
  process.exit(1);
}

const report = JSON.parse(fs.readFileSync(reportPath, "utf8"));
const fixture = report.fixture?.name ?? path.basename(reportPath, ".json");

// Metric extractors per known report schema.
// Each entry: [label, actualFn, budgetFn]
const CHECKS = [
  // Large-workspace schema
  ["workspace model request", (r) => r.modelRequests?.workspace?.elapsedMs, (r) => r.budgets?.workspaceModelRequestMs],
  ["document model request", (r) => r.modelRequests?.document?.elapsedMs, (r) => r.budgets?.documentModelRequestMs],
  ["visualization request", (r) => r.visualization?.elapsedMs, (r) => r.budgets?.visualizationRequestMs],
  // Drone / interconnection schema
  ["cold headless visualization build", (r) => r.phaseBreakdown?.coldHeadlessVisualizationMs, (r) => r.budgets?.visualizationModelBuildMs],
  ["warm visualization cache hit", (r) => r.visualizationWarm?.elapsedMs, (r) => r.budgets?.warmVisualizationCacheHitMs],
];

const rows = [];
let anyExceeded = false;

for (const [label, actualFn, budgetFn] of CHECKS) {
  const actual = actualFn(report);
  const budget = budgetFn(report);
  if (actual == null || budget == null) continue;
  const exceeded = actual > budget;
  if (exceeded) anyExceeded = true;
  rows.push({ label, actual, budget, exceeded });
}

if (rows.length === 0) {
  console.log(`[perf] ${fixture}: no applicable budget checks found in report.`);
  process.exit(0);
}

const labelW = Math.max(8, ...rows.map((r) => r.label.length));
const header = `${"Metric".padEnd(labelW)}  ${"Actual (ms)".padStart(12)}  ${"Budget (ms)".padStart(12)}  Status`;
const sep = "-".repeat(header.length);
console.log(`\n[perf] ${fixture} budget check`);
console.log(sep);
console.log(header);
console.log(sep);
for (const { label, actual, budget, exceeded } of rows) {
  const status = exceeded ? "EXCEEDED" : "ok";
  console.log(`${label.padEnd(labelW)}  ${String(actual).padStart(12)}  ${String(budget).padStart(12)}  ${status}`);
}
console.log(sep);

if (anyExceeded) {
  console.error(`\n[perf] FAIL: one or more budgets exceeded for fixture "${fixture}".`);
  console.error("[perf] If this is a new regression, investigate before merging.");
  console.error("[perf] If budgets need adjustment, update the Rust test that emits the report.");
  process.exit(1);
} else {
  console.log(`\n[perf] All budgets within limits for fixture "${fixture}".`);
}
