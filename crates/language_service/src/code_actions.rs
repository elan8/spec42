//! Neutral quick-fix text edit suggesters.

use sysml_query::resolved_slice::{ElementKind, PublishedModel, TextPosition, TextRange};
use sysml_query::syntax::{ParsedSource, SyntaxOutlineKind, SyntaxOutlineNode};
use url::Url;

use crate::dto::{TextEditDto, TextEditSuggestion};
use crate::text::utf16_len;

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

/// The name of the untyped `part` usage a declaration line declares, if it declares one.
///
/// The one predicate for "this line is an untyped part usage": the code action that offers to
/// create a definition and the host's advisory diagnostic must agree about which lines qualify,
/// and a second copy of the test is a second answer. Text-level by necessity — the line may not
/// have parsed — and it recovers a name, never a semantic fact.
pub fn parse_untyped_part_usage_name(raw_line: &str) -> Option<String> {
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

fn declaration_extent(node: &SyntaxOutlineNode) -> Option<(usize, usize)> {
    node.body_range
        .map(|body| (node.range.start_line as usize, body.end_line as usize))
}

fn contexts(
    parsed: &ParsedSource,
    line: u32,
) -> (Option<SyntaxOutlineNode>, Option<SyntaxOutlineNode>) {
    let enclosing = parsed.enclosing_declarations(line);
    let package = enclosing
        .iter()
        .rev()
        .find(|node| {
            matches!(
                node.kind,
                SyntaxOutlineKind::Package | SyntaxOutlineKind::LibraryPackage
            )
        })
        .cloned();
    let container = enclosing
        .iter()
        .rev()
        .find(|node| node.body_range.is_some())
        .cloned();
    (package, container)
}

fn any_outline_node(parsed: &ParsedSource, predicate: impl Fn(&SyntaxOutlineNode) -> bool) -> bool {
    fn visit(nodes: &[SyntaxOutlineNode], predicate: &impl Fn(&SyntaxOutlineNode) -> bool) -> bool {
        nodes
            .iter()
            .any(|node| predicate(node) || visit(&node.children, predicate))
    }
    visit(&parsed.outline(), &predicate)
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
    container_start: usize,
    container_end: usize,
    package_extent: Option<(usize, usize)>,
    usage_line: &str,
) -> (usize, usize, usize, String) {
    if let Some((pkg_start, pkg_end)) = package_extent {
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

fn has_matching_definition(model: &PublishedModel, kind: SyntaxOutlineKind, name: &str) -> bool {
    let kind = match kind {
        SyntaxOutlineKind::PartDef => ElementKind::PartDefinition,
        SyntaxOutlineKind::PortDef => ElementKind::PortDefinition,
        SyntaxOutlineKind::AttributeDef => ElementKind::AttributeDefinition,
        SyntaxOutlineKind::ItemDef => ElementKind::ItemDefinition,
        SyntaxOutlineKind::RequirementDef => ElementKind::RequirementDefinition,
        SyntaxOutlineKind::VerificationDef => ElementKind::VerificationCaseDefinition,
        _ => return false,
    };
    matches!(
        model.inspection().named_element_exists(kind, name).answer,
        sysml_query::resolved_slice::QueryAnswer::Resolved(true)
    )
}

fn definition_uses_brace_body(definition_keyword: &str) -> bool {
    matches!(definition_keyword, "part def" | "requirement def")
}

fn definition_for_usage(
    node: &SyntaxOutlineNode,
) -> Option<(SyntaxOutlineKind, &'static str, String)> {
    let (kind, keyword) = match node.kind {
        SyntaxOutlineKind::PartUsage | SyntaxOutlineKind::Ref => {
            (SyntaxOutlineKind::PartDef, "part def")
        }
        SyntaxOutlineKind::PortUsage => (SyntaxOutlineKind::PortDef, "port def"),
        SyntaxOutlineKind::AttributeUsage => (SyntaxOutlineKind::AttributeDef, "attribute def"),
        SyntaxOutlineKind::ItemUsage => (SyntaxOutlineKind::ItemDef, "item def"),
        SyntaxOutlineKind::RequirementUsage => {
            (SyntaxOutlineKind::RequirementDef, "requirement def")
        }
        _ => return None,
    };
    Some((kind, keyword, node.simple_typed_by()?.to_string()))
}

fn suggest_create_definition_impl(
    source: &str,
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    let declaration = parsed.declaration_at(diagnostic.line)?;
    let (definition_kind, definition_keyword, type_name) = definition_for_usage(&declaration)?;
    let (package, container) = contexts(parsed, diagnostic.line);
    let (container_start, container_end) = declaration_extent(&container?)?;
    let (search_start, search_end, insert_line, insert_indent) = resolve_definition_insert_site(
        &lines,
        container_start,
        container_end,
        package.as_ref().and_then(declaration_extent),
        raw_line,
    );
    let _ = (search_start, search_end);
    if has_matching_definition(model, definition_kind, &type_name) {
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
            return Some(format!("{leading}{keyword} :>> {remainder}{comment_part}"));
        }
    }
    None
}

fn suggest_create_matching_part_def_impl(
    source: &str,
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let target_line = diagnostic.line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    // This diagnostic exists precisely when the parser could not publish a typed usage node.
    // Recover only the authored name; container and existence decisions remain typed queries.
    let usage_name = parse_untyped_part_usage_name(raw_line)?;
    let type_name = to_pascal_case(&usage_name);
    let (package, container) = contexts(parsed, diagnostic.line);
    let (container_start, container_end) = declaration_extent(&container?)?;
    let (search_start, search_end, insert_line, insert_indent) = resolve_definition_insert_site(
        &lines,
        container_start,
        container_end,
        package.as_ref().and_then(declaration_extent),
        raw_line,
    );

    let mut edits = Vec::new();
    let _ = (search_start, search_end);
    if !has_matching_definition(model, SyntaxOutlineKind::PartDef, &type_name) {
        edits.push(TextEditDto {
            path: path.to_string(),
            range: line_insert_range(insert_line as u32),
            replacement: format!(
                "{indent}part def {type_name} {{ }}\n",
                indent = insert_indent
            ),
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

fn suggest_create_verification_case_impl(
    source: &str,
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    let target_line = line as usize;
    let lines: Vec<&str> = source.lines().collect();
    let raw_line = *lines.get(target_line)?;
    if !parsed.is_clean() {
        return None;
    }
    let declaration = parsed.declaration_at(line)?;
    if declaration.kind != SyntaxOutlineKind::RequirementDef {
        return None;
    }
    let req_name = declaration.name;
    let verify_name = format!("Verify{}", to_pascal_case(&req_name));
    if has_matching_definition(model, SyntaxOutlineKind::VerificationDef, &verify_name) {
        return None;
    }
    let insert_line = declaration.range.end_line as usize + 1;
    let (package, _) = contexts(parsed, line);
    let indent = if let Some((pkg_start, pkg_end)) = package.as_ref().and_then(declaration_extent) {
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

fn suggest_add_missing_case_subject_impl(
    source: &str,
    parsed: &ParsedSource,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    let lines: Vec<&str> = source.lines().collect();
    let case_line = diagnostic.line as usize;
    let header = *lines.get(case_line)?;
    if !parsed.is_clean() {
        return None;
    }
    let declaration = parsed.declaration_at(diagnostic.line)?;
    if !matches!(
        declaration.kind,
        SyntaxOutlineKind::VerificationDef
            | SyntaxOutlineKind::VerificationUsage
            | SyntaxOutlineKind::AnalysisDef
            | SyntaxOutlineKind::AnalysisUsage
    ) {
        return None;
    }
    let (_, block_end) = declaration_extent(&declaration)?;
    if declaration.has_case_subject {
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

fn suggest_create_usage_from_definition_impl(
    source: &str,
    parsed: &ParsedSource,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    let lines: Vec<&str> = source.lines().collect();
    let definition_line = line as usize;
    let raw_line = *lines.get(definition_line)?;
    if !parsed.is_clean() {
        return None;
    }
    let declaration = parsed.declaration_at(line)?;
    let usage_kind = match declaration.kind {
        SyntaxOutlineKind::PartDef => SyntaxOutlineKind::PartUsage,
        SyntaxOutlineKind::PortDef => SyntaxOutlineKind::PortUsage,
        SyntaxOutlineKind::ItemDef => SyntaxOutlineKind::ItemUsage,
        SyntaxOutlineKind::AttributeDef => SyntaxOutlineKind::AttributeUsage,
        SyntaxOutlineKind::RequirementDef => SyntaxOutlineKind::RequirementUsage,
        SyntaxOutlineKind::AnalysisDef => SyntaxOutlineKind::AnalysisUsage,
        SyntaxOutlineKind::VerificationDef => SyntaxOutlineKind::VerificationUsage,
        _ => return None,
    };
    let usage_keyword = usage_kind.keyword();
    let definition_name = declaration.name;
    let usage_name = lower_camel_case(&definition_name)?;
    let insert_line = declaration.range.end_line as usize + 1;
    if any_outline_node(parsed, |node| {
        node.kind == usage_kind
            && node.name == usage_name
            && node.typed_by.as_deref() == Some(definition_name.as_str())
    }) {
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

pub fn suggest_wrap_in_package(
    source: &str,
    parsed: &ParsedSource,
    path: &str,
) -> Option<TextEditSuggestion> {
    // An unparseable document is not a document with one anonymous package.
    if !parsed.is_clean() || !parsed.declares_single_anonymous_package_with_members() {
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
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_create_definition_impl(source, parsed, model, path, diagnostic)
}

pub fn suggest_create_matching_part_def_quick_fix(
    source: &str,
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_create_matching_part_def_impl(source, parsed, model, path, diagnostic)
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
    parsed: &ParsedSource,
    model: &PublishedModel,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    suggest_create_verification_case_impl(source, parsed, model, path, line)
}

pub fn suggest_add_missing_case_subject_quick_fix(
    source: &str,
    parsed: &ParsedSource,
    path: &str,
    diagnostic: DiagnosticLine,
) -> Option<TextEditSuggestion> {
    suggest_add_missing_case_subject_impl(source, parsed, path, diagnostic)
}

pub fn suggest_create_usage_from_definition(
    source: &str,
    parsed: &ParsedSource,
    path: &str,
    line: u32,
) -> Option<TextEditSuggestion> {
    suggest_create_usage_from_definition_impl(source, parsed, path, line)
}

/// Qualify an ambiguous simple name with each candidate qualified name.
pub fn suggest_qualify_ambiguous_name_quick_fixes(
    _source: &str,
    _path: &str,
    _diagnostic: DiagnosticLine,
    _model: &PublishedModel,
    _document_uri: &Url,
) -> Vec<TextEditSuggestion> {
    // TODO(follow-up): restore after a typed query exposes ambiguous candidates and authored
    // replacement ranges. Returning no actions keeps unsupported semantics explicit.
    Vec::new()
}

/// Suggest importing a workspace/library definition for an unresolved type name.
pub fn suggest_add_import_quick_fixes(
    _source: &str,
    _path: &str,
    _diagnostic: DiagnosticLine,
    _model: &PublishedModel,
    _document_uri: &Url,
) -> Vec<TextEditSuggestion> {
    // TODO(follow-up): restore after a typed query exposes importable definitions and the owning
    // package/import insertion contract. Returning no actions is the intentional disabled state.
    Vec::new()
}
