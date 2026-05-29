export type DiagramColorScheme = "vscode" | "light" | "dark" | "auto";

export interface DiagramTheme {
  canvasBackground: string;
  panelBackground: string;
  nodeFill: string;
  nodeBorder: string;
  textPrimary: string;
  textSecondary: string;
  divider: string;
  highlight: string;
  edge: {
    default: string;
  };
  frame: {
    stroke: string;
    text: string;
  };
  colorScheme: DiagramColorScheme;
}

export type DiagramThemeOverrides = Partial<Omit<DiagramTheme, "edge" | "frame">> & {
  edge?: Partial<DiagramTheme["edge"]>;
  frame?: Partial<DiagramTheme["frame"]>;
  colorScheme?: DiagramColorScheme;
};

const NOTATION_THEME_LIGHT: Omit<DiagramTheme, "colorScheme"> = {
  canvasBackground: "#ffffff",
  panelBackground: "#f3f4f6",
  nodeFill: "#ffffff",
  nodeBorder: "#374151",
  textPrimary: "#111827",
  textSecondary: "#6b7280",
  divider: "#d1d5db",
  highlight: "#d97706",
  edge: { default: "#374151" },
  frame: { stroke: "#9ca3af", text: "#374151" },
};

const NOTATION_THEME_DARK: Omit<DiagramTheme, "colorScheme"> = {
  canvasBackground: "#1e1e1e",
  panelBackground: "#2d2d2d",
  nodeFill: "#1e1e1e",
  nodeBorder: "#d4d4d4",
  textPrimary: "#e5e5e5",
  textSecondary: "#a3a3a3",
  divider: "#525252",
  highlight: "#fbbf24",
  edge: { default: "#d4d4d4" },
  frame: { stroke: "#737373", text: "#e5e5e5" },
};

const NOTATION_THEME_VSCODE: Omit<DiagramTheme, "colorScheme"> = {
  canvasBackground: "var(--vscode-editor-background, transparent)",
  panelBackground: "var(--vscode-button-secondaryBackground)",
  nodeFill: "var(--vscode-editor-background)",
  nodeBorder: "var(--vscode-editor-foreground)",
  textPrimary: "var(--vscode-editor-foreground)",
  textSecondary: "var(--vscode-descriptionForeground)",
  divider: "var(--vscode-panel-border)",
  highlight: "var(--vscode-focusBorder, #d97706)",
  edge: { default: "var(--vscode-editor-foreground)" },
  frame: {
    stroke: "var(--vscode-panel-border)",
    text: "var(--vscode-editor-foreground)",
  },
};

/** @deprecated Use resolveDiagramTheme() — kept for test imports during migration. */
export const DEFAULT_DIAGRAM_THEME: DiagramTheme = {
  ...NOTATION_THEME_LIGHT,
  colorScheme: "light",
};

export function detectColorScheme(host?: HTMLElement | null): DiagramColorScheme {
  if (typeof host !== "undefined" && host !== null) {
    const svg = host.closest?.(".sysml-viz-svg");
    const scheme = svg?.getAttribute("data-color-scheme");
    if (scheme === "light" || scheme === "dark") {
      return scheme;
    }
  }
  if (typeof window !== "undefined" && typeof window.matchMedia === "function") {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
  }
  return "light";
}

function baseThemeForScheme(scheme: DiagramColorScheme): Omit<DiagramTheme, "colorScheme"> {
  if (scheme === "vscode") {
    return NOTATION_THEME_VSCODE;
  }
  const resolved = scheme === "auto" ? detectColorScheme() : scheme;
  return resolved === "dark" ? NOTATION_THEME_DARK : NOTATION_THEME_LIGHT;
}

export function resolveDiagramTheme(options?: DiagramThemeOverrides): DiagramTheme {
  const colorScheme = options?.colorScheme ?? "vscode";
  const base = baseThemeForScheme(colorScheme);
  return {
    ...base,
    ...(options ?? {}),
    colorScheme,
    edge: { ...base.edge, ...(options?.edge ?? {}) },
    frame: { ...base.frame, ...(options?.frame ?? {}) },
  };
}

/** Notation-neutral: all nodes share the same ink color. */
export function strokeColorForNode(theme: DiagramTheme): string {
  return theme.nodeBorder;
}

/** Notation-neutral: all edges share the same ink color; markers and dashes convey kind. */
export function strokeColorForEdge(_kind: string, theme: DiagramTheme): string {
  return theme.edge.default;
}

/** @deprecated Use strokeColorForNode(theme) */
export function nodeColorForKind(_kind: string, theme: DiagramTheme): string {
  return strokeColorForNode(theme);
}

/** @deprecated Use strokeColorForEdge(kind, theme) */
export function edgeColorForKind(kind: string, theme: DiagramTheme): string {
  return strokeColorForEdge(kind, theme);
}
