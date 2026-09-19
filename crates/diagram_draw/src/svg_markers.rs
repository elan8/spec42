use std::collections::BTreeMap;

/// Rust port of `summarizeSvgMarkers` in
/// `vscode/diagram-renderer/src/test-support/svg-markers.ts`, hand-rolled (no `regex` crate in
/// the workspace) so a Rust-rendered SVG can be checked against the same golden fixture the TS
/// suite checks against.
#[derive(Debug, PartialEq, Eq)]
pub struct SvgMarkerSummary {
    pub class_counts: BTreeMap<String, u32>,
    pub marker_ids: Vec<String>,
}

pub fn summarize_svg_markers(svg: &str) -> SvgMarkerSummary {
    let mut class_counts: BTreeMap<String, u32> = BTreeMap::new();
    for class_attr in find_attr_values(svg, "class") {
        for class in class_attr.split_whitespace() {
            *class_counts.entry(class.to_string()).or_insert(0) += 1;
        }
    }

    let mut marker_ids: Vec<String> = Vec::new();
    let mut search_from = 0;
    while let Some(marker_start) = svg[search_from..].find("<marker") {
        let tag_start = search_from + marker_start;
        let Some(tag_end_rel) = svg[tag_start..].find('>') else {
            break;
        };
        let tag = &svg[tag_start..tag_start + tag_end_rel];
        if let Some(id) = find_attr_value_in(tag, "id") {
            if !marker_ids.contains(&id) {
                marker_ids.push(id);
            }
        }
        search_from = tag_start + tag_end_rel + 1;
    }
    marker_ids.sort();

    SvgMarkerSummary {
        class_counts,
        marker_ids,
    }
}

fn find_attr_values(haystack: &str, attr: &str) -> Vec<String> {
    let needle = format!("{attr}=\"");
    let mut values = Vec::new();
    let mut search_from = 0;
    while let Some(rel) = haystack[search_from..].find(&needle) {
        let start = search_from + rel + needle.len();
        let Some(end_rel) = haystack[start..].find('"') else {
            break;
        };
        values.push(haystack[start..start + end_rel].to_string());
        search_from = start + end_rel + 1;
    }
    values
}

fn find_attr_value_in(tag: &str, attr: &str) -> Option<String> {
    find_attr_values(tag, attr).into_iter().next()
}
