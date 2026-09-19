//! Port of the General-View-relevant parts of `sysml-node-builder.ts`: compartment collection,
//! header/compartment geometry, and SVG drawing (`renderSysMLNode`). Sizing-time-only functions
//! (`computeNodeWidth`/`computeNodeHeight`, used to build ELK layout input) are intentionally not
//! ported -- this spike draws from an already-laid-out graph, so node width/height are given.

use std::collections::BTreeMap;

use serde::Deserialize;
use serde_json::Value;

use crate::node_notation;
use crate::svg::{format_number as n, Element};
use crate::theme::Theme;
use crate::types::{attr_str, attr_u32};

pub const LINE_HEIGHT: f64 = 12.0;
pub const COMPARTMENT_LABEL_HEIGHT: f64 = 15.0;
pub const COMPARTMENT_PADDING: f64 = 4.0;
pub const PADDING: f64 = 8.0;
pub const HEADER_PADDING_X: f64 = 8.0;
pub const HEADER_PADDING_TOP: f64 = 7.0;
pub const HEADER_PADDING_BOTTOM: f64 = 7.0;
pub const STEREOTYPE_FONT_SIZE: f64 = 9.0;
pub const STEREOTYPE_LINE_HEIGHT: f64 = 11.0;
pub const NAME_FONT_SIZE: f64 = 11.0;
pub const NAME_LINE_HEIGHT: f64 = 14.0;
pub const NAME_MAX_LINES: usize = 2;
pub const TYPING_FONT_SIZE: f64 = 9.5;
pub const TYPING_LINE_HEIGHT: f64 = 12.0;
pub const COMPARTMENT_FONT_SIZE: f64 = 9.0;
pub const DISCLOSURE_TARGET_SIZE: f64 = 24.0;
/// Painted size of the +/- disclosure box inside a disclosure target.
pub const DISCLOSURE_BOX_SIZE: f64 = 13.0;
pub const BADGE_HEIGHT: f64 = 15.0;
pub const CONTROL_EDGE_INSET: f64 = 3.0;
pub const BADGE_MIN_WIDTH: f64 = 18.0;
const OVERFLOW_LINE_HEIGHT: f64 = 12.0;
const AVERAGE_GLYPH_RATIO_REGULAR: f64 = 0.54;

/// General View nodes cap each compartment at this many rows and show a `+n more` line
/// (`GENERAL_NODE_CONFIG` in `render/drawing.ts`).
pub const MAX_LINES_PER_COMPARTMENT: usize = 8;

#[derive(Debug, Clone)]
pub struct DetailItem {
    pub display_text: String,
    pub declared_in: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Section {
    pub key: String,
    pub title: String,
    pub items: Vec<DetailItem>,
    pub collapsed: bool,
}

/// Deserializable mirror of TS's `SysMLNodeCompartments` JSON shape (the precomputed
/// `LaidOutNode.compartments` field). `Compartments` derives `Deserialize` via `#[serde(from =
/// ...)]` so a `LaidOutNode` can carry either the precomputed value or fall back to
/// `collect_compartments`, matching `d.compartments ?? collectCompartments(d)` in `drawing.ts`.
#[derive(Deserialize)]
struct DetailItemWire {
    #[serde(rename = "displayText")]
    display_text: String,
    #[serde(rename = "declaredIn", default)]
    declared_in: Option<String>,
}

impl From<DetailItemWire> for DetailItem {
    fn from(wire: DetailItemWire) -> Self {
        DetailItem {
            display_text: wire.display_text,
            declared_in: wire.declared_in.filter(|s| !s.is_empty()),
        }
    }
}

#[derive(Deserialize)]
struct SectionWire {
    key: String,
    title: String,
    #[serde(default)]
    items: Vec<DetailItemWire>,
    collapsed: bool,
}

impl From<SectionWire> for Section {
    fn from(wire: SectionWire) -> Self {
        Section {
            key: wire.key,
            title: wire.title,
            items: wire.items.into_iter().map(Into::into).collect(),
            collapsed: wire.collapsed,
        }
    }
}

#[derive(Deserialize)]
struct CompartmentsHeaderWire {
    stereotype: String,
    name: String,
}

#[derive(Deserialize)]
struct CompartmentsWire {
    header: CompartmentsHeaderWire,
    #[serde(rename = "typedByName", default)]
    typed_by_name: Option<String>,
    #[serde(default)]
    attributes: Vec<DetailItemWire>,
    #[serde(default)]
    parts: Vec<DetailItemWire>,
    #[serde(default)]
    ports: Vec<DetailItemWire>,
    #[serde(rename = "collapsibleSections", default)]
    collapsible_sections: Vec<SectionWire>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(from = "CompartmentsWire")]
pub struct Compartments {
    pub stereotype: String,
    pub name: String,
    pub typed_by_name: Option<String>,
    pub attributes: Vec<DetailItem>,
    pub parts: Vec<DetailItem>,
    pub ports: Vec<DetailItem>,
    pub collapsible_sections: Vec<Section>,
}

impl From<CompartmentsWire> for Compartments {
    fn from(wire: CompartmentsWire) -> Self {
        Compartments {
            stereotype: wire.header.stereotype,
            name: wire.header.name,
            typed_by_name: wire.typed_by_name.filter(|s| !s.is_empty()),
            attributes: wire.attributes.into_iter().map(Into::into).collect(),
            parts: wire.parts.into_iter().map(Into::into).collect(),
            ports: wire.ports.into_iter().map(Into::into).collect(),
            collapsible_sections: wire
                .collapsible_sections
                .into_iter()
                .map(Into::into)
                .collect(),
        }
    }
}

fn normalize_unit_brackets(text: &str) -> String {
    let mut out = text.to_string();
    while out.contains("[[") {
        let replaced = out.replace("[[", "[").replace("]]", "]");
        if replaced == out {
            break;
        }
        out = replaced;
    }
    out
}

fn normalize_detail_item(value: &Value) -> Option<DetailItem> {
    match value {
        Value::String(s) => {
            let text = normalize_unit_brackets(s.trim());
            if text.is_empty() {
                None
            } else {
                Some(DetailItem {
                    display_text: text,
                    declared_in: None,
                })
            }
        }
        Value::Object(map) => {
            let name = map
                .get("name")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim()
                .to_string();
            let display_text = normalize_unit_brackets(
                map.get("displayText")
                    .and_then(Value::as_str)
                    .unwrap_or(&name)
                    .trim(),
            );
            if display_text.is_empty() {
                return None;
            }
            let declared_in = map
                .get("declaredIn")
                .and_then(Value::as_str)
                .map(str::to_string)
                .filter(|s| !s.is_empty());
            Some(DetailItem {
                display_text,
                declared_in,
            })
        }
        _ => None,
    }
}

fn detail_items(attributes: &BTreeMap<String, Value>, key: &str) -> Vec<DetailItem> {
    attributes
        .get(key)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(normalize_detail_item)
        .collect()
}

/// Port of `collectCompartments` in `sysml-node-builder.ts` (the `{label, kind, attributes}`
/// overload `drawNodes` falls back to when a node has no precomputed `compartments`).
pub fn collect_compartments(
    label: &str,
    kind: &str,
    attributes: &BTreeMap<String, Value>,
) -> Compartments {
    let typed_by_name = attr_str(attributes, "typedByName")
        .or_else(|| attr_str(attributes, "partType"))
        .or_else(|| attr_str(attributes, "type"))
        .or_else(|| attr_str(attributes, "typedBy"))
        .or_else(|| attr_str(attributes, "typing"));

    let direct_attributes = detail_items(attributes, "generalViewDirectAttributes");
    let direct_parts = detail_items(attributes, "generalViewDirectParts");
    let direct_ports = detail_items(attributes, "generalViewDirectPorts");
    let inherited_attributes = detail_items(attributes, "generalViewInheritedAttributes");
    let inherited_parts = detail_items(attributes, "generalViewInheritedParts");
    let mut package_members = detail_items(attributes, "generalViewPackageMembers");
    package_members.extend(detail_items(attributes, "packageMembers"));
    package_members.extend(detail_items(attributes, "members"));
    let mut imports = detail_items(attributes, "generalViewImports");
    imports.extend(detail_items(attributes, "imports"));

    let mut collapsible_sections = Vec::new();
    if let Some(typed_compartments) = attributes
        .get("typedCompartments")
        .and_then(Value::as_array)
    {
        for raw in typed_compartments {
            let Some(compartment) = raw.as_object() else {
                continue;
            };
            let kind = compartment
                .get("kind")
                .and_then(Value::as_str)
                .unwrap_or("members");
            let inherited =
                compartment.get("provenance").and_then(Value::as_str) == Some("inherited");
            let items: Vec<DetailItem> = compartment
                .get("members")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|member| {
                    let record = member.as_object();
                    let name = record
                        .and_then(|r| r.get("name"))
                        .and_then(Value::as_str)
                        .unwrap_or("Unnamed");
                    let type_name = record
                        .and_then(|r| r.get("typeName"))
                        .and_then(Value::as_str)
                        .unwrap_or("");
                    let display_text = if type_name.is_empty() {
                        name.to_string()
                    } else {
                        format!("{name} : {type_name}")
                    };
                    normalize_detail_item(&Value::String(display_text))
                })
                .collect();
            if !items.is_empty() {
                let title = if kind.is_empty() {
                    "Members".to_string()
                } else {
                    let mut c = kind.chars();
                    c.next()
                        .map(|f| f.to_uppercase().collect::<String>() + c.as_str())
                        .unwrap_or_default()
                };
                collapsible_sections.push(Section {
                    key: format!(
                        "{}-{}",
                        if inherited { "inherited" } else { "direct" },
                        kind
                    ),
                    title,
                    items,
                    collapsed: inherited,
                });
            }
        }
    }
    if !inherited_attributes.is_empty() {
        collapsible_sections.push(Section {
            key: "inherited-attributes".into(),
            title: "Attributes".into(),
            items: inherited_attributes,
            collapsed: true,
        });
    }
    if !inherited_parts.is_empty() {
        collapsible_sections.push(Section {
            key: "inherited-parts".into(),
            title: "Parts".into(),
            items: inherited_parts,
            collapsed: true,
        });
    }
    if !package_members.is_empty() {
        collapsible_sections.push(Section {
            key: "package-members".into(),
            title: "Members".into(),
            items: package_members,
            collapsed: false,
        });
    }
    if !imports.is_empty() {
        collapsible_sections.push(Section {
            key: "imports".into(),
            title: "Imports".into(),
            items: imports,
            collapsed: true,
        });
    }

    // Renderer-owned presentation state: a compartment the viewer has opened or closed overrides
    // the default provenance-derived collapse. It never changes which members exist.
    if let Some(section_state) = attributes
        .get("compartmentSectionState")
        .and_then(Value::as_object)
    {
        for section in &mut collapsible_sections {
            if let Some(expanded) = section_state
                .get(section.key.as_str())
                .and_then(Value::as_bool)
            {
                section.collapsed = !expanded;
            }
        }
    }

    let fallback = |key: &str| detail_items(attributes, key);
    Compartments {
        stereotype: if kind.is_empty() {
            "element".to_string()
        } else {
            kind.to_string()
        },
        name: if label.is_empty() {
            "Unnamed".to_string()
        } else {
            label.to_string()
        },
        typed_by_name,
        attributes: if !direct_attributes.is_empty() {
            direct_attributes
        } else {
            fallback("attributes")
        },
        parts: if !direct_parts.is_empty() {
            direct_parts
        } else {
            fallback("parts")
        },
        ports: if !direct_ports.is_empty() {
            direct_ports
        } else {
            fallback("ports")
        },
        collapsible_sections,
    }
}

pub struct NodeChromeState {
    pub disclosure: Option<&'static str>,
    pub hidden_relationship_count: u32,
}

/// Port of `nodeChromeStateFromAttributes`.
pub fn node_chrome_state_from_attributes(attributes: &BTreeMap<String, Value>) -> NodeChromeState {
    let disclosure = match attr_str(attributes, "disclosure").as_deref() {
        Some("expanded") => Some("expanded"),
        Some("collapsed") => Some("collapsed"),
        _ => None,
    };
    NodeChromeState {
        disclosure,
        hidden_relationship_count: attr_u32(attributes, "hiddenRelationshipCount").unwrap_or(0),
    }
}

fn max_chars(available_width: f64, font_size: f64, ratio: f64) -> usize {
    ((available_width / (font_size * ratio)).floor() as i64).max(1) as usize
}

/// Deterministic ellipsis truncation; callers keep the untruncated text in a `<title>`.
pub fn truncate_to_chars(value: &str, limit: usize) -> String {
    let chars: Vec<char> = value.chars().collect();
    if chars.len() <= limit {
        return value.to_string();
    }
    if limit <= 1 {
        return "…".to_string();
    }
    format!("{}…", chars[..limit - 1].iter().collect::<String>())
}

/// Split a long element name over at most `max_lines` lines, breaking at separators and
/// camel-case boundaries before falling back to a hard break.
pub fn wrap_element_name(name: &str, limit: usize, max_lines: usize) -> Vec<String> {
    let text = name.trim();
    if text.is_empty() {
        return vec![String::new()];
    }
    let chars: Vec<char> = text.chars().collect();
    if chars.len() <= limit || max_lines <= 1 {
        return vec![truncate_to_chars(text, limit)];
    }

    let mut segments: Vec<String> = Vec::new();
    let mut current = String::new();
    for index in 0..chars.len() {
        let ch = chars[index];
        let next = chars.get(index + 1);
        current.push(ch);
        let separator = matches!(ch, ' ' | '_' | '.' | ':' | '-');
        let camel_boundary = (ch.is_ascii_lowercase() || ch.is_ascii_digit())
            && next.is_some_and(|n| n.is_ascii_uppercase());
        if separator || camel_boundary {
            segments.push(std::mem::take(&mut current));
        }
    }
    if !current.is_empty() {
        segments.push(current);
    }

    let mut lines: Vec<String> = Vec::new();
    let mut index = 0;
    while index < segments.len() && lines.len() < max_lines {
        let mut line = segments[index].clone();
        index += 1;
        while index < segments.len()
            && line.chars().count() + segments[index].chars().count() <= limit
        {
            line.push_str(&segments[index]);
            index += 1;
        }
        lines.push(line.trim_end().to_string());
    }
    let overflowed = index < segments.len();
    let mut rendered: Vec<String> = lines
        .iter()
        .map(|line| truncate_to_chars(line, limit))
        .collect();
    if overflowed {
        if let Some(last) = lines.last() {
            let last_index = rendered.len() - 1;
            rendered[last_index] = truncate_to_chars(&format!("{last}…"), limit);
        }
    }
    rendered
}

fn badge_width_for(text: &str) -> f64 {
    (BADGE_MIN_WIDTH).max(10.0 + text.chars().count() as f64 * 6.0)
}

pub struct Region {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct HeaderLayout {
    pub height: f64,
    pub center_x: f64,
    pub stereotype_baseline: f64,
    pub name_lines: Vec<String>,
    pub name_baselines: Vec<f64>,
    pub typing_text: Option<String>,
    pub typing_baseline: Option<f64>,
    pub disclosure_target: Option<Region>,
    pub badge: Option<(Region, String)>,
}

/// Port of `layoutNodeHeader`.
pub fn layout_node_header(
    compartments: &Compartments,
    width: f64,
    state: &NodeChromeState,
) -> HeaderLayout {
    let hidden_count = state.hidden_relationship_count;
    let badge_text = if hidden_count > 0 {
        Some(hidden_count.to_string())
    } else {
        None
    };
    let has_disclosure = matches!(state.disclosure, Some("expanded") | Some("collapsed"));

    let control_reserve = if has_disclosure {
        CONTROL_EDGE_INSET + DISCLOSURE_TARGET_SIZE + 2.0
    } else {
        0.0
    };
    let badge_reserve = badge_text
        .as_ref()
        .map(|t| CONTROL_EDGE_INSET + badge_width_for(t) + 2.0)
        .unwrap_or(0.0);
    let gutter = control_reserve.max(badge_reserve).max(HEADER_PADDING_X);
    let text_left = gutter;
    let text_right = (width - gutter).max(text_left + 1.0);
    let available_text = text_right - text_left;

    let name_lines = wrap_element_name(
        &compartments.name,
        max_chars(available_text, NAME_FONT_SIZE, 0.6),
        NAME_MAX_LINES,
    );
    let typing_text = compartments.typed_by_name.as_ref().map(|typed| {
        let raw = format!(": {typed}");
        truncate_to_chars(
            &raw,
            max_chars(
                available_text,
                TYPING_FONT_SIZE,
                AVERAGE_GLYPH_RATIO_REGULAR,
            ),
        )
    });

    let content_height = HEADER_PADDING_TOP
        + STEREOTYPE_LINE_HEIGHT
        + name_lines.len() as f64 * NAME_LINE_HEIGHT
        + if typing_text.is_some() {
            TYPING_LINE_HEIGHT
        } else {
            0.0
        }
        + HEADER_PADDING_BOTTOM;
    let height = content_height.max(DISCLOSURE_TARGET_SIZE + 4.0);

    let stereotype_baseline = HEADER_PADDING_TOP + STEREOTYPE_LINE_HEIGHT - 3.0;
    let name_baselines: Vec<f64> = (0..name_lines.len())
        .map(|index| {
            HEADER_PADDING_TOP + STEREOTYPE_LINE_HEIGHT + (index + 1) as f64 * NAME_LINE_HEIGHT
                - 4.0
        })
        .collect();
    let typing_baseline = typing_text.as_ref().map(|_| {
        HEADER_PADDING_TOP
            + STEREOTYPE_LINE_HEIGHT
            + name_lines.len() as f64 * NAME_LINE_HEIGHT
            + TYPING_LINE_HEIGHT
            - 3.0
    });

    let badge = badge_text.map(|text| {
        let badge_width = badge_width_for(&text);
        (
            Region {
                x: width - CONTROL_EDGE_INSET - badge_width,
                y: (height - BADGE_HEIGHT) / 2.0,
                width: badge_width,
                height: BADGE_HEIGHT,
            },
            text,
        )
    });

    let disclosure_target = has_disclosure.then(|| Region {
        x: CONTROL_EDGE_INSET,
        y: (height - DISCLOSURE_TARGET_SIZE) / 2.0,
        width: DISCLOSURE_TARGET_SIZE,
        height: DISCLOSURE_TARGET_SIZE,
    });

    HeaderLayout {
        height,
        center_x: width / 2.0,
        stereotype_baseline,
        name_lines,
        name_baselines,
        typing_text,
        typing_baseline,
        disclosure_target,
        badge,
    }
}

pub struct CompartmentBlockLayout {
    pub key: String,
    pub title: String,
    pub collapsible: bool,
    pub collapsed: bool,
    pub total_items: usize,
    pub shown_items: Vec<DetailItem>,
    pub overflow_count: usize,
    pub divider_y: f64,
    pub label_baseline: f64,
    pub label_text_x: f64,
    pub item_baselines: Vec<f64>,
    pub overflow_baseline: Option<f64>,
    /// Full-width pointer target for a collapsible compartment's label row.
    pub label_region: Region,
    pub disclosure_box: Option<Region>,
}

struct SectionRef<'a> {
    key: &'a str,
    title: &'a str,
    items: &'a [DetailItem],
    collapsible: bool,
    collapsed: bool,
}

fn compartment_sections(compartments: &Compartments) -> Vec<SectionRef<'_>> {
    let mut sections = Vec::new();
    if !compartments.attributes.is_empty() {
        sections.push(SectionRef {
            key: "attributes",
            title: "Attributes",
            items: &compartments.attributes,
            collapsible: false,
            collapsed: false,
        });
    }
    if !compartments.parts.is_empty() {
        sections.push(SectionRef {
            key: "parts",
            title: "Parts",
            items: &compartments.parts,
            collapsible: false,
            collapsed: false,
        });
    }
    if !compartments.ports.is_empty() {
        sections.push(SectionRef {
            key: "ports",
            title: "Ports",
            items: &compartments.ports,
            collapsible: false,
            collapsed: false,
        });
    }
    for section in &compartments.collapsible_sections {
        if !section.items.is_empty() {
            sections.push(SectionRef {
                key: &section.key,
                title: &section.title,
                items: &section.items,
                collapsible: true,
                collapsed: section.collapsed,
            });
        }
    }
    sections
}

/// Port of `layoutSysMLNode`'s compartment-block pass (header layout is `layout_node_header`).
pub fn layout_compartment_blocks(
    compartments: &Compartments,
    width: f64,
    header_height: f64,
    stroke_width_px: f64,
) -> (Vec<CompartmentBlockLayout>, f64) {
    let inset = stroke_width_px / 2.0;
    let mut blocks = Vec::new();
    let mut cursor = header_height;
    for section in compartment_sections(compartments) {
        let limit = section.items.len().min(MAX_LINES_PER_COMPARTMENT);
        let shown_items: Vec<DetailItem> = if section.collapsed {
            Vec::new()
        } else {
            section.items[..limit].to_vec()
        };
        let overflow_count = if section.collapsed {
            0
        } else {
            section.items.len() - shown_items.len()
        };
        let label_top = cursor + COMPARTMENT_PADDING;
        let label_baseline = label_top + COMPARTMENT_LABEL_HEIGHT - 5.0;
        let disclosure_box = section.collapsible.then(|| Region {
            x: PADDING,
            y: label_top + (COMPARTMENT_LABEL_HEIGHT - DISCLOSURE_BOX_SIZE) / 2.0 - 1.0,
            width: DISCLOSURE_BOX_SIZE,
            height: DISCLOSURE_BOX_SIZE,
        });
        let label_text_x = if section.collapsible {
            PADDING + DISCLOSURE_BOX_SIZE + 5.0
        } else {
            PADDING
        };
        let item_top = label_top + COMPARTMENT_LABEL_HEIGHT;
        let item_baselines: Vec<f64> = (0..shown_items.len())
            .map(|index| item_top + (index + 1) as f64 * LINE_HEIGHT - 3.0)
            .collect();
        let overflow_baseline = if overflow_count > 0 {
            Some(item_top + shown_items.len() as f64 * LINE_HEIGHT + OVERFLOW_LINE_HEIGHT - 3.0)
        } else {
            None
        };
        let height = COMPARTMENT_PADDING
            + COMPARTMENT_LABEL_HEIGHT
            + shown_items.len() as f64 * LINE_HEIGHT
            + if overflow_count > 0 {
                OVERFLOW_LINE_HEIGHT
            } else {
                0.0
            }
            + COMPARTMENT_PADDING;
        let label_region = Region {
            x: inset + CONTROL_EDGE_INSET,
            y: cursor + 1.0,
            width: (width - (inset + CONTROL_EDGE_INSET) * 2.0).max(0.0),
            height: COMPARTMENT_PADDING + COMPARTMENT_LABEL_HEIGHT,
        };
        blocks.push(CompartmentBlockLayout {
            key: section.key.to_string(),
            title: section.title.to_string(),
            collapsible: section.collapsible,
            collapsed: section.collapsed,
            total_items: section.items.len(),
            shown_items,
            overflow_count,
            divider_y: cursor,
            label_baseline,
            label_text_x,
            item_baselines,
            overflow_baseline,
            label_region,
            disclosure_box,
        });
        cursor += height;
    }
    (
        blocks,
        cursor.max(header_height).max(DISCLOSURE_TARGET_SIZE + 4.0),
    )
}

/// `+`/`-` glyph inside a disclosure box, drawn as filled bars so it stays legible at any zoom.
/// Port of `disclosureGlyphPaths` in `sysml-node-builder.ts`.
fn disclosure_glyph_paths(box_region: &Region, expanded: bool) -> Vec<String> {
    let thickness = (box_region.width * 0.14).max(1.4);
    let arm = box_region.width * 0.54;
    let cx = box_region.x + box_region.width / 2.0;
    let cy = box_region.y + box_region.height / 2.0;
    let horizontal = format!(
        "M{},{}h{}v{}h{}Z",
        n(cx - arm / 2.0),
        n(cy - thickness / 2.0),
        n(arm),
        n(thickness),
        n(-arm)
    );
    if expanded {
        return vec![horizontal];
    }
    let vertical = format!(
        "M{},{}v{}h{}v{}Z",
        n(cx - thickness / 2.0),
        n(cy - arm / 2.0),
        n(arm),
        n(thickness),
        n(-arm)
    );
    vec![horizontal, vertical]
}

const NOTATION_KEYWORDS: &[(&str, &str)] = &[
    ("PartDefinition", "part def"),
    ("PartUsage", "part"),
    ("PortDefinition", "port def"),
    ("PortUsage", "port"),
    ("AttributeDefinition", "attribute def"),
    ("AttributeUsage", "attribute"),
    ("ItemDefinition", "item def"),
    ("ItemUsage", "item"),
    ("OccurrenceDefinition", "occurrence def"),
    ("OccurrenceUsage", "occurrence"),
    ("ConnectionDefinition", "connection def"),
    ("ConnectionUsage", "connection"),
    ("InterfaceDefinition", "interface def"),
    ("InterfaceUsage", "interface"),
    ("AllocationDefinition", "allocation def"),
    ("AllocationUsage", "allocation"),
    ("ActionDefinition", "action def"),
    ("ActionUsage", "action"),
    ("StateDefinition", "state def"),
    ("StateUsage", "state"),
    ("CalculationDefinition", "calc def"),
    ("CalculationUsage", "calc"),
    ("ConstraintDefinition", "constraint def"),
    ("ConstraintUsage", "constraint"),
    ("RequirementDefinition", "requirement def"),
    ("RequirementUsage", "requirement"),
    ("ConcernDefinition", "concern def"),
    ("ConcernUsage", "concern"),
    ("CaseDefinition", "case def"),
    ("CaseUsage", "case"),
    ("AnalysisCaseDefinition", "analysis def"),
    ("AnalysisCaseUsage", "analysis"),
    ("VerificationCaseDefinition", "verification def"),
    ("VerificationCaseUsage", "verification"),
    ("UseCaseDefinition", "use case def"),
    ("UseCaseUsage", "use case"),
    ("ViewDefinition", "view def"),
    ("ViewUsage", "view"),
    ("ViewpointDefinition", "viewpoint def"),
    ("ViewpointUsage", "viewpoint"),
    ("RenderingDefinition", "rendering def"),
    ("RenderingUsage", "rendering"),
    ("MetadataDefinition", "metadata def"),
    ("MetadataUsage", "metadata"),
    ("ReferenceUsage", "ref"),
];

pub fn format_stereotype(kind: &str) -> String {
    let keyword = NOTATION_KEYWORDS
        .iter()
        .find(|(k, _)| *k == kind)
        .map(|(_, v)| *v);
    let text = keyword
        .map(str::to_string)
        .unwrap_or_else(|| kind.replace('_', " "));
    format!("«{text}»")
}

pub struct RenderNodeOptions<'a> {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub node_class: String,
    pub data_element_name: &'a str,
    pub stroke_color: &'a str,
    pub selected: bool,
    pub chrome: crate::node_notation::NodeChrome,
    pub theme: &'a Theme,
    pub state: NodeChromeState,
}

/// Port of `renderSysMLNode`'s General-View output, including the interactive-*looking* disclosure
/// chrome. That chrome is not actually gated on interactivity: `renderVisualization`'s General
/// View branch builds its own `DisclosureActions` unconditionally (`const disclosure = {
/// toggleNode, toggleSection }; const generalOptions = { ...options, disclosure };` in
/// `renderer.ts`) and uses it for every render `redrawGeneral` performs, including the very first
/// one -- so `drawNodes`' `options.disclosure`/`compartmentDisclosure` are never actually null for
/// General View, headless export included. Only the click/keydown *handlers* are real-interaction-
/// only, and handlers never appear in serialized SVG text regardless. So both the node-level
/// toggle (`general-node-toggle`, gated on `header.disclosure_target`) and the compartment-level
/// toggle chrome (gated on `block.collapsible`) are drawn unconditionally here.
pub fn render_node(compartments: &Compartments, options: RenderNodeOptions<'_>) -> Element {
    let theme = options.theme;
    let body = node_notation::node_body_chrome_style(
        &options.chrome,
        node_notation::NodeBodyOptions {
            selected: options.selected,
            general_view: true,
            ..Default::default()
        },
    );
    let header = layout_node_header(compartments, options.width, &options.state);
    let (blocks, _total_height) = layout_compartment_blocks(
        compartments,
        options.width,
        header.height,
        body.stroke_width_px,
    );
    let show_header_fill = !blocks.is_empty();
    let (span_x1, span_x2) = node_notation::node_inner_span(options.width, body.stroke_width_px);

    let mut node = Element::new("g")
        .attr(
            "class",
            format!(
                "{}{}{}",
                options.node_class,
                options.chrome.node_class_suffix,
                if options.selected { " is-selected" } else { "" }
            ),
        )
        .attr(
            "transform",
            format!("translate({},{})", n(options.x), n(options.y)),
        )
        .attr("data-element-name", options.data_element_name);

    node = node.child(
        Element::new("rect")
            .attr_f("width", options.width)
            .attr_f("height", options.height)
            .attr_f("rx", body.corner_radius)
            .attr("class", "graph-node-background sysml-node-bg")
            .attr("data-original-stroke", options.stroke_color)
            .attr(
                "data-original-width",
                format!("{}px", n(body.stroke_width_px)),
            )
            .style("fill", theme.node_fill)
            .style(
                "stroke",
                if options.selected {
                    theme.highlight
                } else {
                    options.stroke_color
                },
            )
            .style("stroke-width", format!("{}px", n(body.stroke_width_px)))
            .style("stroke-dasharray", body.stroke_dasharray),
    );

    if show_header_fill {
        node = node.child(
            Element::new("path")
                .attr("class", "sysml-header-compartment")
                .attr(
                    "d",
                    node_notation::header_fill_path(
                        options.width,
                        header.height,
                        body.corner_radius,
                        body.stroke_width_px,
                    ),
                )
                .style("fill", theme.panel_background),
        );
    }

    node = node.child(
        Element::new("text")
            .attr("class", "sysml-node-stereotype")
            .attr_f("x", header.center_x)
            .attr_f("y", header.stereotype_baseline)
            .attr("text-anchor", "middle")
            .text(format_stereotype(&compartments.stereotype))
            .style("font-size", format!("{}px", n(STEREOTYPE_FONT_SIZE)))
            .style("fill", theme.text_secondary),
    );

    let mut name_group = Element::new("g").attr("class", "node-name-text viz-node-name");
    for (index, line) in header.name_lines.iter().enumerate() {
        name_group = name_group.child(
            Element::new("text")
                .attr_f("x", header.center_x)
                .attr_f("y", header.name_baselines[index])
                .attr("text-anchor", "middle")
                .text(line.clone())
                .style("font-size", format!("{}px", n(NAME_FONT_SIZE)))
                .style("font-weight", "600")
                .style("fill", theme.text_primary),
        );
    }
    name_group = name_group.child(Element::new("title").text(compartments.name.clone()));
    node = node.child(name_group);

    if let (Some(typing_text), Some(typing_baseline)) =
        (&header.typing_text, header.typing_baseline)
    {
        let typing = Element::new("text")
            .attr("class", "sysml-node-typing")
            .attr_f("x", header.center_x)
            .attr_f("y", typing_baseline)
            .attr("text-anchor", "middle")
            .text(typing_text.clone())
            .style("font-size", format!("{}px", n(TYPING_FONT_SIZE)))
            .style("font-style", "italic")
            .style("fill", theme.text_secondary)
            .child(Element::new("title").text(format!(
                ": {}",
                compartments.typed_by_name.clone().unwrap_or_default()
            )));
        node = node.child(typing);
    }

    if let Some(disclosure_target) = &header.disclosure_target {
        let expanded = options.state.disclosure == Some("expanded");
        let box_region = Region {
            x: disclosure_target.x + (disclosure_target.width - DISCLOSURE_BOX_SIZE) / 2.0,
            y: disclosure_target.y + (disclosure_target.height - DISCLOSURE_BOX_SIZE) / 2.0,
            width: DISCLOSURE_BOX_SIZE,
            height: DISCLOSURE_BOX_SIZE,
        };
        let verb = if expanded { "Collapse" } else { "Expand" };
        let tooltip = if expanded {
            format!(
                "Collapse {}: hide its nested elements and their relationships.",
                options.data_element_name
            )
        } else {
            format!(
                "Expand {}: show its nested elements and their relationships.",
                options.data_element_name
            )
        };
        let mut control = Element::new("g")
            .attr("class", "general-node-toggle sysml-disclosure")
            .attr("role", "button")
            .attr("tabindex", "0")
            .attr(
                "aria-label",
                format!("{verb} {}", options.data_element_name),
            )
            .attr("aria-expanded", if expanded { "true" } else { "false" })
            .attr(
                "data-disclosure-state",
                if expanded { "expanded" } else { "collapsed" },
            )
            .child(Element::new("title").text(tooltip))
            .child(
                Element::new("rect")
                    .attr("class", "sysml-disclosure-target")
                    .attr_f("x", disclosure_target.x)
                    .attr_f("y", disclosure_target.y)
                    .attr_f("width", disclosure_target.width)
                    .attr_f("height", disclosure_target.height)
                    .attr("rx", "4")
                    .style("fill", "transparent")
                    .style("pointer-events", "all"),
            )
            .child(
                Element::new("rect")
                    .attr("class", "sysml-disclosure-box")
                    .attr_f("x", box_region.x)
                    .attr_f("y", box_region.y)
                    .attr_f("width", box_region.width)
                    .attr_f("height", box_region.height)
                    .attr("rx", "2")
                    .style("fill", theme.control_fill)
                    .style("stroke", theme.control_stroke)
                    .style("stroke-width", "1px"),
            );
        for d in disclosure_glyph_paths(&box_region, expanded) {
            control = control.child(
                Element::new("path")
                    .attr("class", "sysml-disclosure-glyph")
                    .attr("d", d)
                    .style("fill", theme.control_foreground),
            );
        }
        node = node.child(control);
    }

    if let Some((badge, badge_text)) = &header.badge {
        let group = Element::new("g")
            .attr("class", "general-hidden-relationships sysml-badge")
            .attr("role", "img")
            .attr("aria-label", format!("{badge_text} hidden relationships"))
            .child(Element::new("title").text(format!("{badge_text} hidden relationships")))
            .child(
                Element::new("rect")
                    .attr_f("x", badge.x)
                    .attr_f("y", badge.y)
                    .attr_f("width", badge.width)
                    .attr_f("height", badge.height)
                    .attr_f("rx", badge.height / 2.0)
                    .style("fill", theme.badge_fill)
                    .style("stroke", theme.divider)
                    .style("stroke-width", "1px"),
            )
            .child(
                Element::new("text")
                    .attr_f("x", badge.x + badge.width / 2.0)
                    .attr_f("y", badge.y + badge.height - 4.5)
                    .attr("text-anchor", "middle")
                    .text(badge_text.clone())
                    .style("font-size", "9px")
                    .style("font-weight", "600")
                    .style("fill", theme.badge_text),
            );
        node = node.child(group);
    }

    let item_chars = max_chars(
        (options.width - PADDING * 2.0 - 4.0).max(1.0),
        COMPARTMENT_FONT_SIZE,
        AVERAGE_GLYPH_RATIO_REGULAR,
    );
    for block in &blocks {
        node = node.child(
            Element::new("line")
                .attr_f("x1", span_x1)
                .attr_f("y1", block.divider_y)
                .attr_f("x2", span_x2)
                .attr_f("y2", block.divider_y)
                .attr("class", "sysml-compartment-divider")
                .style("stroke", theme.divider)
                .style("stroke-width", "1px"),
        );

        let member_word = if block.total_items == 1 {
            "member"
        } else {
            "members"
        };
        let title_text = format!("{} — {} {}", block.title, block.total_items, member_word);
        let mut label_group = if block.collapsible {
            Element::new("g")
                .attr(
                    "class",
                    "sysml-compartment-label sysml-disclosure sysml-compartment-toggle",
                )
                .attr("role", "button")
                .attr("tabindex", "0")
                .attr(
                    "aria-expanded",
                    if block.collapsed { "false" } else { "true" },
                )
                .attr("data-compartment-key", block.key.clone())
                .attr(
                    "aria-label",
                    format!(
                        "{} {} of {} ({})",
                        if block.collapsed { "Show" } else { "Hide" },
                        block.title,
                        options.data_element_name,
                        block.total_items,
                    ),
                )
        } else {
            Element::new("g")
                .attr("class", "sysml-compartment-label")
                .attr("data-compartment-key", block.key.clone())
        };
        label_group = label_group.child(Element::new("title").text(title_text));
        if block.collapsible {
            label_group = label_group.child(
                Element::new("rect")
                    .attr("class", "sysml-disclosure-target")
                    .attr_f("x", block.label_region.x)
                    .attr_f("y", block.label_region.y)
                    .attr_f("width", block.label_region.width)
                    .attr_f("height", block.label_region.height)
                    .attr("rx", "3")
                    .style("fill", "transparent")
                    .style("pointer-events", "all"),
            );
        }
        if let Some(disclosure_box) = &block.disclosure_box {
            label_group = label_group.child(
                Element::new("rect")
                    .attr("class", "sysml-disclosure-box")
                    .attr_f("x", disclosure_box.x)
                    .attr_f("y", disclosure_box.y)
                    .attr_f("width", disclosure_box.width)
                    .attr_f("height", disclosure_box.height)
                    .attr("rx", "2")
                    .style("fill", theme.control_fill)
                    .style("stroke", theme.control_stroke)
                    .style("stroke-width", "1px"),
            );
            for d in disclosure_glyph_paths(disclosure_box, !block.collapsed) {
                label_group = label_group.child(
                    Element::new("path")
                        .attr("class", "sysml-disclosure-glyph")
                        .attr("d", d)
                        .style("fill", theme.control_foreground),
                );
            }
        }
        label_group = label_group.child(
            Element::new("text")
                .attr_f("x", block.label_text_x)
                .attr_f("y", block.label_baseline)
                .text(block.title.clone())
                .style("font-size", format!("{}px", n(COMPARTMENT_FONT_SIZE)))
                .style("font-weight", "600")
                .style("letter-spacing", "0.02em")
                .style("fill", theme.text_secondary)
                .style("pointer-events", "none"),
        );
        node = node.child(label_group);

        for (index, item) in block.shown_items.iter().enumerate() {
            let text_el = Element::new("text")
                .attr("class", "sysml-compartment-item")
                .attr_f("x", PADDING)
                .attr_f("y", block.item_baselines[index])
                .text(truncate_to_chars(&item.display_text, item_chars))
                .style("font-size", format!("{}px", n(COMPARTMENT_FONT_SIZE)))
                .style("fill", theme.text_secondary)
                .child(Element::new("title").text(match &item.declared_in {
                    Some(declared_in) => format!("{} (from {})", item.display_text, declared_in),
                    None => item.display_text.clone(),
                }));
            node = node.child(text_el);
        }
        if block.overflow_count > 0 {
            if let Some(overflow_baseline) = block.overflow_baseline {
                node = node.child(
                    Element::new("text")
                        .attr("class", "sysml-compartment-overflow")
                        .attr_f("x", PADDING)
                        .attr_f("y", overflow_baseline)
                        .text(format!("+{} more", block.overflow_count))
                        .style("font-size", format!("{}px", n(COMPARTMENT_FONT_SIZE)))
                        .style("font-style", "italic")
                        .style("fill", theme.text_secondary)
                        .child(Element::new("title").text(format!(
                            "{} further {} not shown",
                            block.overflow_count,
                            block.title.to_lowercase()
                        ))),
                );
            }
        }
    }

    node
}
