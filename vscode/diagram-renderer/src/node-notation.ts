/** SysML v2 graphical notation: definition / usage / reference node chrome. */

export type NodeStructureClass =
  | "viz-node--definition"
  | "viz-node--usage"
  | "viz-node--reference"
  | "viz-node--container"
  | "viz-node--unsupported";

export type NodeNotationRole =
  | "definition"
  | "usage"
  | "reference-usage"
  | "namespace"
  | "annotation"
  | "unsupported";

export interface NodeChrome {
  isDefinition: boolean;
  isReference: boolean;
  isContainer: boolean;
  cornerRadius: number;
  /** `null` = solid stroke */
  strokeDasharray: string | null;
  structureClass: NodeStructureClass;
  /** CSS class suffix for legacy renderers, e.g. ` definition-node` */
  nodeClassSuffix: string;
}

export interface NodeBodyChromeStyle {
  cornerRadius: number;
  strokeDasharray: string;
  strokeWidthPx: number;
}

/** Resolved stroke dash for a node body rect (package containers stay solid). */
export function nodeBodyStrokeDasharray(chrome: NodeChrome, isPackageContainer = false): string {
  if (chrome.isContainer && isPackageContainer) return "none";
  return chrome.strokeDasharray ?? "none";
}

/** Shared body rect metrics for general and interconnection node backgrounds. */
export function nodeBodyChromeStyle(
  chrome: NodeChrome,
  opts?: {
    selected?: boolean;
    isContainer?: boolean;
    isPackageContainer?: boolean;
    /** General view uses slightly heavier definition borders. */
    generalView?: boolean;
  },
): NodeBodyChromeStyle {
  const selected = opts?.selected ?? false;
  const isContainer = opts?.isContainer ?? chrome.isContainer;
  let strokeWidthPx = 2;
  if (selected) strokeWidthPx = 3;
  else if (isContainer) strokeWidthPx = 1.5;
  // General view keeps one restrained ink weight with only a slight definition emphasis; the
  // normative definition/usage distinction is carried by corner shape and dash, not by weight.
  else if (opts?.generalView) strokeWidthPx = chrome.isDefinition ? 2 : 1.5;
  else strokeWidthPx = chrome.isDefinition ? 2 : 3;

  return {
    cornerRadius: chrome.cornerRadius,
    strokeDasharray: nodeBodyStrokeDasharray(chrome, opts?.isPackageContainer),
    strokeWidthPx,
  };
}

/**
 * Decode the typed notation role a prepared node carries. Compatibility DTOs predate the typed
 * role and are decoded here only; the role is never inferred from the display kind or the label.
 */
export function notationRoleFromAttributes(attributes: Record<string, unknown> | undefined): NodeNotationRole {
  const attrs = attributes ?? {};
  const role = attrs.notationRole;
  if (
    role === "definition" || role === "usage" || role === "reference-usage" ||
    role === "namespace" || role === "annotation" || role === "unsupported"
  ) {
    return role;
  }
  if (attrs.isReference === true) return "reference-usage";
  if (attrs.isDefinition === true) return "definition";
  return "unsupported";
}

export function resolveNodeChrome(
  role: NodeNotationRole,
  opts?: {
    isContainer?: boolean;
    isPackageContainer?: boolean;
  },
): NodeChrome {
  const isContainer = opts?.isContainer ?? role === "namespace";

  if (isContainer) {
    const isPackageContainer = opts?.isPackageContainer ?? false;
    return {
      isDefinition: false,
      isReference: false,
      isContainer: true,
      cornerRadius: 8,
      strokeDasharray: isPackageContainer ? null : "4,4",
      structureClass: "viz-node--container",
      nodeClassSuffix: "",
    };
  }

  if (role === "reference-usage") {
    return {
      isDefinition: false,
      isReference: true,
      isContainer: false,
      cornerRadius: 8,
      strokeDasharray: "2,4",
      structureClass: "viz-node--reference",
      nodeClassSuffix: " reference-node",
    };
  }

  if (role === "definition") {
    return {
      isDefinition: true,
      isReference: false,
      isContainer: false,
      cornerRadius: 0,
      strokeDasharray: null,
      structureClass: "viz-node--definition",
      nodeClassSuffix: " definition-node",
    };
  }

  if (role === "unsupported") {
    return {
      isDefinition: false,
      isReference: false,
      isContainer: false,
      cornerRadius: 4,
      strokeDasharray: "3,3",
      structureClass: "viz-node--unsupported",
      nodeClassSuffix: " unsupported-node",
    };
  }

  return {
    isDefinition: false,
    isReference: false,
    isContainer: false,
    cornerRadius: 8,
    strokeDasharray: null,
    structureClass: "viz-node--usage",
    nodeClassSuffix: " usage-node",
  };
}

/**
 * Rounded-rectangle outline for a node body, inset from the laid-out box by `inset`.
 *
 * The body rect strokes on its own path, so half the stroke width falls inside the box. Any fill
 * that must stay *inside* the border -- the header compartment fill in particular -- has to be
 * inset by that half width and use a concentric corner radius, otherwise it paints over the inner
 * half of the border and the outline reads as broken around the rounded corners.
 */
export function nodeOutlinePath(width: number, height: number, radius: number, inset = 0): string {
  const left = inset;
  const top = inset;
  const right = Math.max(left, width - inset);
  const bottom = Math.max(top, height - inset);
  const maxRadius = Math.min((right - left) / 2, (bottom - top) / 2);
  const r = Math.max(0, Math.min(radius - inset, maxRadius));
  if (r <= 0) {
    return `M${left},${top}H${right}V${bottom}H${left}Z`;
  }
  return [
    `M${left + r},${top}`,
    `H${right - r}`,
    `A${r},${r} 0 0 1 ${right},${top + r}`,
    `V${bottom - r}`,
    `A${r},${r} 0 0 1 ${right - r},${bottom}`,
    `H${left + r}`,
    `A${r},${r} 0 0 1 ${left},${bottom - r}`,
    `V${top + r}`,
    `A${r},${r} 0 0 1 ${left + r},${top}`,
    "Z",
  ].join("");
}

/**
 * Fill region for the header compartment: follows the node's own top corners concentrically and
 * stops on a straight edge at `headerBottom`, entirely inside the body stroke. Never introduces
 * independently rounded header corners that would cut across the outer border.
 */
export function headerFillPath(
  width: number,
  headerBottom: number,
  radius: number,
  strokeWidthPx: number,
): string {
  const inset = strokeWidthPx / 2;
  const left = inset;
  const top = inset;
  const right = Math.max(left, width - inset);
  const bottom = Math.max(top, headerBottom - inset);
  const r = Math.max(0, Math.min(radius - inset, (right - left) / 2, bottom - top));
  if (r <= 0) {
    return `M${left},${top}H${right}V${bottom}H${left}Z`;
  }
  return [
    `M${left + r},${top}`,
    `H${right - r}`,
    `A${r},${r} 0 0 1 ${right},${top + r}`,
    `V${bottom}`,
    `H${left}`,
    `V${top + r}`,
    `A${r},${r} 0 0 1 ${left + r},${top}`,
    "Z",
  ].join("");
}

/** Horizontal extent a compartment divider may span without crossing the body stroke. */
export function nodeInnerSpan(width: number, strokeWidthPx: number): { x1: number; x2: number } {
  const inset = strokeWidthPx / 2;
  return { x1: inset, x2: Math.max(inset, width - inset) };
}
