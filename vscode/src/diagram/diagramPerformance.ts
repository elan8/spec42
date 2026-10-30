/** Internal measurement envelope, separate from the generated diagram-product contract. */
export type RenderMeasurementIdentity = {
  renderId: number;
  modelDigest: string;
  viewHandle: string;
};

export const RENDER_MEASUREMENT_OUTCOMES = ["rendered", "empty", "unavailable", "failed"] as const;
export type RenderMeasurementOutcome = typeof RENDER_MEASUREMENT_OUTCOMES[number];

export type RenderMeasurement = RenderMeasurementIdentity & {
  type: "renderComplete";
  outcome: RenderMeasurementOutcome;
  webviewTotalMs: number;
  webviewPrepareMs: number;
  webviewRenderMs: number;
  frameWaitMs: number;
  framesObserved: number;
  visible: boolean;
  viewportReused: boolean;
  disclosureReused: boolean;
};

export function parseRenderMeasurement(value: unknown): RenderMeasurement | undefined {
  if (!value || typeof value !== "object") return undefined;
  const record = value as Record<string, unknown>;
  if (record.type !== "renderComplete" || !Number.isSafeInteger(record.renderId) || (record.renderId as number) < 1 ||
      typeof record.modelDigest !== "string" || typeof record.viewHandle !== "string" ||
      !RENDER_MEASUREMENT_OUTCOMES.some(outcome => outcome === record.outcome) ||
      !Number.isInteger(record.framesObserved) || (record.framesObserved as number) < 0 || (record.framesObserved as number) > 2 ||
      ["visible", "viewportReused", "disclosureReused"].some(key => typeof record[key] !== "boolean")) return undefined;
  for (const key of ["webviewTotalMs", "webviewPrepareMs", "webviewRenderMs", "frameWaitMs"]) {
    if (typeof record[key] !== "number" || !Number.isFinite(record[key]) || record[key] < 0) return undefined;
  }
  const result = record as RenderMeasurement;
  if (result.webviewPrepareMs + result.webviewRenderMs + result.frameWaitMs > result.webviewTotalMs + 0.01) return undefined;
  return {
    type: "renderComplete", renderId: result.renderId, modelDigest: result.modelDigest,
    viewHandle: result.viewHandle, outcome: result.outcome,
    webviewTotalMs: result.webviewTotalMs, webviewPrepareMs: result.webviewPrepareMs,
    webviewRenderMs: result.webviewRenderMs, frameWaitMs: result.frameWaitMs,
    framesObserved: result.framesObserved, visible: result.visible,
    viewportReused: result.viewportReused, disclosureReused: result.disclosureReused,
  };
}

/** A successful frame observation requires a mounted SVG and two visible animation frames.
 * This records a browser frame opportunity, not a GPU/compositor or physical-display timestamp. */
export function isPresentedFrame(record: RenderMeasurement): boolean {
  return record.outcome === "rendered" && record.visible && record.framesObserved === 2;
}

/** One pending render per provider. Replacing/invalidating it makes old acknowledgements inert. */
export class RenderMeasurementTracker {
  private sequence = 0;
  private pending: { identity: RenderMeasurementIdentity; postedMs: number } | undefined;

  begin(modelDigest: string, viewHandle: string, postedMs: number): RenderMeasurementIdentity {
    const identity = { renderId: ++this.sequence, modelDigest, viewHandle };
    this.pending = { identity, postedMs };
    return identity;
  }

  invalidate(): void { this.pending = undefined; }

  complete(value: unknown, receivedMs: number): (RenderMeasurement & { postToAckMs: number }) | undefined {
    const result = parseRenderMeasurement(value);
    const pending = this.pending;
    if (!result || !pending || !Number.isFinite(pending.postedMs) || !Number.isFinite(receivedMs) || receivedMs < pending.postedMs ||
        result.renderId !== pending.identity.renderId || result.modelDigest !== pending.identity.modelDigest ||
        result.viewHandle !== pending.identity.viewHandle) return undefined;
    this.pending = undefined;
    return { ...result, postToAckMs: receivedMs - pending.postedMs };
  }
}
