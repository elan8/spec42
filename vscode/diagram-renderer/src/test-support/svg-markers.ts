/**
 * Structural-marker extraction shared by the golden CLI-vs-webview parity tests
 * (`renderer.golden-parity.test.ts` runs in jsdom, `headless-export.golden-parity.test.ts`
 * runs against the headless virtual DOM). Both render the same fixture payload through the
 * identical `renderVisualization`/`exportSvg` code path and must produce the same summary here.
 */
export interface SvgMarkerSummary {
  classCounts: Record<string, number>;
  markerIds: string[];
}

export function summarizeSvgMarkers(svg: string): SvgMarkerSummary {
  const classCounts: Record<string, number> = {};
  const classAttrPattern = /\bclass="([^"]*)"/g;
  for (const match of svg.matchAll(classAttrPattern)) {
    for (const cls of match[1].split(/\s+/).filter(Boolean)) {
      classCounts[cls] = (classCounts[cls] ?? 0) + 1;
    }
  }

  const markerIds = new Set<string>();
  const markerPattern = /<marker[^>]*\bid="([^"]+)"/g;
  for (const match of svg.matchAll(markerPattern)) {
    markerIds.add(match[1]);
  }

  return { classCounts, markerIds: Array.from(markerIds).sort() };
}
