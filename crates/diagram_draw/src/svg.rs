/// Minimal SVG element tree + serializer, standing in for the `VirtualElement` DOM shim
/// `headless-export.ts` uses to let D3 build an SVG tree without a real browser. Attribute order
/// is irrelevant here: the golden-parity check (`svg-markers.ts`) only regexes `class="..."` and
/// `<marker ... id="...">` occurrences, not a byte-for-byte tree diff.
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

fn escape_xml(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
