/** SysML v2 graphical notation: definition / usage / reference node chrome. */

export type NodeStructureClass =
  | "viz-node--definition"
  | "viz-node--usage"
  | "viz-node--reference"
  | "viz-node--container";

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

export function isDefinitionKind(kind: string): boolean {
  const normalized = kind.trim().toLowerCase();
  return (
    normalized.includes(" def") ||
    normalized.includes("_def") ||
    normalized.endsWith(" def") ||
    normalized.includes("definition")
  );
}

export function isReferenceKind(kind: string): boolean {
  const k = kind.trim().toLowerCase();
  if (k === "ref") return true;
  if (k.endsWith("-ref")) return true;
  if (k.endsWith(" ref")) return true;
  if (/\bref\b/.test(k) && !k.includes("refine")) return true;
  return false;
}

type NodeCategory = "structural" | "requirement" | "behavior" | "other";

function nodeCategory(kind: string): NodeCategory {
  const k = kind.toLowerCase();
  if (
    k.includes("requirement") ||
    k.includes("concern") ||
    k.includes("viewpoint") ||
    k.includes("stakeholder")
  ) {
    return "requirement";
  }
  if (
    k.includes("action") ||
    k.includes("state") ||
    k.includes("calc") ||
    k.includes("analysis") ||
    k.includes("enumeration")
  ) {
    return "behavior";
  }
  if (
    k.includes("part") ||
    k.includes("port") ||
    k.includes("item") ||
    k.includes("attribute") ||
    k.includes("interface") ||
    k.includes("occurrence")
  ) {
    return "structural";
  }
  return "other";
}

function usageCornerRadius(kind: string): number {
  const cat = nodeCategory(kind);
  if (cat === "requirement") return 16;
  if (cat === "behavior") return 12;
  return 8;
}

export interface NodeBodyChromeStyle {
  cornerRadius: number;
  strokeDasharray: string;
  strokeWidthPx: number;
  headerCornerRadius: number;
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
  if (selected) strokeWidthPx = 4;
  else if (isContainer) strokeWidthPx = 2;
  else if (opts?.generalView) strokeWidthPx = chrome.isDefinition ? 3 : 2;
  else strokeWidthPx = chrome.isDefinition ? 2 : 3;

  return {
    cornerRadius: chrome.cornerRadius,
    strokeDasharray: nodeBodyStrokeDasharray(chrome, opts?.isPackageContainer),
    strokeWidthPx,
    headerCornerRadius: chrome.isDefinition ? 0 : Math.max(2, chrome.cornerRadius - 2),
  };
}

export function resolveNodeChrome(
  kind: string,
  opts?: {
    isDefinition?: boolean;
    isReference?: boolean;
    isContainer?: boolean;
    isPackageContainer?: boolean;
  },
): NodeChrome {
  const normalized = kind.toLowerCase();
  const isContainer =
    opts?.isContainer ??
    (normalized.includes("container") || normalized.includes("part_usage"));

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

  const isReference = opts?.isReference ?? isReferenceKind(kind);
  const isDefinition = !isReference && (opts?.isDefinition ?? isDefinitionKind(kind));

  if (isReference) {
    return {
      isDefinition: false,
      isReference: true,
      isContainer: false,
      cornerRadius: usageCornerRadius(kind),
      strokeDasharray: "2,4",
      structureClass: "viz-node--reference",
      nodeClassSuffix: " reference-node",
    };
  }

  if (isDefinition) {
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

  return {
    isDefinition: false,
    isReference: false,
    isContainer: false,
    cornerRadius: usageCornerRadius(kind),
    strokeDasharray: null,
    structureClass: "viz-node--usage",
    nodeClassSuffix: " usage-node",
  };
}
