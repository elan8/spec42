//! Neutral quick-fix text edit suggesters.

use semantic_core::semantic::ast_util::identification_name;
use semantic_core::{TextPosition, TextRange};
use sysml_v2_parser::ast::{PackageBody, RootElement};

use crate::dto::{TextEditDto, TextEditSuggestion};

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

fn find_insertion_context(lines: &[&str], target_line: usize) -> Option<(usize, usize)> {
    for start in (0..=target_line).rev() {
        let trimmed = lines[start].trim();
        let is_container = (trimmed.starts_with("package ") || trimmed.starts_with("part def "))
            && trimmed.contains('{');
        if !is_container {
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
        } else {
            return None;
        };
    let after_keyword = trimmed.strip_prefix(usage_keyword)?.trim_start();
    let colon = after_keyword.find(':')?;
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
    let body = if definition_keyword == "part def" {
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
    Some(TextEditSuggestion {
        title: format!("Create `{definition_keyword} {type_name}`"),
        edits: vec![TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: body,
        }],
    })
}

fn rewrite_untyped_part_usage_line(raw_line: &str, usage_name: &str, type_name: &str) -> String {
    let code_only = raw_line.split("//").next().unwrap_or("");
    let comment_part = &raw_line[code_only.len()..];
    let leading_ws_len = code_only.len() - code_only.trim_start().len();
    let leading = &code_only[..leading_ws_len];
    format!(
        "{leading}part {usage_name} : {type_name};{comment_part}",
        leading = leading,
        usage_name = usage_name,
        type_name = type_name,
        comment_part = comment_part
    )
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
                "{leading}{keyword} :>> {remainder}{comment_part}",
                leading = leading,
                keyword = keyword,
                remainder = remainder,
                comment_part = comment_part
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
            replacement: format!(
                "{indent}part def {type_name} {{ }}
",
                indent = insert_indent
            ),
        });
    }
    edits.push(TextEditDto {
        path: path.to_string(),
        range: line_full_range(target_line as u32, raw_line),
        replacement: rewrite_untyped_part_usage_line(raw_line, &usage_name, &type_name),
    });
    Some(TextEditSuggestion {
        title: format!("Create matching `part def {}` and type usage", type_name),
        edits,
    })
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
    Some(TextEditSuggestion {
        title: "Make redefinition explicit with `:>>`".to_string(),
        edits: vec![TextEditDto {
            path: path.to_string(),
            range: line_full_range(target_line as u32, raw_line),
            replacement: rewritten,
        }],
    })
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
    Some(TextEditSuggestion {
        title: "Wrap in package".to_string(),
        edits: vec![TextEditDto {
            path: path.to_string(),
            range: TextRange::new(TextPosition::new(0, 0), TextPosition::new(last_line, last_char)),
            replacement: format!("package Generated {{\n{}\n}}\n", source.trim_end()),
        }],
    })
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
