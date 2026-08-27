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
  /** Disclosure-control chrome. Kept as tokens so controls stay legible in every scheme. */
  controlFill: string;
  controlStroke: string;
  controlForeground: string;
  controlHoverFill: string;
  focusRing: string;
  badgeFill: string;
  badgeText: string;
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
  canvasBackground: "#f6f7f9",
  panelBackground: "#eef0f4",
  nodeFill: "#ffffff",
  nodeBorder: "#374151",
  textPrimary: "#111827",
  textSecondary: "#6b7280",
  divider: "#d1d5db",
  highlight: "#d97706",
  controlFill: "#ffffff",
  controlStroke: "#6b7280",
  controlForeground: "#374151",
  controlHoverFill: "#e5e7eb",
  focusRing: "#2563eb",
  badgeFill: "#e5e7eb",
  badgeText: "#374151",
  edge: { default: "#374151" },
  frame: { stroke: "#9ca3af", text: "#374151" },
};

const NOTATION_THEME_DARK: Omit<DiagramTheme, "colorScheme"> = {
  canvasBackground: "#1a1a1a",
  panelBackground: "#2c2c2c",
  nodeFill: "#232323",
  nodeBorder: "#d4d4d4",
  textPrimary: "#e5e5e5",
  textSecondary: "#a3a3a3",
  divider: "#525252",
  highlight: "#fbbf24",
  controlFill: "#232323",
  controlStroke: "#a3a3a3",
  controlForeground: "#e5e5e5",
  controlHoverFill: "#3f3f3f",
  focusRing: "#60a5fa",
  badgeFill: "#3a3a3a",
  badgeText: "#e5e5e5",
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
  controlFill: "var(--vscode-editor-background)",
  controlStroke: "var(--vscode-descriptionForeground)",
  controlForeground: "var(--vscode-editor-foreground)",
  controlHoverFill: "var(--vscode-toolbar-hoverBackground, var(--vscode-button-secondaryBackground))",
  focusRing: "var(--vscode-focusBorder)",
  badgeFill: "var(--vscode-badge-background, var(--vscode-button-secondaryBackground))",
  badgeText: "var(--vscode-badge-foreground, var(--vscode-editor-foreground))",
  edge: { default: "var(--vscode-editor-foreground)" },
  frame: {
    stroke: "var(--vscode-panel-border)",
    text: "var(--vscode-editor-foreground)",
  },
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

