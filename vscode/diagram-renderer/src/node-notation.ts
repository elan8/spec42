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
