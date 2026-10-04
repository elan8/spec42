/// Mirrors `DiagramTheme` / `NOTATION_THEME_LIGHT` and `NOTATION_THEME_DARK` in
/// `vscode/diagram-renderer/src/theme.ts`. VS Code CSS-variable tokens cannot be used in a
/// server-rendered SVG, so the webview maps `vscode-light`/`vscode-dark` body classes onto these
/// two palettes before calling `spec42/draw`.

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub canvas_background: &'static str,
    pub panel_background: &'static str,
    pub node_fill: &'static str,
    pub node_border: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub divider: &'static str,
    pub highlight: &'static str,
    pub control_fill: &'static str,
    pub control_stroke: &'static str,
    pub control_foreground: &'static str,
    pub control_hover_fill: &'static str,
    pub focus_ring: &'static str,
    pub badge_fill: &'static str,
    pub badge_text: &'static str,
    pub edge_default: &'static str,
    pub frame_stroke: &'static str,
    pub frame_text: &'static str,
    pub color_scheme: &'static str,
}

pub const LIGHT: Theme = Theme {
    canvas_background: "#f6f7f9",
    panel_background: "#eef0f4",
    node_fill: "#ffffff",
    node_border: "#374151",
    text_primary: "#111827",
    text_secondary: "#6b7280",
    divider: "#d1d5db",
    highlight: "#d97706",
    control_fill: "#ffffff",
    control_stroke: "#6b7280",
    control_foreground: "#374151",
    control_hover_fill: "#e5e7eb",
    focus_ring: "#2563eb",
    badge_fill: "#e5e7eb",
    badge_text: "#374151",
    edge_default: "#374151",
    frame_stroke: "#9ca3af",
    frame_text: "#374151",
    color_scheme: "light",
};

pub const DARK: Theme = Theme {
    canvas_background: "#1a1a1a",
    panel_background: "#2c2c2c",
    node_fill: "#232323",
    node_border: "#d4d4d4",
    text_primary: "#e5e5e5",
    text_secondary: "#a3a3a3",
    divider: "#525252",
    highlight: "#fbbf24",
    control_fill: "#232323",
    control_stroke: "#a3a3a3",
    control_foreground: "#e5e5e5",
    control_hover_fill: "#3f3f3f",
    focus_ring: "#60a5fa",
    badge_fill: "#3a3a3a",
    badge_text: "#e5e5e5",
    edge_default: "#d4d4d4",
    frame_stroke: "#737373",
    frame_text: "#e5e5e5",
    color_scheme: "dark",
};

/// `"dark"` selects [`DARK`]; every other scheme (including `"light"`, `"vscode"`, and omitted)
/// selects [`LIGHT`].
pub fn theme_for_scheme(scheme: &str) -> &'static Theme {
    if scheme.eq_ignore_ascii_case("dark") {
        &DARK
    } else {
        &LIGHT
    }
}

/// Notation-neutral: all nodes share the same ink color (`strokeColorForNode` in theme.ts).
pub fn stroke_color_for_node(theme: &Theme) -> &'static str {
    theme.node_border
}

/// Notation-neutral: all edges share the same ink color; markers and dashes convey kind
/// (`strokeColorForEdge` in theme.ts).
pub fn stroke_color_for_edge(theme: &Theme) -> &'static str {
    theme.edge_default
}
