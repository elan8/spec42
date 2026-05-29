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

export function resolveNodeChrome(
  kind: string,
  opts?: {
    isDefinition?: boolean;
    isReference?: boolean;
    isContainer?: boolean;
  },
): NodeChrome {
  const normalized = kind.toLowerCase();
  const isContainer =
    opts?.isContainer ??
    (normalized.includes("container") || normalized.includes("part_usage"));

  if (isContainer) {
    return {
      isDefinition: false,
      isReference: false,
      isContainer: true,
      cornerRadius: 8,
      strokeDasharray: "4,4",
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
