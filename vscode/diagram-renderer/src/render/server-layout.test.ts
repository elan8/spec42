// @vitest-environment jsdom
import { describe, expect, it, vi } from "vitest";
import { renderVisualization } from "../renderer";
import { SYNTHETIC_CASES } from "../../visual/synthetic-cases";
import type { DiagramProductIdentity, RequestServerLayout } from "./types";

/**
 * `redrawGeneral`'s server-layout path (issue #119): requestLayout is tried first when the host
 * wires one up, with the local elk.js path as fallback whenever it declines, fails, or is absent.
 * These tests exercise that through the real disclosure-toggle flow (the same one a user
 * expanding/collapsing a node in the webview drives), not by calling internal renderer functions
 * directly -- redrawGeneral, layoutGeneralView, and the generation counter are all closure-private.
 *
 * `redrawGeneral` also runs once at mount (not just on toggle), so every test's requestLayout
 * mock must handle that first call too -- tests key their mock's behavior off
 * `presentationRevision` (the redraw generation, 1 at mount, 2 on the first toggle, ...) rather
 * than raw call count, since that is what actually identifies "the toggle under test" here.
 */

const identity: DiagramProductIdentity = { modelDigest: "blake3:test", viewHandle: "general/root" };

function host(): HTMLElement {
  const target = document.createElement("div");
  Object.defineProperty(target, "clientWidth", { value: 1280, configurable: true });
  Object.defineProperty(target, "clientHeight", { value: 900, configurable: true });
  return target;
}

function collapsedRootPrepared() {
  const entry = SYNTHETIC_CASES.find((candidate) => candidate.id === "node-collapsed-root");
  if (!entry) throw new Error("node-collapsed-root fixture is missing from SYNTHETIC_CASES");
  return JSON.parse(JSON.stringify(entry.prepared));
}

/** A structurally valid but trivially fake ELK layout output for `graph`: same node/edge ids
 * (so `reshapeGeneralLayoutResult` can find every one), with `x` set to `marker` so the test can
 * tell which of several competing responses actually reached the DOM. */
function fakeLayoutOutput(graph: Record<string, unknown>, marker: number): Record<string, unknown> {
  const place = (node: Record<string, unknown>): Record<string, unknown> => ({
    ...node,
    x: marker,
    y: 0,
    width: typeof node.width === "number" ? node.width : 100,
    height: typeof node.height === "number" ? node.height : 40,
    children: Array.isArray(node.children) ? (node.children as Record<string, unknown>[]).map(place) : undefined,
  });
  return place(graph);
}

async function clickRootToggle(target: HTMLElement): Promise<void> {
  const control = target.querySelector('[data-node-id="n:0"] .general-node-toggle');
  if (!control) throw new Error("root node toggle control not found");
  control.dispatchEvent(new MouseEvent("click", { bubbles: true }));
  // The click handler kicks off an async redraw; let its microtasks settle before assertions.
  await new Promise((resolve) => setTimeout(resolve, 0));
}

function transformXOf(target: HTMLElement, nodeId: string): number | undefined {
  const group = target.querySelector(`[data-node-id="${nodeId}"]`);
  const transform = group?.getAttribute("transform");
  const match = transform?.match(/translate\(([-\d.]+)/);
  return match ? Number(match[1]) : undefined;
}

/** Resolves immediately with a marker derived from the revision, for every call except the ones
 * listed in `hang`, which the test resolves manually later. */
function makeRequestLayout(hang: number[] = []): {
  requestLayout: RequestServerLayout;
  spy: ReturnType<typeof vi.fn>;
  resolveHung: (revision: number, marker: number) => void;
} {
  const hung = new Map<number, (value: Record<string, unknown> | null) => void>();
  const spy = vi.fn((graph: Record<string, unknown>, _identity: DiagramProductIdentity, revision: number) => {
    if (hang.includes(revision)) {
      return new Promise<Record<string, unknown> | null>((resolve) => hung.set(revision, resolve));
    }
    return Promise.resolve(fakeLayoutOutput(graph, revision * 1000));
  });
  return {
    requestLayout: spy as unknown as RequestServerLayout,
    spy,
    resolveHung: (revision, marker) => {
      const resolve = hung.get(revision);
      if (!resolve) throw new Error(`no hung request for revision ${revision}`);
      resolve(fakeLayoutOutput({ id: "root" }, marker));
    },
  };
}

describe("server-owned relayout (redrawGeneral -> requestLayout)", () => {
  it("uses the server's layout when requestLayout resolves", async () => {
    const target = host();
    const { requestLayout, spy } = makeRequestLayout();
    const controller = await renderVisualization(target, collapsedRootPrepared(), {
      theme: { colorScheme: "light" },
      productIdentity: identity,
      requestLayout,
    });
    expect(transformXOf(target, "n:0")).toBe(1000); // mount, revision 1

    await clickRootToggle(target);

    expect(spy).toHaveBeenCalledTimes(2); // mount + one toggle
    const [graph, sentIdentity, revision, signal] = spy.mock.calls[1];
    expect(sentIdentity).toEqual(identity);
    expect(revision).toBe(2);
    expect(signal).toBeInstanceOf(AbortSignal);
    expect(graph).toMatchObject({ id: "root" });
    expect(transformXOf(target, "n:0")).toBe(2000);

    controller.destroy();
  });

  it("falls back to local layout when requestLayout declines (offline/unreachable host)", async () => {
    const target = host();
    const requestLayout = vi.fn<RequestServerLayout>(() => Promise.resolve(null));
    const controller = await renderVisualization(target, collapsedRootPrepared(), {
      theme: { colorScheme: "light" },
      productIdentity: identity,
      requestLayout,
    });

    await clickRootToggle(target);

    expect(requestLayout).toHaveBeenCalledTimes(2); // mount + one toggle, both declined
    // No server layout was ever used, but the local elk.js fallback must still have produced a
    // complete, drawable layout -- not a blank or partial canvas.
    expect(target.querySelectorAll(".general-node").length).toBeGreaterThan(0);
    expect(target.querySelector("svg")?.outerHTML).not.toContain("NaN");

    controller.destroy();
  });

  it("falls back to local layout when no requestLayout is configured", async () => {
    const target = host();
    const controller = await renderVisualization(target, collapsedRootPrepared(), {
      theme: { colorScheme: "light" },
    });

    await clickRootToggle(target);

    expect(target.querySelectorAll(".general-node").length).toBeGreaterThan(0);
    controller.destroy();
  });

  it("cancels the in-flight request and ignores its late, stale response on a second toggle", async () => {
    const target = host();
    // Mount is revision 1 (resolves immediately); the first toggle is revision 2 (left hanging,
    // resolved late by the test); the second toggle is revision 3 (resolves immediately).
    const { requestLayout, spy, resolveHung } = makeRequestLayout([2]);
    const controller = await renderVisualization(target, collapsedRootPrepared(), {
      theme: { colorScheme: "light" },
      productIdentity: identity,
      requestLayout,
    });
    expect(transformXOf(target, "n:0")).toBe(1000);

    await clickRootToggle(target); // revision 2: starts, hangs
    await clickRootToggle(target); // revision 3: cancels revision 2's signal, resolves immediately

    expect(spy).toHaveBeenCalledTimes(3);
    const revision2Signal = spy.mock.calls[1][3] as AbortSignal;
    expect(revision2Signal.aborted).toBe(true);
    expect(transformXOf(target, "n:0")).toBe(3000);

    // The stale revision-2 response arrives late, as if the server had answered it anyway.
    resolveHung(2, 424242);
    await new Promise((resolve) => setTimeout(resolve, 0));

    // Revision 3's result must still be what's drawn; the late, superseded revision-2 response
    // must not have overwritten it.
    expect(transformXOf(target, "n:0")).toBe(3000);

    controller.destroy();
  });
});
