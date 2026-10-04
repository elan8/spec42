// @vitest-environment jsdom
import { readFileSync, readdirSync } from "node:fs";
import { resolve } from "node:path";
import { describe, expect, it } from "vitest";
import { parseDiagramProduct } from "../../src/diagram/diagramViewerCore";
import { prepareViewData } from "./prepare";
import { isNativeDiagramView } from "./renderer";
import type { PreparedView } from "./prepare/types";

/**
 * Every snapshot view prepares as a native `spec42/draw` view. Drawing is covered by
 * `diagram_draw` goldens and `spec42/draw` integration tests — there is no client fallback.
 */
const SNAPSHOT_DIR = resolve(process.cwd(), "../../tests/snapshots/generation");

function productFiles(): string[] {
  return readdirSync(SNAPSHOT_DIR)
    .filter((name) => name.startsWith("diagram_") && name.endsWith(".md"))
    .sort();
}

function preparedFromSnapshot(file: string): PreparedView {
  const text = readFileSync(resolve(SNAPSHOT_DIR, file), "utf8");
  const generated = text.split("# GENERATED\n", 2)[1];
  const match = generated?.match(/## diagram\.json\n~~~json\n([\s\S]*?)\n~~~/);
  if (!match) throw new Error(`${file} has no exact diagram.json product`);
  return prepareViewData(parseDiagramProduct(match[1]) as unknown as Record<string, unknown>);
}

describe("visual review corpus", () => {
  it.each(productFiles())("prepares %s as a native view", (file) => {
    const prepared = preparedFromSnapshot(file);
    expect(prepared.view).toBeTruthy();
    expect(isNativeDiagramView(prepared.view)).toBe(true);
  });
});
