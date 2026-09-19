/// Mirrors `DiagramTheme` / `NOTATION_THEME_LIGHT` in
/// `vscode/diagram-renderer/src/theme.ts`. Only the light scheme is ported for this spike --
/// the golden-parity fixtures the headless export path checks against are always rendered with
/// `colorScheme: "light"`.
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
};

/// Notation-neutral: all nodes share the same ink color (`strokeColorForNode` in theme.ts).
pub fn stroke_color_for_node(theme: &Theme) -> &'static str {
    theme.node_border
}

/// Notation-neutral: all edges share the same ink color; markers and dashes convey kind
/// (`strokeColorForEdge` in theme.ts).
pub fn stroke_color_for_edge(theme: &Theme) -> &'static str {
    theme.edge_default
}
