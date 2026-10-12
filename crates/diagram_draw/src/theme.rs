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
    /// CSS font family of every text in the figure; empty leaves it to the viewer.
    pub font_family: &'static str,
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
    font_family: "",
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
    font_family: "",
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

/// The house style a document gives its figures: a font family and any of the theme's colours,
/// each by the name of its [`Theme`] field in camel case (`canvasBackground`, `nodeBorder`, ...).
/// What it leaves out stays as the base theme has it.
///
/// Text is measured by character count, not with the font, so a font much wider than the
/// viewer's default sans-serif can overrun its box.
#[derive(Clone, Debug, Default, PartialEq, Eq, serde::Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Style {
    #[serde(default)]
    pub font_family: Option<String>,
    #[serde(default)]
    pub colors: std::collections::BTreeMap<String, String>,
}

/// A style value is written into SVG `style` attributes, so it must not be able to end one.
fn style_value_is_safe(value: &str) -> bool {
    !value.trim().is_empty()
        && value
            .chars()
            .all(|c| c.is_alphanumeric() || " #(),.%-_'".contains(c))
}

/// Themes are `Copy` and hold `&'static str`. A style's values are interned, so each distinct
/// value is kept once for the life of the process however often a style is applied.
fn intern(value: &str) -> &'static str {
    use std::collections::HashSet;
    use std::sync::{Mutex, OnceLock};
    static INTERNED: OnceLock<Mutex<HashSet<&'static str>>> = OnceLock::new();
    let mut interned = INTERNED
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(existing) = interned.get(value) {
        return existing;
    }
    let leaked: &'static str = Box::leak(value.to_owned().into_boxed_str());
    interned.insert(leaked);
    leaked
}

impl Theme {
    /// This theme with `style` applied. An unknown colour name or a value that is not a plain
    /// CSS colour or font list is an error, not a silent default.
    pub fn with_style(mut self, style: &Style) -> Result<Theme, String> {
        if let Some(font_family) = &style.font_family {
            if !style_value_is_safe(font_family) {
                return Err(format!("`{font_family}` is not a usable font family"));
            }
            self.font_family = intern(font_family.trim());
        }
        for (name, value) in &style.colors {
            if !style_value_is_safe(value) {
                return Err(format!("`{value}` is not a usable colour for `{name}`"));
            }
            let slot = match name.as_str() {
                "canvasBackground" => &mut self.canvas_background,
                "panelBackground" => &mut self.panel_background,
                "nodeFill" => &mut self.node_fill,
                "nodeBorder" => &mut self.node_border,
                "textPrimary" => &mut self.text_primary,
                "textSecondary" => &mut self.text_secondary,
                "divider" => &mut self.divider,
                "highlight" => &mut self.highlight,
                "controlFill" => &mut self.control_fill,
                "controlStroke" => &mut self.control_stroke,
                "controlForeground" => &mut self.control_foreground,
                "controlHoverFill" => &mut self.control_hover_fill,
                "focusRing" => &mut self.focus_ring,
                "badgeFill" => &mut self.badge_fill,
                "badgeText" => &mut self.badge_text,
                "edgeDefault" => &mut self.edge_default,
                "frameStroke" => &mut self.frame_stroke,
                "frameText" => &mut self.frame_text,
                other => return Err(format!("`{other}` is not a colour of the diagram theme")),
            };
            *slot = intern(value.trim());
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_style_overrides_what_it_names_and_nothing_else() {
        let style: Style = serde_json::from_str(
            r##"{ "fontFamily": "Inter, sans-serif", "colors": { "nodeBorder": "#003366" } }"##,
        )
        .unwrap();
        let theme = LIGHT.with_style(&style).unwrap();
        assert_eq!(theme.font_family, "Inter, sans-serif");
        assert_eq!(theme.node_border, "#003366");
        assert_eq!(theme.node_fill, LIGHT.node_fill);
    }

    #[test]
    fn a_style_that_cannot_be_applied_is_an_error() {
        let colors = |name: &str, value: &str| Style {
            font_family: None,
            colors: [(name.to_owned(), value.to_owned())].into(),
        };
        assert!(LIGHT.with_style(&colors("nodeColour", "#000")).is_err());
        assert!(LIGHT
            .with_style(&colors("nodeFill", "red; stroke: blue"))
            .is_err());
        assert!(serde_json::from_str::<Style>(r#"{ "font": "x" }"#).is_err());
    }
}
