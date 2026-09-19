use std::collections::BTreeMap;

use serde_json::Value;

use crate::types::attr_str;

/// Mirrors `node-notation.ts`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotationRole {
    Definition,
    Usage,
    ReferenceUsage,
    Namespace,
    Annotation,
    Unsupported,
}

pub struct NodeChrome {
    pub is_definition: bool,
    pub is_container: bool,
    pub corner_radius: f64,
    /// `None` = solid stroke.
    pub stroke_dasharray: Option<&'static str>,
    pub structure_class: &'static str,
    /// CSS class suffix for the node's inner drawn `<g>` (`renderSysMLNode`'s own group), e.g.
    /// `" definition-node"`.
    pub node_class_suffix: &'static str,
}

pub struct NodeBodyChromeStyle {
    pub corner_radius: f64,
    pub stroke_dasharray: &'static str,
    pub stroke_width_px: f64,
}

pub fn notation_role_from_attributes(attributes: &BTreeMap<String, Value>) -> NotationRole {
    match attr_str(attributes, "notationRole").as_deref() {
        Some("definition") => return NotationRole::Definition,
        Some("usage") => return NotationRole::Usage,
        Some("reference-usage") => return NotationRole::ReferenceUsage,
        Some("namespace") => return NotationRole::Namespace,
        Some("annotation") => return NotationRole::Annotation,
        Some("unsupported") => return NotationRole::Unsupported,
        _ => {}
    }
    if matches!(attributes.get("isReference"), Some(Value::Bool(true))) {
        return NotationRole::ReferenceUsage;
    }
    if matches!(attributes.get("isDefinition"), Some(Value::Bool(true))) {
        return NotationRole::Definition;
    }
    NotationRole::Unsupported
}

#[derive(Default)]
pub struct ChromeOptions {
    pub is_container: Option<bool>,
    pub is_package_container: bool,
}

pub fn resolve_node_chrome(role: NotationRole, opts: ChromeOptions) -> NodeChrome {
    let is_container = opts.is_container.unwrap_or(role == NotationRole::Namespace);

    if is_container {
        return NodeChrome {
            is_definition: false,
            is_container: true,
            corner_radius: 8.0,
            stroke_dasharray: if opts.is_package_container {
                None
            } else {
                Some("4,4")
            },
            structure_class: "viz-node--container",
            node_class_suffix: "",
        };
    }

    match role {
        NotationRole::ReferenceUsage => NodeChrome {
            is_definition: false,
            is_container: false,
            corner_radius: 8.0,
            stroke_dasharray: Some("2,4"),
            structure_class: "viz-node--reference",
            node_class_suffix: " reference-node",
        },
        NotationRole::Definition => NodeChrome {
            is_definition: true,
            is_container: false,
            corner_radius: 0.0,
            stroke_dasharray: None,
            structure_class: "viz-node--definition",
            node_class_suffix: " definition-node",
        },
        NotationRole::Unsupported => NodeChrome {
            is_definition: false,
            is_container: false,
            corner_radius: 4.0,
            stroke_dasharray: Some("3,3"),
            structure_class: "viz-node--unsupported",
            node_class_suffix: " unsupported-node",
        },
        _ => NodeChrome {
            is_definition: false,
            is_container: false,
            corner_radius: 8.0,
            stroke_dasharray: None,
            structure_class: "viz-node--usage",
            node_class_suffix: " usage-node",
        },
    }
}

/// Resolved stroke dash for a node body rect (package containers stay solid).
fn node_body_stroke_dasharray(chrome: &NodeChrome, is_package_container: bool) -> &'static str {
    if chrome.is_container && is_package_container {
        return "none";
    }
    chrome.stroke_dasharray.unwrap_or("none")
}

#[derive(Default)]
pub struct NodeBodyOptions {
    pub selected: bool,
    pub is_container: Option<bool>,
    pub is_package_container: bool,
    /// General view uses slightly heavier definition borders.
    pub general_view: bool,
}

/// Shared body rect metrics for general and interconnection node backgrounds.
pub fn node_body_chrome_style(chrome: &NodeChrome, opts: NodeBodyOptions) -> NodeBodyChromeStyle {
    let is_container = opts.is_container.unwrap_or(chrome.is_container);
    let stroke_width_px = if opts.selected {
        3.0
    } else if is_container {
        1.5
    } else if opts.general_view {
        if chrome.is_definition {
            2.0
        } else {
            1.5
        }
    } else if chrome.is_definition {
        2.0
    } else {
        3.0
    };

    NodeBodyChromeStyle {
        corner_radius: chrome.corner_radius,
        stroke_dasharray: node_body_stroke_dasharray(chrome, opts.is_package_container),
        stroke_width_px,
    }
}

/// Fill region for the header compartment: follows the node's own top corners concentrically and
/// stops on a straight edge at `header_bottom`, entirely inside the body stroke.
pub fn header_fill_path(
    width: f64,
    header_bottom: f64,
    radius: f64,
    stroke_width_px: f64,
) -> String {
    let inset = stroke_width_px / 2.0;
    let left = inset;
    let top = inset;
    let right = (width - inset).max(left);
    let bottom = (header_bottom - inset).max(top);
    let r = (radius - inset)
        .min((right - left) / 2.0)
        .min(bottom - top)
        .max(0.0);
    if r <= 0.0 {
        return format!(
            "M{left},{top}H{right}V{bottom}H{left}Z",
            left = n(left),
            top = n(top),
            right = n(right),
            bottom = n(bottom)
        );
    }
    [
        format!("M{},{}", n(left + r), n(top)),
        format!("H{}", n(right - r)),
        format!("A{},{} 0 0 1 {},{}", n(r), n(r), n(right), n(top + r)),
        format!("V{}", n(bottom)),
        format!("H{}", n(left)),
        format!("V{}", n(top + r)),
        format!("A{},{} 0 0 1 {},{}", n(r), n(r), n(left + r), n(top)),
        "Z".to_string(),
    ]
    .join("")
}

/// Horizontal extent a compartment divider may span without crossing the body stroke.
pub fn node_inner_span(width: f64, stroke_width_px: f64) -> (f64, f64) {
    let inset = stroke_width_px / 2.0;
    (inset, (width - inset).max(inset))
}

fn n(value: f64) -> String {
    crate::svg::format_number(value)
}
