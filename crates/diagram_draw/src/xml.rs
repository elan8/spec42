//! Minimal recursive-descent XML/SVG parser for test use: comparing a Rust-rendered SVG string
//! against a real TS/D3-rendered one (`tests/golden_parity.rs`). Only handles what both
//! serializers actually produce -- nested elements with quoted attributes and text content, no
//! self-closing tags, no comments, no CDATA, no namespaces beyond attribute names -- since both
//! `svg::Element::serialize` and `VirtualElement.serialize` (`headless-export.ts`) are that simple.

use std::collections::BTreeMap;

#[derive(Debug, PartialEq, Eq)]
pub struct XmlNode {
    pub tag: String,
    pub attrs: BTreeMap<String, String>,
    pub text: String,
    pub children: Vec<XmlNode>,
}

pub fn parse(input: &str) -> XmlNode {
    let trimmed = input.trim();
    let bytes = trimmed.as_bytes();
    let start = bytes
        .iter()
        .position(|&b| b == b'<')
        .expect("input must contain an XML element");
    let (node, _) = parse_element(bytes, start);
    node
}

/// Parses one `<tag attr="value" ...>...</tag>` element starting at `start` (which must point at
/// the opening `<`). Returns the node and the index just past its closing tag.
fn parse_element(bytes: &[u8], start: usize) -> (XmlNode, usize) {
    assert_eq!(bytes[start], b'<', "expected '<' at {start}");
    let mut index = start + 1;
    let tag_start = index;
    while bytes[index] != b' ' && bytes[index] != b'>' {
        index += 1;
    }
    let tag = std::str::from_utf8(&bytes[tag_start..index])
        .unwrap()
        .to_string();

    let mut attrs = BTreeMap::new();
    loop {
        while bytes[index] == b' ' {
            index += 1;
        }
        if bytes[index] == b'>' {
            index += 1;
            break;
        }
        let name_start = index;
        while bytes[index] != b'=' {
            index += 1;
        }
        let name = std::str::from_utf8(&bytes[name_start..index])
            .unwrap()
            .to_string();
        index += 2; // skip `="`
        let value_start = index;
        while bytes[index] != b'"' {
            index += 1;
        }
        let value = unescape_xml(std::str::from_utf8(&bytes[value_start..index]).unwrap());
        index += 1; // skip closing quote
        attrs.insert(name, value);
    }

    let close_tag = format!("</{tag}>");
    let mut text = String::new();
    let mut children = Vec::new();
    loop {
        if bytes[index] == b'<' {
            if bytes[index..].starts_with(close_tag.as_bytes()) {
                index += close_tag.len();
                break;
            }
            let (child, next) = parse_element(bytes, index);
            children.push(child);
            index = next;
        } else {
            let text_start = index;
            while bytes[index] != b'<' {
                index += 1;
            }
            text.push_str(&unescape_xml(
                std::str::from_utf8(&bytes[text_start..index]).unwrap(),
            ));
        }
    }

    (
        XmlNode {
            tag,
            attrs,
            text,
            children,
        },
        index,
    )
}

fn unescape_xml(value: &str) -> String {
    value
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&amp;", "&")
}
