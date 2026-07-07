/**
 * Single source of truth for ELK root-graph `layoutOptions` across the 3 places that
 * independently constructed these option bags before: general-view (`render/layout.ts`),
 * interconnection-view (`render/interconnection-elk-input.ts`), and the behavior-family views
 * (`views/behavior-common.ts`, action-flow/state-transition/sequence). Each previously hand-wrote
 * a similar-but-drifted option object; this module preserves every value exactly as it was live
 * at each call site (verified per-kind before consolidating — see
 * docs/VIEW-RENDERING-ISSUES.md Phase 4 entry) and only removes true duplication.
 *
 * Options that were IDENTICAL across every live call site become `COMMON_ELK_OPTIONS`. Anything
 * that differs per kind — even by one value — stays as an explicit per-kind default, not a
 * "clever" computed default, so a future reader can see exactly what each kind asked for without
 * cross-referencing this file against the old call sites.
 */

export type ElkViewKind = "general" | "interconnection" | "behavior-state" | "behavior-action";

/** Options every live call site already agreed on before this consolidation. */
const COMMON_ELK_OPTIONS: Record<string, string> = {
  "elk.algorithm": "layered",
  "elk.edgeRouting": "ORTHOGONAL",
  "elk.layered.nodePlacement.strategy": "NETWORK_SIMPLEX",
  "elk.separateConnectedComponents": "true",
  "elk.json.edgeCoords": "ROOT",
};

const PER_KIND_DEFAULTS: Record<ElkViewKind, Record<string, string>> = {
  general: {
    "elk.direction": "DOWN",
    "elk.spacing.nodeNode": "140",
    "elk.layered.spacing.nodeNodeBetweenLayers": "180",
    "elk.spacing.edgeNode": "90",
    "elk.spacing.edgeEdge": "80",
    "elk.aspectRatio": "1.4",
    "elk.padding": "[top=100,left=100,bottom=100,right=100]",
    "org.eclipse.elk.portConstraints": "FIXED_SIDE",
  },
  interconnection: {
    "elk.hierarchyHandling": "INCLUDE_CHILDREN",
    "elk.direction": "RIGHT",
    "elk.spacing.nodeNode": "150",
    "elk.layered.spacing.nodeNodeBetweenLayers": "220",
    "elk.spacing.edgeNode": "110",
    "elk.spacing.edgeEdge": "90",
    "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
    "elk.padding": "[top=70,left=70,bottom=70,right=70]",
    "org.eclipse.elk.portConstraints": "FIXED_ORDER",
    "org.eclipse.elk.portAlignment.default": "CENTER",
  },
  "behavior-state": {
    "elk.hierarchyHandling": "INCLUDE_CHILDREN",
    "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
    "elk.layered.spacing.nodeNodeBetweenLayers": "230",
    "elk.spacing.nodeNode": "190",
    "elk.spacing.edgeNode": "130",
    "elk.spacing.edgeEdge": "110",
    "elk.spacing.edgeLabel": "12",
    "elk.padding": "[top=100,left=90,bottom=90,right=90]",
  },
  "behavior-action": {
    "elk.layered.crossingMinimization.strategy": "LAYER_SWEEP",
    "elk.spacing.edgeNode": "80",
    "elk.spacing.edgeEdge": "60",
    "elk.spacing.edgeLabel": "12",
    "elk.padding": "[top=80,left=80,bottom=80,right=80]",
  },
};

/**
 * Build ELK root-graph `layoutOptions` for `kind`, starting from the shared defaults and this
 * kind's previously-hand-rolled values, then applying `overrides` (per-call-site deltas that
 * genuinely vary per invocation, e.g. horizontal-vs-vertical direction or conditional
 * hierarchy handling) last so callers can still customize without duplicating the base bag.
 */
export function buildElkLayoutOptions(
  kind: ElkViewKind,
  overrides: Record<string, string | undefined> = {},
): Record<string, string> {
  const merged: Record<string, string> = {
    ...COMMON_ELK_OPTIONS,
    ...PER_KIND_DEFAULTS[kind],
  };
  for (const [key, value] of Object.entries(overrides)) {
    if (value === undefined) {
      delete merged[key];
    } else {
      merged[key] = value;
    }
  }
  return merged;
}
