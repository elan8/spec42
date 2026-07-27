import { describe, expect, it } from "vitest";
import { exportHeadlessSvg } from "./headless-export";
import { generalViewGoldenPayload, interconnectionViewGoldenPayload } from "./test-support/golden-parity-payloads";
import { summarizeSvgMarkers } from "./test-support/svg-markers";
import generalViewGolden from "./test-support/golden-parity/general-view.markers.json";
import interconnectionViewGolden from "./test-support/golden-parity/interconnection-view.markers.json";

/**
 * Headless path: same virtual DOM used by `spec42 diagrams export` and
 * `POST /v1/diagrams/export` (via the compiled `headless-renderer.js` bundle run through
 * QuickJS in `crates/server/src/headless_renderer.rs`). Renders the identical fixture
 * payloads `renderer.golden-parity.test.ts` renders through a real jsdom DOM, and must
 * produce the same structural markers -- see that file's doc comment and
 * DIAGRAM-EXPORT-QUALITY-ANALYSIS.md's "CLI vs VS Code golden parity tests" item.
 *
 * A failure here that the jsdom suite doesn't share means the virtual DOM shim in
 * `headless-export.ts` has diverged from real DOM behavior for this fixture -- e.g. a new
 * renderer feature relying on a DOM API `VirtualElement` doesn't implement.
 */
describe("headless vs webview (jsdom) golden parity", () => {
  it("General View structural markers match the golden fixture", async () => {
    const svg = await exportHeadlessSvg(generalViewGoldenPayload, { width: 1280, height: 900, colorScheme: "light" });
    expect(summarizeSvgMarkers(svg)).toEqual(generalViewGolden);
  });

  it("Interconnection View structural markers match the golden fixture", async () => {
    const svg = await exportHeadlessSvg(interconnectionViewGoldenPayload, {
      width: 1280,
      height: 900,
      colorScheme: "light",
    });
    expect(summarizeSvgMarkers(svg)).toEqual(interconnectionViewGolden);
  });
});
