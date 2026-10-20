/// Minimal SVG element tree + serializer for native diagram drawing. Attribute order is stable
/// (first write wins position; later writes update in place) so golden fixtures stay comparable.
pub struct Element {
    tag: String,
    attrs: Vec<(String, String)>,
    styles: Vec<(String, String)>,
    /// Attributes serialized after `style="..."`. Exists for `xmlns` on the export root: `VirtualElement.cloneNode`
    /// copies existing attrs and resolves `style` onto the clone *before* `exportSvg` calls `setAttribute("xmlns",
    /// ...)`, so `xmlns` lands after `style` in the real serialized output -- this reproduces that without modeling
    /// the clone step itself.
    trailing_attrs: Vec<(String, String)>,
    text: Option<String>,
    children: Vec<Element>,
}

impl Element {
    pub fn new(tag: &str) -> Self {
        Element {
            tag: tag.to_string(),
            attrs: Vec::new(),
            styles: Vec::new(),
            trailing_attrs: Vec::new(),
            text: None,
            children: Vec::new(),
        }
    }

    /// Sets an attribute, or -- matching `VirtualElement.setAttribute`'s `Map.set` semantics --
    /// overwrites the value in place (keeping its original position) if already set on this
    /// element. `attr`/`style` are called in one long builder chain per element, and it is
    /// unremarkable for two calls to target the same name (`applyEdgeMarker`'s branches all
    /// re-set `stroke`, for instance); pushing a second pair would serialize both, which no real
    /// `VirtualElement`-backed output ever does.
    pub fn attr(mut self, name: &str, value: impl Into<String>) -> Self {
        set_or_update(&mut self.attrs, name, value.into());
        self
    }

    pub fn attr_trailing(mut self, name: &str, value: impl Into<String>) -> Self {
        self.trailing_attrs.push((name.to_string(), value.into()));
        self
    }

    pub fn attr_f(mut self, name: &str, value: f64) -> Self {
        set_or_update(&mut self.attrs, name, format_number(value));
        self
    }

    pub fn maybe_attr(self, name: &str, value: Option<impl Into<String>>) -> Self {
        match value {
            Some(v) => self.attr(name, v),
            None => self,
        }
    }

    /// Same overwrite-in-place semantics as `attr` -- `VirtualStyle.setProperty` is backed by a
    /// `Map` too, so a later `.style()` call for an already-set property replaces its value
    /// without moving it or duplicating the declaration in the composed `style="..."` string.
    pub fn style(mut self, name: &str, value: impl Into<String>) -> Self {
        set_or_update(&mut self.styles, name, value.into());
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    pub fn maybe_child(self, child: Option<Element>) -> Self {
        match child {
            Some(c) => self.child(c),
            None => self,
        }
    }

    pub fn extend(mut self, children: impl IntoIterator<Item = Element>) -> Self {
        self.children.extend(children);
        self
    }

    fn has_class(&self, class_name: &str) -> bool {
        self.attrs.iter().any(|(name, value)| {
            name == "class" && value.split_whitespace().any(|c| c == class_name)
        })
    }

    /// Port of d3's `selection.insert(tag, beforeSelector)`, restricted to the one selector shape
    /// this crate needs: insert `new_child` immediately before the first existing child whose
    /// `class` attribute contains `before_class`, or append at the end when no child matches.
    pub fn insert_before_child(mut self, before_class: &str, new_child: Element) -> Self {
        let index = self
            .children
            .iter()
            .position(|child| child.has_class(before_class));
        match index {
            Some(index) => self.children.insert(index, new_child),
            None => self.children.push(new_child),
        }
        self
    }

    pub fn serialize(&self, out: &mut String) {
        out.push('<');
        out.push_str(&self.tag);
        for (name, value) in &self.attrs {
            out.push(' ');
            out.push_str(name);
            out.push_str("=\"");
            out.push_str(&escape_xml(value));
            out.push('"');
        }
        if !self.styles.is_empty() {
            let style_value: String = self
                .styles
                .iter()
                .map(|(k, v)| format!("{k}: {v};"))
                .collect::<Vec<_>>()
                .join(" ");
            out.push_str(" style=\"");
            out.push_str(&escape_xml(&style_value));
            out.push('"');
        }
        for (name, value) in &self.trailing_attrs {
            out.push(' ');
            out.push_str(name);
            out.push_str("=\"");
            out.push_str(&escape_xml(value));
            out.push('"');
        }
        out.push('>');
        if let Some(text) = &self.text {
            out.push_str(&escape_xml(text));
        }
        for child in &self.children {
            child.serialize(out);
        }
        out.push_str("</");
        out.push_str(&self.tag);
        out.push('>');
    }
}

impl std::fmt::Display for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = String::new();
        self.serialize(&mut out);
        f.write_str(&out)
    }
}

/// Matches D3/JS default number-to-string formatting closely enough for path/coordinate data:
/// integral values print without a trailing `.0`.
pub fn format_number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        let mut text = format!("{value}");
        if text.ends_with(".0") {
            text.truncate(text.len() - 2);
        }
        text
    }
}

fn set_or_update(pairs: &mut Vec<(String, String)>, name: &str, value: String) {
    match pairs.iter_mut().find(|(existing, _)| existing == name) {
        Some((_, existing_value)) => *existing_value = value,
        None => pairs.push((name.to_string(), value)),
    }
}

/// A width for `text` at `font_size` that a bold sans-serif rendering stays within, without font
/// metrics. Per-character classes keep it close for ordinary names: a flat one-em advance made
/// a typical label about 1.7 times its drawn width. Wide East Asian, fullwidth and emoji
/// characters, and the widest Latin letters, still get at least a full em.
pub(crate) fn estimated_text_width(text: &str, font_size: f64) -> f64 {
    text.chars().map(advance_em).sum::<f64>() * font_size
}

fn advance_em(character: char) -> f64 {
    match character {
        'W' | 'M' | 'm' | 'w' | '@' | '%' | 'Æ' | 'Œ' | 'æ' | 'œ' | '—' => 1.05,
        'i' | 'j' | 'l' | 'I' | 'f' | 't' | 'r' | '.' | ',' | ':' | ';' | '\'' | '"' | '`'
        | '!' | '|' | '(' | ')' | '[' | ']' | ' ' => 0.45,
        'A'..='Z' | '0'..='9' => 0.78,
        _ if is_wide(character) => 1.0,
        _ => 0.7,
    }
}

/// East Asian wide and fullwidth characters, and emoji (Unicode East_Asian_Width W/F blocks).
fn is_wide(character: char) -> bool {
    matches!(character as u32,
        0x1100..=0x115F
        | 0x2E80..=0x303E
        | 0x3041..=0x33FF
        | 0x3400..=0x4DBF
        | 0x4E00..=0x9FFF
        | 0xA000..=0xA4CF
        | 0xAC00..=0xD7A3
        | 0xF900..=0xFAFF
        | 0xFE30..=0xFE4F
        | 0xFF00..=0xFF60
        | 0xFFE0..=0xFFE6
        | 0x1F300..=0x1FAFF
        | 0x20000..=0x3FFFD)
}

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod text_width_tests {
    use super::estimated_text_width;

    #[test]
    fn ordinary_names_are_estimated_near_their_drawn_width_and_wide_text_by_full_ems() {
        // Typical identifiers sit well below a flat em per character.
        let name = "accept delivered / send archive";
        assert!(estimated_text_width(name, 10.0) < name.chars().count() as f64 * 10.0 * 0.75);
        // The widest Latin letters and East Asian text keep at least a full em per character.
        assert!(estimated_text_width("WWWW", 12.0) >= 48.0);
        assert_eq!(estimated_text_width("視圖承認", 12.0), 48.0);
        assert_eq!(estimated_text_width("", 12.0), 0.0);
    }
}
