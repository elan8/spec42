//! Neutral quick-fix text edit suggesters.

use std::collections::BTreeSet;

use sysml_model::semantic::ast_util::identification_name;
use sysml_model::semantic::kinds::TYPING_TARGET_KINDS;
use sysml_model::semantic::model::node_matches_simple_name;
use sysml_model::{
    resolve_imported_node_ids_for_simple_name, ElementKind, SemanticGraph, SemanticNode,
    TextPosition, TextRange,
};
use sysml_v2_parser::ast::{PackageBody, RootElement};
use url::Url;

use crate::dto::{TextEditDto, TextEditSuggestion};

const MAX_CANDIDATE_ACTIONS: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticLine {
    pub line: u32,
}

fn line_insert_range(line: u32) -> TextRange {
    TextRange::new(TextPosition::new(line, 0), TextPosition::new(line, 0))
}

fn line_full_range(line: u32, line_text: &str) -> TextRange {
    TextRange::new(
        TextPosition::new(line, 0),
        TextPosition::new(line, utf16_len(line_text)),
    )
}

fn utf16_len(s: &str) -> u32 {
    s.encode_utf16().count() as u32
}

fn parse_untyped_part_usage_name(raw_line: &str) -> Option<String> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    if !trimmed.starts_with("part ") || trimmed.starts_with("part def") {
        return None;
    }
    if !trimmed.ends_with(';') || trimmed.contains(':') {
        return None;
    }
    let after_part = trimmed.strip_prefix("part ")?;
    let name = after_part.strip_suffix(';')?.trim();
    if name.is_empty() || name.contains(char::is_whitespace) {
        return None;
    }
    Some(name.to_string())
}

fn to_pascal_case(name: &str) -> String {
    let mut out = String::new();
    let mut capitalize = true;
    for ch in name.chars() {
        if ch.is_alphanumeric() {
            if capitalize {
                for upper in ch.to_uppercase() {
                    out.push(upper);
                }
                capitalize = false;
            } else {
                out.push(ch);
            }
        } else {
            capitalize = true;
        }
    }
    if out.is_empty() {
        "GeneratedPart".to_string()
    } else {
        out
    }
}

fn find_block_end(lines: &[&str], start_line: usize) -> Option<usize> {
    let mut depth = 0i32;
    let mut seen_open = false;
    for (idx, line) in lines.iter().enumerate().skip(start_line) {
        for ch in line.chars() {
            match ch {
                '{' => {
                    depth += 1;
                    seen_open = true;
                }
                '}' if seen_open => {
                    depth -= 1;
                    if depth == 0 {
                        return Some(idx);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn is_definition_container_line(trimmed: &str) -> bool {
    (trimmed.starts_with("package ")
        || trimmed.starts_with("part def ")
        || trimmed.starts_with("item def ")
        || trimmed.starts_with("requirement def "))
        && trimmed.contains('{')
}

fn find_insertion_context(lines: &[&str], target_line: usize) -> Option<(usize, usize)> {
    for start in (0..=target_line).rev() {
        let trimmed = lines[start].trim();
        if !is_definition_container_line(trimmed) {
            continue;
        }
        let end = find_block_end(lines, start)?;
        if start <= target_line && target_line <= end {
            return Some((start, end));
        }
    }
    None
}

fn find_package_context(lines: &[&str], target_line: usize) -> Option<(usize, usize)> {
    for start in (0..=target_line).rev() {
        let trimmed = lines[start].trim();
        if !(trimmed.starts_with("package ") && trimmed.contains('{')) {
            continue;
        }
        let end = find_block_end(lines, start)?;
        if start <= target_line && target_line <= end {
            return Some((start, end));
        }
    }
    None
}

fn leading_indent(line: &str) -> String {
    let len = line.len().saturating_sub(line.trim_start().len());
    line[..len].to_string()
}

/// First non-empty member line inside `start..end` (exclusive of closing `}`).
fn member_indent_in_range(lines: &[&str], start: usize, end: usize) -> Option<String> {
    for line in lines.iter().take(end).skip(start + 1) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed == "}" {
            continue;
        }
        return Some(leading_indent(line));
    }
    None
}

/// Where to insert a new definition and which leading whitespace to use.
fn resolve_definition_insert_site(
    lines: &[&str],
    target_line: usize,
    container_start: usize,
    container_end: usize,
    usage_line: &str,
) -> (usize, usize, usize, String) {
    if let Some((pkg_start, pkg_end)) = find_package_context(lines, target_line) {
        let insert_line = if container_start > pkg_start && container_start < pkg_end {
            container_start
        } else {
            pkg_end
        };
        let insert_indent = if insert_line == container_start {
            lines
                .get(container_start)
                .map(|line| leading_indent(line))
                .unwrap_or_default()
        } else {
            member_indent_in_range(lines, pkg_start, pkg_end).unwrap_or_else(|| {
                let pkg_indent = lines
                    .get(pkg_start)
                    .map(|line| leading_indent(line))
                    .unwrap_or_default();
                let step = member_indent_in_range(lines, container_start, container_end)
                    .and_then(|member| {
                        member
                            .strip_prefix(&pkg_indent)
                            .filter(|s| !s.is_empty())
                            .map(str::to_string)
                    })
                    .unwrap_or_else(|| "  ".to_string());
                format!("{pkg_indent}{step}")
            })
        };
        (pkg_start, pkg_end, insert_line, insert_indent)
    } else {
        let insert_indent = leading_indent(usage_line);
        (0, container_end, container_end, insert_indent)
    }
}

fn has_matching_part_def(lines: &[&str], start: usize, end: usize, type_name: &str) -> bool {
    let needle = format!("part def {}", type_name);
    lines
        .iter()
        .take(end + 1)
        .skip(start)
        .any(|line| line.trim().starts_with(&needle))
}

fn has_matching_definition(
    lines: &[&str],
    start: usize,
    end: usize,
    definition_keyword: &str,
    type_name: &str,
) -> bool {
    let needle = format!("{definition_keyword} {type_name}");
    lines
        .iter()
        .take(end + 1)
        .skip(start)
        .any(|line| line.trim().starts_with(&needle))
}

fn definition_uses_brace_body(definition_keyword: &str) -> bool {
    matches!(definition_keyword, "part def" | "requirement def")
}

fn parse_simple_unresolved_type_usage(raw_line: &str) -> Option<(&'static str, String)> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    let (usage_keyword, definition_keyword) =
        if trimmed.starts_with("part ") && !trimmed.starts_with("part def ") {
            ("part", "part def")
        } else if trimmed.starts_with("port ") && !trimmed.starts_with("port def ") {
            ("port", "port def")
        } else if trimmed.starts_with("attribute ") && !trimmed.starts_with("attribute def ") {
            ("attribute", "attribute def")
        } else if trimmed.starts_with("item ") && !trimmed.starts_with("item def ") {
            ("item", "item def")
        } else if trimmed.starts_with("requirement ") && !trimmed.starts_with("requirement def ") {
            ("requirement", "requirement def")
        } else if trimmed.starts_with("ref ") {
            ("ref", "part def")
        } else {
            return None;
        };
    let after_keyword = trimmed.strip_prefix(usage_keyword)?.trim_start();
    // Prefer a typing colon that is not part of `:>` / `:>>`.
    let colon = after_keyword
        .char_indices()
        .find(|(idx, ch)| {
            *ch == ':'
                && !after_keyword[*idx..].starts_with(":>")
                && !after_keyword[*idx..].starts_with(":>>")
        })
        .map(|(idx, _)| idx)?;
    let after_colon = after_keyword[colon + 1..].trim_start();
    let type_part = after_colon
        .split(|ch: char| ch == ';' || ch == '{' || ch == '=' || ch.is_whitespace())
        .next()?
        .trim()
        .trim_start_matches('~');
    if type_part.is_empty()
        || type_part.contains("::")
        || type_part.contains('<')
        || type_part.contains('>')
    {
        return None;
    }
    Some((definition_keyword, type_part.to_string()))
}

/// Extract a simple name from a typing (`: Name`) or specializes (`:> Name` / `specializes Name`) line.
fn extract_simple_reference_name(raw_line: &str) -> Option<String> {
    if let Some((_, type_name)) = parse_simple_unresolved_type_usage(raw_line) {
        return Some(type_name);
    }
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    if let Some(after) = trimmed.split("specializes ").nth(1) {
        let name = after
            .split(|ch: char| ch == ';' || ch == '{' || ch == ',' || ch.is_whitespace())
            .next()?
            .trim();
        if !name.is_empty() && !name.contains("::") {
            return Some(name.to_string());
        }
    }
    // `:> Name` specializes (but not `:>>`).
    if let Some(idx) = trimmed.find(":>") {
        if trimmed[idx..].starts_with(":>>") {
            return None;
        }
        let after = trimmed[idx + 2..].trim_start();
        let name = after
            .split(|ch: char| ch == ';' || ch == '{' || ch == ',' || ch.is_whitespace())
            .next()?
            .trim();
        if !name.is_empty() && !name.contains("::") {
            return Some(name.to_string());
        }
    }
    None
}

fn display_qualified_name(qn: &str) -> String {
    match qn.split_once('#') {
        Some((base, _)) => base.to_string(),
        None => qn.to_string(),
    }
}

fn package_prefix(qn: &str) -> Option<String> {
    let display = display_qualified_name(qn);
    display
        .rsplit_once("::")
        .map(|(prefix, _)| prefix.to_string())
}

fn is_typing_definition_kind(kind: &ElementKind) -> bool {
    TYPING_TARGET_KINDS.iter().any(|allowed| allowed == kind)
}

fn is_ambiguous_candidate_kind(kind: &ElementKind) -> bool {
    is_typing_definition_kind(kind) || matches!(kind, ElementKind::Package)
}

fn context_node_at_line<'a>(
    graph: &'a SemanticGraph,
    uri: &Url,
    line: u32,
) -> Option<&'a SemanticNode> {
    let mut best: Option<&SemanticNode> = None;
    for node in graph.nodes_for_uri(uri) {
        if node.range.start.line > line || line > node.range.end.line {
            continue;
        }
        let better = match best {
            None => true,
            Some(prev) => {
                let prev_span = prev.range.end.line.saturating_sub(prev.range.start.line);
                let cur_span = node.range.end.line.saturating_sub(node.range.start.line);
                cur_span < prev_span
                    || (cur_span == prev_span
                        && node.id.qualified_name.len() > prev.id.qualified_name.len())
            }
        };
        if better {
            best = Some(node);
        }
    }
    best
}

fn enclosing_package_qn(graph: &SemanticGraph, node: &SemanticNode) -> Option<String> {
    let mut current = Some(node);
    while let Some(n) = current {
        if matches!(n.element_kind, ElementKind::Package) {
            let qn = display_qualified_name(&n.id.qualified_name);
            if !qn.is_empty() {
                return Some(qn);
            }
        }
        current = n.parent_id.as_ref().and_then(|id| graph.get_node(id));
    }
    None
}

fn collect_ambiguous_candidate_qns(
    graph: &SemanticGraph,
    context: &SemanticNode,
    name: &str,
) -> Vec<String> {
    let mut qns = BTreeSet::new();
    for id in resolve_imported_node_ids_for_simple_name(graph, context, name) {
        if let Some(node) = graph.get_node(&id) {
            if is_ambiguous_candidate_kind(&node.element_kind) {
                qns.insert(display_qualified_name(&node.id.qualified_name));
            }
        }
    }
    for node in graph.nodes_for_uri(&context.id.uri) {
        if node_matches_simple_name(node, name)
            && is_ambiguous_candidate_kind(&node.element_kind)
            && node.id.uri == context.id.uri
        {
            qns.insert(display_qualified_name(&node.id.qualified_name));
        }
    }
    qns.into_iter()
        .filter(|qn| qn.contains("::"))
        .take(MAX_CANDIDATE_ACTIONS)
        .collect()
}

fn collect_import_candidate_qns(
    graph: &SemanticGraph,
    document_uri: &Url,
    name: &str,
    enclosing_package: Option<&str>,
) -> Vec<String> {
    let mut scored: Vec<(bool, String)> = Vec::new();
    let mut seen = BTreeSet::new();
    for node in graph.nodes_named(name) {
        if !is_typing_definition_kind(&node.element_kind) {
            continue;
        }
        let qn = display_qualified_name(&node.id.qualified_name);
        if !qn.contains("::") || !seen.insert(qn.clone()) {
            continue;
        }
        if let Some(pkg) = enclosing_package {
            if package_prefix(&qn).as_deref() == Some(pkg) {
                continue;
            }
        }
        // Prefer definitions from other documents when ranking, but keep all.
        let other_doc = &node.id.uri != document_uri;
        scored.push((other_doc, qn));
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0).then_with(|| a.1.cmp(&b.1)));
    scored
        .into_iter()
        .map(|(_, qn)| qn)
        .take(MAX_CANDIDATE_ACTIONS)
        .collect()
}

fn find_import_insert_site(lines: &[&str], target_line: usize) -> (usize, String) {
    let package = find_package_context(lines, target_line);
    let (pkg_start, pkg_end) = package.unwrap_or((0, lines.len()));
    let search_start = if package.is_some() { pkg_start + 1 } else { 0 };
    let indent = if package.is_some() {
        member_indent_in_range(lines, pkg_start, pkg_end).unwrap_or_else(|| "  ".to_string())
    } else {
        String::new()
    };
    let mut last_import_line: Option<usize> = None;
    for (idx, line) in lines.iter().enumerate().take(pkg_end).skip(search_start) {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }
        let is_import = (trimmed.starts_with("import ")
            || trimmed.starts_with("private import ")
            || trimmed.starts_with("public import ")
            || trimmed.starts_with("protected import "))
            && trimmed.ends_with(';');
        if is_import {
            last_import_line = Some(idx);
            continue;
        }
        if last_import_line.is_some() {
            break;
        }
        return (idx, indent);
    }
    if let Some(idx) = last_import_line {
        (idx + 1, indent)
    } else {
        (search_start, indent)
    }
}

fn source_already_imports(lines: &[&str], qn: &str) -> bool {
    let needles = [
        format!("import {qn};"),
        format!("import {qn}::"),
        format!("import {qn}::*;"),
    ];
    lines.iter().any(|line| {
        let trimmed = line.trim();
        needles.iter().any(|needle| trimmed.contains(needle.as_str()))
    })
}

fn rewrite_line_replacing_simple_name(raw_line: &str, simple_name: &str, qualified: &str) -> Option<String> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let comment_part = &raw_line[code_only.len()..];
    // Replace the type/specializes reference token, preferring the last whole-word match
    // after `:` / `:>` / `specializes`.
    let mut replace_at: Option<usize> = None;
    let mut search_from = 0usize;
    while let Some(rel) = code_only[search_from..].find(simple_name) {
        let abs = search_from + rel;
        let before_ok = abs == 0
            || !code_only[..abs]
                .chars()
                .next_back()
                .is_some_and(|ch| ch.is_alphanumeric() || ch == '_');
        let after = abs + simple_name.len();
        let after_ok = after >= code_only.len()
            || !code_only[after..]
                .chars()
                .next()
                .is_some_and(|ch| ch.is_alphanumeric() || ch == '_' || ch == ':');
        if before_ok && after_ok {
            replace_at = Some(abs);
        }
        search_from = abs + simple_name.len();
    }
    let abs = replace_at?;
    let mut rewritten = String::new();
    rewritten.push_str(&code_only[..abs]);
    rewritten.push_str(qualified);
    rewritten.push_str(&code_only[abs + simple_name.len()..]);
    rewritten.push_str(comment_part);
    Some(rewritten)
}

fn suggest_create_definition_impl(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let (definition_keyword, type_name) = parse_simple_unresolved_type_usage(raw_line)?;
    let (container_start, container_end) = find_insertion_context(&lines, target_line)?;
    let (search_start, search_end, insert_line, insert_indent) = resolve_definition_insert_site(
        &lines,
        target_line,
        container_start,
        container_end,
        raw_line,
    );
    if has_matching_definition(
        &lines,
        search_start,
        search_end,
        definition_keyword,
        &type_name,
    ) {
        return None;
    }
    let body = if definition_uses_brace_body(definition_keyword) {
        format!(
            "{indent}{definition_keyword} {type_name} {{ }}\n",
            indent = insert_indent
        )
    } else {
        format!(
            "{indent}{definition_keyword} {type_name};\n",
            indent = insert_indent
        )
    };
    Some(TextEditSuggestion::new(
        format!("Create `{definition_keyword} {type_name}`"),
        vec![TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: body,
        }],
    ))
}

fn rewrite_untyped_part_usage_line(raw_line: &str, usage_name: &str, type_name: &str) -> String {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let comment_part = &raw_line[code_only.len()..];
    let leading_ws_len = code_only.len() - code_only.trim_start().len();
    let leading = &code_only[..leading_ws_len];
    format!("{leading}part {usage_name} : {type_name};{comment_part}")
}

fn rewrite_implicit_redefinition_line(raw_line: &str) -> Option<String> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let comment_part = &raw_line[code_only.len()..];
    if !code_only.contains('=') || code_only.contains(":>>") {
        return None;
    }
    let leading_ws_len = code_only.len() - code_only.trim_start().len();
    let leading = &code_only[..leading_ws_len];
    let trimmed = code_only.trim_start();
    let keywords = [
        "attribute",
        "part",
        "port",
        "ref",
        "item",
        "actor",
        "perform",
        "in",
        "out",
        "inout",
    ];
    for keyword in keywords {
        let prefix = format!("{keyword} ");
        if trimmed.starts_with(&prefix) {
            let remainder = &trimmed[prefix.len()..];
            if remainder.starts_with(":>>") {
                return None;
            }
            return Some(format!(
                "{leading}{keyword} :>> {remainder}{comment_part}"
            ));
        }
    }
    None
}

fn suggest_create_matching_part_def_impl(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let usage_name = parse_untyped_part_usage_name(raw_line)?;
    let type_name = to_pascal_case(&usage_name);
    let (container_start, container_end) = find_insertion_context(&lines, target_line)?;
    let (search_start, search_end, insert_line, insert_indent) = resolve_definition_insert_site(
        &lines,
        target_line,
        container_start,
        container_end,
        raw_line,
    );

    let mut edits = Vec::new();
    if !has_matching_part_def(&lines, search_start, search_end, &type_name) {
        edits.push(TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: format!("{indent}part def {type_name} {{ }}\n", indent = insert_indent),
        });
    }
    edits.push(TextEditDto {
        path: path.to_string(),
        range: line_full_range(target_line as u32, raw_line),
        replacement: rewrite_untyped_part_usage_line(raw_line, &usage_name, &type_name),
    });
    Some(TextEditSuggestion::new(
        format!("Create matching `part def {}` and type usage", type_name),
        edits,
    ))
}

fn suggest_explicit_redefinition_impl(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let rewritten = rewrite_implicit_redefinition_line(raw_line)?;
    Some(TextEditSuggestion::new(
        "Make redefinition explicit with `:>>`",
        vec![TextEditDto {
            path: path.to_string(),
            range: line_full_range(target_line as u32, raw_line),
            replacement: rewritten,
        }],
    ))
}

fn parse_requirement_name(raw_line: &str) -> Option<(String, bool)> {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let trimmed = code_only.trim();
    let (rest, is_def) = if let Some(rest) = trimmed.strip_prefix("requirement def ") {
        (rest, true)
    } else {
        (trimmed.strip_prefix("requirement ")?, false)
    };
    let name = rest
        .split(|ch: char| ch == ';' || ch == '{' || ch == ':' || ch.is_whitespace())
        .next()?
        .trim();
    if name.is_empty() || name.contains("::") {
        return None;
    }
    Some((name.to_string(), is_def))
}

fn suggest_create_verification_case_impl(
    source: &str,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    let target_line = line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let (req_name, _) = parse_requirement_name(raw_line)?;
    let verify_name = format!("Verify{}", to_pascal_case(&req_name));
    let (search_start, search_end) = find_package_context(&lines, target_line)
        .unwrap_or((0, lines.len().saturating_sub(1)));
    if has_matching_definition(&lines, search_start, search_end, "verification def", &verify_name)
    {
        return None;
    }
    let insert_line = if raw_line.contains('{') {
        find_block_end(&lines, target_line)?.saturating_add(1)
    } else {
        target_line + 1
    };
    let indent = if let Some((pkg_start, pkg_end)) = find_package_context(&lines, target_line) {
        member_indent_in_range(&lines, pkg_start, pkg_end).unwrap_or_else(|| "  ".to_string())
    } else {
        leading_indent(raw_line)
    };
    let step = if indent.is_empty() {
        "  ".to_string()
    } else if indent.contains('\t') {
        "\t".to_string()
    } else {
        "  ".to_string()
    };
    let body = format!(
        "{indent}verification def {verify_name} {{\n{indent}{step}objective {{\n{indent}{step}{step}verify {req_name};\n{indent}{step}}}\n{indent}}}\n"
    );
    Some(TextEditSuggestion::new(
        format!("Create verification case `verification def {verify_name}`"),
        vec![TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: body,
        }],
    ))
}

fn parse_case_header(raw_line: &str) -> bool {
    let trimmed = raw_line.split("//").next().unwrap_or("").trim_start();
    [
        "verification def ",
        "verification ",
        "analysis def ",
        "analysis ",
    ]
    .iter()
    .any(|prefix| trimmed.starts_with(prefix) && trimmed.contains('{'))
}

fn suggest_add_missing_case_subject_impl(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let lines: Vec<&str> = source.lines().collect();
    let case_line = diagnostic.line as usize;
    let header = *lines.get(case_line)?;
    if !parse_case_header(header) {
        return None;
    }
    let block_end = find_block_end(&lines, case_line)?;
    if lines
        .iter()
        .take(block_end)
        .skip(case_line + 1)
        .any(|line| line.trim_start().starts_with("subject "))
    {
        return None;
    }
    let indent = member_indent_in_range(&lines, case_line, block_end).unwrap_or_else(|| {
        let header_indent = leading_indent(header);
        let step = if header_indent.contains('\t') {
            "\t"
        } else {
            "  "
        };
        format!("{header_indent}{step}")
    });
    Some(
        TextEditSuggestion::new(
            "Add missing case subject",
            vec![TextEditDto {
                path: path.to_string(),
                range: line_insert_range(case_line as u32 + 1),
                replacement: format!("{indent}subject subjectUnderVerification;\n"),
            }],
        )
        .with_preferred(true),
    )
}

fn lower_camel_case(name: &str) -> Option<String> {
    let mut chars = name.chars();
    let first = chars.next()?;
    if !(first == '_' || first.is_ascii_alphabetic())
        || !chars
            .clone()
            .all(|ch| ch == '_' || ch.is_ascii_alphanumeric())
    {
        return None;
    }
    Some(first.to_ascii_lowercase().to_string() + chars.as_str())
}

fn parse_definition_header(raw_line: &str) -> Option<(&'static str, String)> {
    let trimmed = raw_line.split("//").next().unwrap_or("").trim();
    for (definition_keyword, usage_keyword) in [
        ("requirement def ", "requirement"),
        ("verification def ", "verification"),
        ("viewpoint def ", "viewpoint"),
        ("constraint def ", "constraint"),
        ("connection def ", "connection"),
        ("interface def ", "interface"),
        ("rendering def ", "rendering"),
        ("occurrence def ", "occurrence"),
        ("attribute def ", "attribute"),
        ("analysis def ", "analysis"),
        ("use case def ", "use case"),
        ("action def ", "action"),
        ("state def ", "state"),
        ("part def ", "part"),
        ("item def ", "item"),
        ("port def ", "port"),
        ("calc def ", "calc"),
    ] {
        let Some(rest) = trimmed.strip_prefix(definition_keyword) else {
            continue;
        };
        let name = rest
            .split(|ch: char| {
                ch == ';' || ch == '{' || ch == ':' || ch == '[' || ch.is_whitespace()
            })
            .next()?
            .trim();
        if name.is_empty() || name.contains("::") || name.starts_with('\'') {
            return None;
        }
        return Some((usage_keyword, name.to_string()));
    }
    None
}

fn suggest_create_usage_from_definition_impl(
    source: &str,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    let lines: Vec<&str> = source.lines().collect();
    let definition_line = line as usize;
    let raw_line = *lines.get(definition_line)?;
    let (usage_keyword, definition_name) = parse_definition_header(raw_line)?;
    let usage_name = lower_camel_case(&definition_name)?;
    let insert_line = if raw_line.contains('{') {
        find_block_end(&lines, definition_line)?.saturating_add(1)
    } else if raw_line.trim_end().ends_with(';') {
        definition_line + 1
    } else {
        return None;
    };
    let (search_start, search_end) =
        find_package_context(&lines, definition_line).unwrap_or((0, lines.len().saturating_sub(1)));
    let existing_prefix = format!("{usage_keyword} {usage_name} ");
    if lines
        .iter()
        .take(search_end + 1)
        .skip(search_start)
        .any(|line| {
            let trimmed = line.trim_start();
            trimmed.starts_with(&existing_prefix)
                && trimmed
                    .split_once(':')
                    .is_some_and(|(_, target)| target.trim_start().starts_with(&definition_name))
        })
    {
        return None;
    }
    let indent = leading_indent(raw_line);
    Some(TextEditSuggestion::new(
        format!("Create `{usage_keyword} {usage_name} : {definition_name}`"),
        vec![TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: format!("{indent}{usage_keyword} {usage_name} : {definition_name};\n"),
        }],
    ))
}

pub fn suggest_wrap_in_package(source: &str, path: &str) -> Option<TextEditSuggestion> {
    let root = sysml_v2_parser::parse(source).ok()?;
    let packages: Vec<_> = root
        .elements
        .iter()
        .filter_map(|n| match &n.value {
            RootElement::Package(p) => Some(p),
            _ => None,
        })
        .collect();
    if packages.len() != 1 {
        return None;
    }
    let pkg = packages[0];
    if !identification_name(&pkg.identification).is_empty() {
        return None;
    }
    let has_members = match &pkg.body {
        PackageBody::Brace { elements } => !elements.is_empty(),
        _ => false,
    };
    if !has_members {
        return None;
    }
    let lines: Vec<&str> = source.lines().collect();
    let last_line = lines.len().saturating_sub(1) as u32;
    let last_char = lines.last().map(|l| utf16_len(l)).unwrap_or(0);
    Some(TextEditSuggestion::new(
        "Wrap in package",
        vec![TextEditDto {
            path: path.to_string(),
            range: TextRange::new(
                TextPosition::new(0, 0),
                TextPosition::new(last_line, last_char),
            ),
            replacement: format!("package Generated {{\n{}\n}}\n", source.trim_end()),
        }],
    ))
}

pub fn suggest_create_definition_for_unresolved_type_quick_fix(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_create_definition_impl(source, path, diagnostic)
}

pub fn suggest_create_matching_part_def_quick_fix(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_create_matching_part_def_impl(source, path, diagnostic)
}

pub fn suggest_explicit_redefinition_quick_fix(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_explicit_redefinition_impl(source, path, diagnostic)
}

pub fn suggest_create_verification_case(
    source: &str,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    suggest_create_verification_case_impl(source, path, line)
}

pub fn suggest_add_missing_case_subject_quick_fix(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_add_missing_case_subject_impl(source, path, diagnostic)
}

pub fn suggest_create_usage_from_definition(
    source: &str,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    suggest_create_usage_from_definition_impl(source, path, line)
}

/// Qualify an ambiguous simple name with each candidate qualified name.
pub fn suggest_qualify_ambiguous_name_quick_fixes(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
    graph: &SemanticGraph,
    document_uri: &Url,
) -> Vec<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let Some(raw_line) = lines.get(target_line).copied() else {
        return Vec::new();
    };
    let Some(simple_name) = extract_simple_reference_name(raw_line) else {
        return Vec::new();
    };
    let Some(context) = context_node_at_line(graph, document_uri, diagnostic.line) else {
        return Vec::new();
    };
    let candidates = collect_ambiguous_candidate_qns(graph, context, &simple_name);
    let preferred = candidates.len() == 1;
    candidates
        .into_iter()
        .filter_map(|qn| {
            let rewritten = rewrite_line_replacing_simple_name(raw_line, &simple_name, &qn)?;
            Some(
                TextEditSuggestion::new(
                    format!("Qualify as `{qn}`"),
                    vec![TextEditDto {
                        path: path.to_string(),
                        range: line_full_range(diagnostic.line, raw_line),
                        replacement: rewritten,
                    }],
                )
                .with_preferred(preferred),
            )
        })
        .collect()
}

/// Suggest importing a workspace/library definition for an unresolved type name.
pub fn suggest_add_import_quick_fixes(
    source: &str,
    path: &str,
    diagnostic: DiagnosticLine,
    graph: &SemanticGraph,
    document_uri: &Url,
) -> Vec<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let Some(raw_line) = lines.get(target_line).copied() else {
        return Vec::new();
    };
    let Some(simple_name) = extract_simple_reference_name(raw_line) else {
        return Vec::new();
    };
    let enclosing = context_node_at_line(graph, document_uri, diagnostic.line)
        .and_then(|node| enclosing_package_qn(graph, node));
    let candidates = collect_import_candidate_qns(
        graph,
        document_uri,
        &simple_name,
        enclosing.as_deref(),
    );
    if candidates.is_empty() {
        return Vec::new();
    }
    let (insert_line, indent) = find_import_insert_site(&lines, target_line);
    let preferred = candidates.len() == 1;
    candidates
        .into_iter()
        .filter(|qn| !source_already_imports(&lines, qn))
        .map(|qn| {
            TextEditSuggestion::new(
                format!("Import `{qn}`"),
                vec![TextEditDto {
                    path: path.to_string(),
                    range: line_insert_range(insert_line as u32),
                    replacement: format!("{indent}private import {qn};\n"),
                }],
            )
            .with_preferred(preferred)
        })
        .collect()
}
