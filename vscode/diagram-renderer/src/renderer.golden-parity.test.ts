// @vitest-environment jsdom
import { describe, expect, it } from "vitest";
import { prepareViewData } from "./prepare";
import { renderVisualization } from "./renderer";
import { generalViewGoldenPayload, interconnectionViewGoldenPayload } from "./test-support/golden-parity-payloads";
import { summarizeSvgMarkers } from "./test-support/svg-markers";
import generalViewGolden from "./test-support/golden-parity/general-view.markers.json";
import interconnectionViewGolden from "./test-support/golden-parity/interconnection-view.markers.json";

/**
 * Reference-quality path: renders through a real DOM, same as the VS Code webview
 * (`renderSharedView()` in `vscode/src/visualization/webview/sharedRendererAdapter.ts`).
 * `headless-export.golden-parity.test.ts` renders the identical payloads through the
 * headless virtual DOM used by `spec42 diagrams export` / `POST /v1/diagrams/export` and
 * asserts against the same golden files. A mismatch between the two suites means the
 * headless bundle has silently diverged from the webview's shared-renderer output --
 * CLI vs VS Code golden parity structural markers.
 */
async function renderToSvg(payload: Record<string, unknown>): Promise<string> {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: 1280, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
  const prepared = prepareViewData(payload);
  const controller = await renderVisualization(target, prepared, { theme: { colorScheme: "light" } });
  const svg = controller.exportSvg();
  controller.destroy();
  return svg;
}

describe("webview (jsdom) vs headless golden parity", () => {
  it("General View structural markers match the golden fixture", async () => {
    const svg = await renderToSvg(generalViewGoldenPayload);
    expect(summarizeSvgMarkers(svg)).toEqual(generalViewGolden);
  });

  it("Interconnection View structural markers match the golden fixture", async () => {
    const svg = await renderToSvg(interconnectionViewGoldenPayload);
    expect(summarizeSvgMarkers(svg)).toEqual(interconnectionViewGolden);
  });
});
