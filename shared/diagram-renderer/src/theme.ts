export interface DiagramTheme {
  canvasBackground: string;
  panelBackground: string;
  nodeFill: string;
  nodeBorder: string;
  textPrimary: string;
  textSecondary: string;
  divider: string;
  highlight: string;
  node: Record<string, string>;
  edge: Record<string, string>;
  port: Record<string, string>;
  frame: {
    stroke: string;
    text: string;
  };
}

export type DiagramThemeOverrides = Partial<Omit<DiagramTheme, "node" | "edge" | "port" | "frame">> & {
  node?: Partial<DiagramTheme["node"]>;
  edge?: Partial<DiagramTheme["edge"]>;
  port?: Partial<DiagramTheme["port"]>;
  frame?: Partial<DiagramTheme["frame"]>;
};

export const DEFAULT_DIAGRAM_THEME: DiagramTheme = {
  canvasBackground: "var(--vscode-editor-background, transparent)",
  panelBackground: "var(--vscode-button-secondaryBackground)",
  nodeFill: "var(--vscode-editor-background)",
  nodeBorder: "#E5E7EB",
  textPrimary: "var(--vscode-editor-foreground)",
  textSecondary: "var(--vscode-descriptionForeground)",
  divider: "var(--vscode-panel-border)",
  highlight: "#FFD700",
  node: {
    default: "var(--vscode-panel-border, #E5E7EB)",
    package: "#6B7280",
    part: "#2D8A6E",
    port: "#0E7C7B",
    attribute: "#4A9B7F",
    item: "#5A9B6E",
    interface: "#7BAA7D",
    action: "#D4A02C",
    state: "#B85C38",
    requirement: "#5B8FC4",
    useCase: "#6B9BD1",
    allocation: "#9CA3AF",
    constraint: "#E07C5A",
    enumeration: "#C9A227",
    metadata: "#8B7355",
    occurrence: "#5A9B6E",
    analysis: "#D4A02C",
    verification: "#C9A227",
  },
  edge: {
    default: "var(--vscode-editor-foreground, #d0d0d0)",
    relationship: "var(--vscode-editor-foreground, #d0d0d0)",
    specializes: "var(--vscode-editor-foreground, #d0d0d0)",
    typing: "var(--vscode-editor-foreground, #d0d0d0)",
    hierarchy: "var(--vscode-editor-foreground, #d0d0d0)",
    dependency: "var(--vscode-editor-foreground, #d0d0d0)",
    allocate: "#9CA3AF",
    satisfy: "#5B8FC4",
    verify: "#C9A227",
    bind: "#2F6FDD",
    connection: "#2F6FDD",
    flow: "var(--vscode-charts-green, #2f8f46)",
    interface: "var(--vscode-charts-purple, #8b5cf6)",
    reference: "#2F6FDD",
    transition: "var(--vscode-editor-foreground, #d0d0d0)",
  },
  port: {
    default: "#2F6FDD",
    connection: "#2F6FDD",
  },
  frame: {
    stroke: "#E5E7EB",
    text: "var(--vscode-editor-foreground)",
  },
};

export function resolveDiagramTheme(overrides?: DiagramThemeOverrides): DiagramTheme {
  return {
    ...DEFAULT_DIAGRAM_THEME,
    ...(overrides ?? {}),
    node: { ...DEFAULT_DIAGRAM_THEME.node, ...(overrides?.node ?? {}) },
    edge: { ...DEFAULT_DIAGRAM_THEME.edge, ...(overrides?.edge ?? {}) },
    port: { ...DEFAULT_DIAGRAM_THEME.port, ...(overrides?.port ?? {}) },
    frame: { ...DEFAULT_DIAGRAM_THEME.frame, ...(overrides?.frame ?? {}) },
  };
}

export function nodeColorForKind(kind: string, theme: DiagramTheme): string {
  const normalized = normalizeKind(kind);
  if (normalized.includes("package")) return theme.node.package;
  if (normalized.includes("requirement") || normalized === "req") return theme.node.requirement;
  if (normalized.includes("use_case") || normalized.includes("usecase")) return theme.node.useCase;
  if (normalized.includes("verification")) return theme.node.verification;
  if (normalized.includes("analysis")) return theme.node.analysis;
  if (normalized.includes("allocation") || normalized.includes("allocate")) return theme.node.allocation;
  if (normalized.includes("constraint")) return theme.node.constraint;
  if (normalized.includes("enumeration") || normalized.includes("enum")) return theme.node.enumeration;
  if (normalized.includes("metadata")) return theme.node.metadata;
  if (normalized.includes("occurrence")) return theme.node.occurrence;
  if (normalized.includes("interface")) return theme.node.interface;
  if (normalized.includes("state")) return theme.node.state;
  if (normalized.includes("action") || normalized.includes("calc")) return theme.node.action;
  if (normalized.includes("item")) return theme.node.item;
  if (normalized.includes("attribute")) return theme.node.attribute;
  if (normalized.includes("port")) return theme.node.port;
  if (normalized.includes("part")) return theme.node.part;
  return theme.node.default;
}

export function edgeColorForKind(kind: string, theme: DiagramTheme): string {
  const normalized = normalizeKind(kind);
  return theme.edge[normalized] ?? theme.edge.default;
}

function normalizeKind(kind: string): string {
  return String(kind || "")
    .trim()
    .toLowerCase()
    .replace(/[^a-z0-9]+/g, "_")
    .replace(/^_+|_+$/g, "");
}
