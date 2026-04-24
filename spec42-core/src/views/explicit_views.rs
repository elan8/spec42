use std::collections::{HashMap, HashSet};

use tower_lsp::lsp_types::Url;

use crate::views::dto::{PositionDto, RangeDto, SysmlVisualizationViewCandidateDto};
use crate::views::extracted_model::ActivityDiagramDto;
use sysml_v2_parser::ast::{
    Identification, PackageBody, PackageBodyElement, RootElement, RootNamespace, Span, ViewBody,
    ViewBodyElement, ViewDefBody, ViewDefBodyElement,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterExpr {
    Matches(String),
    Not(Box<FilterExpr>),
    And(Box<FilterExpr>, Box<FilterExpr>),
    Or(Box<FilterExpr>, Box<FilterExpr>),
    Unsupported(String),
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ViewDefinitionSpec {
    pub id: String,
    pub name: String,
    pub filters: Vec<FilterExpr>,
}

#[derive(Debug, Clone)]
pub struct ExposeSpec {
    pub target: String,
    pub filter: Option<FilterExpr>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ViewUsageSpec {
    pub id: String,
    pub name: String,
    pub definition_ref: Option<String>,
    pub definition_id: Option<String>,
    pub filters: Vec<FilterExpr>,
    pub exposes: Vec<ExposeSpec>,
    pub range: RangeDto,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ViewCatalog {
    pub definitions: HashMap<String, ViewDefinitionSpec>,
    pub usages: Vec<ViewUsageSpec>,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct EvaluatedView {
    pub id: String,
    pub name: String,
    pub effective_view_type: Option<String>,
    pub exposed_ids: HashSet<String>,
    pub filters: Vec<FilterExpr>,
    pub visible_ids: HashSet<String>,
    pub issues: Vec<String>,
}

pub fn build_view_catalog(
    index: &HashMap<Url, crate::workspace::state::IndexEntry>,
    workspace_uris: &[Url],
) -> ViewCatalog {
    let mut definitions = HashMap::new();
    let mut usages = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(entry) = index.get(workspace_uri) else {
            continue;
        };
        let Some(parsed) = entry.parsed.as_ref() else {
            continue;
        };
        walk_root_namespace(parsed, &entry.content, None, &mut definitions, &mut usages);
    }

    for usage in &mut usages {
        usage.definition_id = usage
            .definition_ref
            .as_deref()
            .and_then(|reference| resolve_definition_id(reference, &definitions));
    }

    ViewCatalog {
        definitions,
        usages,
    }
}

fn walk_root_namespace(
    root: &RootNamespace,
    content: &str,
    container: Option<&str>,
    definitions: &mut HashMap<String, ViewDefinitionSpec>,
    usages: &mut Vec<ViewUsageSpec>,
) {
    for element in &root.elements {
        match &element.value {
            RootElement::Package(package) => {
                walk_package_body(
                    &package.identification,
                    &package.body,
                    content,
                    container,
                    definitions,
                    usages,
                );
            }
            RootElement::Namespace(namespace) => {
                walk_package_body(
                    &namespace.identification,
                    &namespace.body,
                    content,
                    container,
                    definitions,
                    usages,
                );
            }
            _ => {}
        }
    }
}

fn walk_package_body(
    identification: &Identification,
    body: &PackageBody,
    content: &str,
    container: Option<&str>,
    definitions: &mut HashMap<String, ViewDefinitionSpec>,
    usages: &mut Vec<ViewUsageSpec>,
) {
    let next_container = identification
        .name
        .as_deref()
        .map(|name| qualify_name(container, name))
        .or_else(|| container.map(str::to_string));

    let PackageBody::Brace { elements } = body else {
        return;
    };

    for element in elements {
        match &element.value {
            PackageBodyElement::Package(package) => walk_package_body(
                &package.identification,
                &package.body,
                content,
                next_container.as_deref(),
                definitions,
                usages,
            ),
            PackageBodyElement::ViewDef(view_def) => {
                let name = identification_name(&view_def.identification);
                let id = qualify_name(next_container.as_deref(), &name);
                let filters = match &view_def.body {
                    ViewDefBody::Brace { elements } => elements
                        .iter()
                        .filter_map(|member| match &member.value {
                            ViewDefBodyElement::Filter(filter) => {
                                Some(parse_filter_span(content, &filter.condition.span))
                            }
                            _ => None,
                        })
                        .collect(),
                    ViewDefBody::Semicolon => Vec::new(),
                };
                definitions.insert(id.clone(), ViewDefinitionSpec { id, name, filters });
            }
            PackageBodyElement::ViewUsage(view_usage) => {
                let id = qualify_name(next_container.as_deref(), &view_usage.name);
                let mut filters = Vec::new();
                let mut exposes = Vec::new();
                if let ViewBody::Brace { elements } = &view_usage.body {
                    for member in elements {
                        match &member.value {
                            ViewBodyElement::Filter(filter) => {
                                filters.push(parse_filter_span(content, &filter.condition.span));
                            }
                            ViewBodyElement::Expose(expose) => exposes.push(ExposeSpec {
                                target: expose.target.clone(),
                                filter: parse_expose_filter(content, &member.span),
                            }),
                            _ => {}
                        }
                    }
                }
                usages.push(ViewUsageSpec {
                    id,
                    name: view_usage.name.clone(),
                    definition_ref: view_usage.type_name.clone(),
                    definition_id: None,
                    filters,
                    exposes,
                    range: span_to_range_dto(&view_usage.span),
                    issues: Vec::new(),
                });
            }
            _ => {}
        }
    }
}

fn identification_name(identification: &Identification) -> String {
    identification
        .name
        .clone()
        .or_else(|| identification.short_name.clone())
        .unwrap_or_else(|| "AnonymousView".to_string())
}

fn qualify_name(container: Option<&str>, name: &str) -> String {
    match container {
        Some(prefix) if !prefix.is_empty() => format!("{prefix}::{name}"),
        _ => name.to_string(),
    }
}

fn resolve_definition_id(
    reference: &str,
    definitions: &HashMap<String, ViewDefinitionSpec>,
) -> Option<String> {
    let normalized = normalize_path(reference);
    let mut matches: Vec<_> = definitions
        .keys()
        .filter(|id| {
            let candidate = normalize_path(id);
            candidate == normalized || candidate.ends_with(&format!("::{normalized}"))
        })
        .cloned()
        .collect();
    matches.sort();
    if matches.len() == 1 {
        matches.into_iter().next()
    } else {
        None
    }
}

pub fn evaluate_views(
    catalog: &ViewCatalog,
    graph: &crate::views::dto::SysmlGraphDto,
) -> Vec<EvaluatedView> {
    let node_by_id: HashMap<&str, &crate::views::dto::GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let parent_by_id: HashMap<&str, &str> = graph
        .nodes
        .iter()
        .filter_map(|node| {
            node.parent_id
                .as_deref()
                .map(|parent| (node.id.as_str(), parent))
        })
        .collect();
    let children_by_parent: HashMap<&str, Vec<&str>> = {
        let mut map = HashMap::new();
        for node in &graph.nodes {
            if let Some(parent_id) = node.parent_id.as_deref() {
                map.entry(parent_id)
                    .or_insert_with(Vec::new)
                    .push(node.id.as_str());
            }
        }
        map
    };

    catalog
        .usages
        .iter()
        .map(|usage| {
            let mut issues = usage.issues.clone();
            let mut filters = usage.filters.clone();
            let effective_view_type = usage
                .definition_id
                .as_deref()
                .and_then(|definition_id| catalog.definitions.get(definition_id))
                .map(|definition| definition.name.clone())
                .or_else(|| usage.definition_ref.clone());
            if let Some(definition_id) = usage.definition_id.as_deref() {
                if let Some(definition) = catalog.definitions.get(definition_id) {
                    filters.extend(definition.filters.clone());
                }
            }

            let mut exposed_ids = HashSet::new();
            for expose in &usage.exposes {
                let exposed = resolve_expose_targets(graph, expose, &children_by_parent);
                for node_id in exposed {
                    if node_matches_expose_filter(node_id, &node_by_id, expose.filter.as_ref()) {
                        exposed_ids.insert(node_id.to_string());
                    }
                }
            }

            if usage.exposes.is_empty() {
                issues.push("View has no expose members.".to_string());
            }

            let filtered_ids: HashSet<String> = exposed_ids
                .iter()
                .filter(|node_id| node_matches_all_filters(node_id, &node_by_id, &filters))
                .cloned()
                .collect();
            let closure = with_ancestors(filtered_ids, &parent_by_id);
            EvaluatedView {
                id: usage.id.clone(),
                name: usage.name.clone(),
                effective_view_type,
                exposed_ids,
                filters,
                visible_ids: closure,
                issues,
            }
        })
        .collect()
}

pub fn project_ids_for_renderer(
    evaluated: &EvaluatedView,
    graph: &crate::views::dto::SysmlGraphDto,
    renderer_view: &str,
) -> HashSet<String> {
    let node_by_id: HashMap<&str, &crate::views::dto::GraphNodeDto> = graph
        .nodes
        .iter()
        .map(|node| (node.id.as_str(), node))
        .collect();
    let parent_by_id: HashMap<&str, &str> = graph
        .nodes
        .iter()
        .filter_map(|node| {
            node.parent_id
                .as_deref()
                .map(|parent| (node.id.as_str(), parent))
        })
        .collect();
    let children_by_parent: HashMap<&str, Vec<&str>> = {
        let mut map = HashMap::new();
        for node in &graph.nodes {
            if let Some(parent_id) = node.parent_id.as_deref() {
                map.entry(parent_id)
                    .or_insert_with(Vec::new)
                    .push(node.id.as_str());
            }
        }
        map
    };
    let typing_targets: HashMap<&str, Vec<&str>> = {
        let mut map = HashMap::new();
        for edge in &graph.edges {
            let rel_type = edge.rel_type.to_lowercase();
            if rel_type == "typing" || rel_type == "specializes" {
                map.entry(edge.source.as_str())
                    .or_insert_with(Vec::new)
                    .push(edge.target.as_str());
            }
        }
        map
    };

    let expanded_ids = match renderer_view {
        "general-view" | "interconnection-view" => expand_structural_scope(
            &evaluated.exposed_ids,
            &children_by_parent,
            &typing_targets,
            &node_by_id,
        ),
        _ => evaluated.exposed_ids.clone(),
    };
    let filtered_ids: HashSet<String> = expanded_ids
        .iter()
        .filter(|node_id| node_matches_all_filters(node_id, &node_by_id, &evaluated.filters))
        .cloned()
        .collect();
    with_ancestors(filtered_ids, &parent_by_id)
}

fn resolve_expose_targets<'a>(
    graph: &'a crate::views::dto::SysmlGraphDto,
    expose: &ExposeSpec,
    children_by_parent: &HashMap<&'a str, Vec<&'a str>>,
) -> HashSet<&'a str> {
    let target = normalize_path(&expose.target);
    let recursive = target.ends_with("::**");
    let direct_namespace = target.ends_with("::*");
    let base = target
        .trim_end_matches("::**")
        .trim_end_matches("::*")
        .trim_end_matches("::*");

    let mut matches = HashSet::new();
    for node in &graph.nodes {
        let qualified = normalize_path(&node.id);
        if qualified == base || qualified.ends_with(&format!("::{base}")) {
            if recursive {
                collect_descendants(node.id.as_str(), children_by_parent, &mut matches);
                matches.insert(node.id.as_str());
            } else if direct_namespace {
                if let Some(children) = children_by_parent.get(node.id.as_str()) {
                    matches.extend(children.iter().copied());
                }
            } else {
                matches.insert(node.id.as_str());
            }
        }
    }
    matches
}

fn collect_descendants<'a>(
    node_id: &'a str,
    children_by_parent: &HashMap<&'a str, Vec<&'a str>>,
    out: &mut HashSet<&'a str>,
) {
    if let Some(children) = children_by_parent.get(node_id) {
        for child in children {
            if out.insert(child) {
                collect_descendants(child, children_by_parent, out);
            }
        }
    }
}

#[allow(dead_code)]
fn expand_descendants(
    root_ids: &HashSet<String>,
    children_by_parent: &HashMap<&str, Vec<&str>>,
) -> HashSet<String> {
    let mut expanded = root_ids.clone();
    let mut stack: Vec<String> = root_ids.iter().cloned().collect();
    while let Some(current) = stack.pop() {
        if let Some(children) = children_by_parent.get(current.as_str()) {
            for child in children {
                let child_string = (*child).to_string();
                if expanded.insert(child_string.clone()) {
                    stack.push(child_string);
                }
            }
        }
    }
    expanded
}

fn expand_structural_scope(
    root_ids: &HashSet<String>,
    children_by_parent: &HashMap<&str, Vec<&str>>,
    typing_targets: &HashMap<&str, Vec<&str>>,
    node_by_id: &HashMap<&str, &crate::views::dto::GraphNodeDto>,
) -> HashSet<String> {
    let mut expanded = HashSet::new();
    let mut stack: Vec<String> = root_ids.iter().cloned().collect();

    while let Some(current) = stack.pop() {
        if !expanded.insert(current.clone()) {
            continue;
        }

        if let Some(children) = children_by_parent.get(current.as_str()) {
            for child in children {
                stack.push((*child).to_string());
            }
        }

        let is_part_like = node_by_id
            .get(current.as_str())
            .is_some_and(|node| is_part_like(&node.element_type));
        if is_part_like {
            if let Some(targets) = typing_targets.get(current.as_str()) {
                for target in targets {
                    stack.push((*target).to_string());
                }
            }
        }
    }

    expanded
}

fn is_part_like(element_type: &str) -> bool {
    let lower = element_type.to_lowercase();
    lower.contains("part")
}

fn with_ancestors(
    mut visible_ids: HashSet<String>,
    parent_by_id: &HashMap<&str, &str>,
) -> HashSet<String> {
    let mut stack: Vec<String> = visible_ids.iter().cloned().collect();
    while let Some(current) = stack.pop() {
        if let Some(parent) = parent_by_id.get(current.as_str()) {
            let parent_string = (*parent).to_string();
            if visible_ids.insert(parent_string.clone()) {
                stack.push(parent_string);
            }
        }
    }
    visible_ids
}

fn node_matches_all_filters(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::views::dto::GraphNodeDto>,
    filters: &[FilterExpr],
) -> bool {
    filters
        .iter()
        .all(|filter| match_filter_expr(filter, node_id, node_by_id))
}

fn node_matches_expose_filter(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::views::dto::GraphNodeDto>,
    filter: Option<&FilterExpr>,
) -> bool {
    filter.is_none_or(|expr| match_filter_expr(expr, node_id, node_by_id))
}

fn match_filter_expr(
    filter: &FilterExpr,
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::views::dto::GraphNodeDto>,
) -> bool {
    match filter {
        FilterExpr::Matches(qualified) => node_matches_kind(node_id, qualified, node_by_id),
        FilterExpr::Not(inner) => !match_filter_expr(inner, node_id, node_by_id),
        FilterExpr::And(left, right) => {
            match_filter_expr(left, node_id, node_by_id)
                && match_filter_expr(right, node_id, node_by_id)
        }
        FilterExpr::Or(left, right) => {
            match_filter_expr(left, node_id, node_by_id)
                || match_filter_expr(right, node_id, node_by_id)
        }
        FilterExpr::Unsupported(_) => false,
    }
}

fn node_matches_kind(
    node_id: &str,
    qualified: &str,
    node_by_id: &HashMap<&str, &crate::views::dto::GraphNodeDto>,
) -> bool {
    let wanted = normalize_kind_name(qualified);
    node_by_id.get(node_id).is_some_and(|node| {
        let actual = node.element_type.to_lowercase();
        actual == wanted
            || actual.contains(&wanted)
            || wanted.contains(actual.as_str())
            || actual == map_sysml_kind_alias(&wanted)
    })
}

fn map_sysml_kind_alias(wanted: &str) -> String {
    match wanted {
        "partusage" => "part".to_string(),
        "partdefinition" | "partdef" => "part def".to_string(),
        "connectionusage" => "connection".to_string(),
        "actionusage" => "action".to_string(),
        "actiondefinition" | "actiondef" => "action def".to_string(),
        "portusage" => "port".to_string(),
        "stateusage" => "state".to_string(),
        "statedefinition" | "statedef" => "state def".to_string(),
        other => other.to_string(),
    }
}

fn normalize_kind_name(value: &str) -> String {
    normalize_path(value)
        .split("::")
        .last()
        .unwrap_or(value)
        .replace([' ', '_'], "")
        .to_lowercase()
}

fn normalize_path(value: &str) -> String {
    value
        .replace('.', "::")
        .trim()
        .trim_matches('\'')
        .to_string()
}

fn parse_filter_span(content: &str, span: &Span) -> FilterExpr {
    let text = source_slice(content, span);
    parse_filter_text(&text)
}

fn parse_expose_filter(content: &str, span: &Span) -> Option<FilterExpr> {
    let text = source_slice(content, span);
    let start = text.find('[')?;
    let end = text[start + 1..].find(']')?;
    Some(parse_filter_text(&text[start + 1..start + 1 + end]))
}

fn source_slice(content: &str, span: &Span) -> String {
    let end = span.offset.saturating_add(span.len).min(content.len());
    content
        .get(span.offset..end)
        .unwrap_or("")
        .trim()
        .to_string()
}

fn span_to_range_dto(span: &Span) -> RangeDto {
    let (start_line, start_character, end_line, end_character) = span.to_lsp_range();
    RangeDto {
        start: PositionDto {
            line: start_line,
            character: start_character,
        },
        end: PositionDto {
            line: end_line,
            character: end_character,
        },
    }
}

fn parse_filter_text(text: &str) -> FilterExpr {
    let tokens = tokenize_filter(text);
    let mut parser = FilterParser { tokens, index: 0 };
    parser.parse_expr()
}

#[derive(Debug, Clone)]
enum FilterToken {
    At(String),
    Not,
    And,
    Or,
    LParen,
    RParen,
    Unknown(String),
}

fn tokenize_filter(text: &str) -> Vec<FilterToken> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.trim().chars().collect();
    let mut index = 0;
    while index < chars.len() {
        match chars[index] {
            ' ' | '\t' | '\r' | '\n' => index += 1,
            '(' => {
                tokens.push(FilterToken::LParen);
                index += 1;
            }
            ')' => {
                tokens.push(FilterToken::RParen);
                index += 1;
            }
            '@' => {
                let start = index;
                index += 1;
                while index < chars.len()
                    && (chars[index].is_alphanumeric()
                        || matches!(chars[index], '_' | ':' | '.' | '\''))
                {
                    index += 1;
                }
                tokens.push(FilterToken::At(chars[start + 1..index].iter().collect()));
            }
            _ => {
                let start = index;
                while index < chars.len()
                    && !chars[index].is_whitespace()
                    && !matches!(chars[index], '(' | ')')
                {
                    index += 1;
                }
                let word: String = chars[start..index].iter().collect();
                let normalized = word.to_lowercase();
                tokens.push(match normalized.as_str() {
                    "not" => FilterToken::Not,
                    "and" => FilterToken::And,
                    "or" => FilterToken::Or,
                    _ => FilterToken::Unknown(word),
                });
            }
        }
    }
    tokens
}

struct FilterParser {
    tokens: Vec<FilterToken>,
    index: usize,
}

impl FilterParser {
    fn parse_expr(&mut self) -> FilterExpr {
        self.parse_or()
    }

    fn parse_or(&mut self) -> FilterExpr {
        let mut expr = self.parse_and();
        while self.matches(|token| matches!(token, FilterToken::Or)) {
            let rhs = self.parse_and();
            expr = FilterExpr::Or(Box::new(expr), Box::new(rhs));
        }
        expr
    }

    fn parse_and(&mut self) -> FilterExpr {
        let mut expr = self.parse_unary();
        while self.matches(|token| matches!(token, FilterToken::And)) {
            let rhs = self.parse_unary();
            expr = FilterExpr::And(Box::new(expr), Box::new(rhs));
        }
        expr
    }

    fn parse_unary(&mut self) -> FilterExpr {
        if self.matches(|token| matches!(token, FilterToken::Not)) {
            return FilterExpr::Not(Box::new(self.parse_unary()));
        }
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> FilterExpr {
        match self.peek().cloned() {
            Some(FilterToken::At(value)) => {
                self.index += 1;
                FilterExpr::Matches(value)
            }
            Some(FilterToken::LParen) => {
                self.index += 1;
                let expr = self.parse_expr();
                if self.matches(|token| matches!(token, FilterToken::RParen)) {
                    expr
                } else {
                    FilterExpr::Unsupported("missing ')' in filter expression".to_string())
                }
            }
            Some(FilterToken::Unknown(text)) => {
                self.index += 1;
                FilterExpr::Unsupported(text)
            }
            _ => FilterExpr::Unsupported("empty filter expression".to_string()),
        }
    }

    fn matches(&mut self, predicate: impl FnOnce(&FilterToken) -> bool) -> bool {
        if let Some(token) = self.peek() {
            if predicate(token) {
                self.index += 1;
                return true;
            }
        }
        false
    }

    fn peek(&self) -> Option<&FilterToken> {
        self.tokens.get(self.index)
    }
}

pub fn build_view_candidates(
    evaluated_views: &[EvaluatedView],
    _projected_activity_diagrams: &HashMap<&str, Vec<ActivityDiagramDto>>,
    _projected_workspace_graphs: &HashMap<&str, crate::views::dto::SysmlGraphDto>,
) -> Vec<SysmlVisualizationViewCandidateDto> {
    let mut candidates: Vec<_> = evaluated_views
        .iter()
        .map(|evaluated| {
            let renderer_view =
                renderer_view_for_view_type(evaluated.effective_view_type.as_deref());
            let supported = renderer_view.is_some();
            SysmlVisualizationViewCandidateDto {
                id: evaluated.id.clone(),
                name: evaluated.name.clone(),
                renderer_view: renderer_view.map(ToString::to_string),
                supported,
                view_type: evaluated.effective_view_type.clone(),
                description: (!evaluated.issues.is_empty()).then(|| evaluated.issues.join(" ")),
            }
        })
        .collect();
    candidates.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| right.supported.cmp(&left.supported))
            .then_with(|| left.renderer_view.cmp(&right.renderer_view))
            .then_with(|| left.id.cmp(&right.id))
    });
    candidates
}

pub fn renderer_view_for_view_type(effective_view_type: Option<&str>) -> Option<&'static str> {
    let view_type = effective_view_type?;
    let normalized = normalize_kind_name(view_type);
    match normalized.as_str() {
        "generalview" => Some("general-view"),
        "interconnectionview" => Some("interconnection-view"),
        "actionflowview" | "actionview" => Some("action-flow-view"),
        "sequenceview" => Some("sequence-view"),
        "statetransitionview" | "stateview" => Some("state-transition-view"),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{
        build_view_candidates, build_view_catalog, parse_filter_text, project_ids_for_renderer,
        EvaluatedView, FilterExpr,
    };
    use crate::views::dto::{GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
    use crate::workspace::state::{IndexEntry, ParseMetadata};
    use std::collections::{HashMap, HashSet};
    use sysml_v2_parser::parse;
    use tower_lsp::lsp_types::Url;

    #[test]
    fn extracts_view_definitions_and_usages_with_filters_and_expose() {
        let uri = Url::parse("file:///C:/demo/model.sysml").expect("uri");
        let content = r#"
            package Demo {
                view def StructuralView {
                    filter @SysML::PartUsage and not @SysML::ConnectionUsage;
                }

                view VehicleView : StructuralView {
                    expose Demo::Vehicle::**[not @SysML::PortUsage];
                }
            }
        "#;
        let parsed = parse(content).expect("parse");
        let index = HashMap::from([(
            uri.clone(),
            IndexEntry {
                content: content.to_string(),
                parsed: Some(parsed),
                parse_metadata: ParseMetadata::default(),
            },
        )]);

        let catalog = build_view_catalog(&index, &[uri]);
        assert_eq!(catalog.definitions.len(), 1);
        assert_eq!(catalog.usages.len(), 1);
        assert_eq!(
            catalog.usages[0].definition_ref.as_deref(),
            Some("StructuralView")
        );
        assert_eq!(catalog.usages[0].exposes.len(), 1);
        assert!(catalog.usages[0].exposes[0].filter.is_some());
    }

    #[test]
    fn parses_supported_filter_subset() {
        let parsed = parse_filter_text(
            "@SysML::PartUsage and not (@SysML::ConnectionUsage or @SysML::PortUsage)",
        );
        match parsed {
            FilterExpr::And(_, right) => match *right {
                FilterExpr::Not(_) => {}
                other => panic!("expected unary not, got {other:?}"),
            },
            other => panic!("expected conjunction, got {other:?}"),
        }
    }

    #[test]
    fn includes_unsupported_view_types_in_candidates() {
        let evaluated_views = vec![
            EvaluatedView {
                id: "Demo::Supported".to_string(),
                name: "Supported".to_string(),
                effective_view_type: Some("GeneralView".to_string()),
                exposed_ids: HashSet::new(),
                filters: Vec::new(),
                visible_ids: HashSet::new(),
                issues: Vec::new(),
            },
            EvaluatedView {
                id: "Demo::Safety".to_string(),
                name: "Safety".to_string(),
                effective_view_type: Some("SafetyView".to_string()),
                exposed_ids: HashSet::new(),
                filters: Vec::new(),
                visible_ids: HashSet::new(),
                issues: Vec::new(),
            },
        ];

        let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
        assert_eq!(candidates.len(), 2);
        assert_eq!(candidates[0].name, "Safety");
        assert!(!candidates[0].supported);
        assert_eq!(candidates[0].renderer_view, None);
        assert_eq!(candidates[1].name, "Supported");
        assert!(candidates[1].supported);
        assert_eq!(candidates[1].renderer_view.as_deref(), Some("general-view"));
    }

    #[test]
    fn sequence_view_type_maps_to_sequence_renderer() {
        let evaluated_views = vec![EvaluatedView {
            id: "Demo::CheckoutSequence".to_string(),
            name: "Checkout Sequence".to_string(),
            effective_view_type: Some("SequenceView".to_string()),
            exposed_ids: HashSet::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        }];

        let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
        assert_eq!(candidates.len(), 1);
        assert!(candidates[0].supported);
        assert_eq!(candidates[0].renderer_view.as_deref(), Some("sequence-view"));
    }

    #[test]
    fn general_view_projection_expands_exposed_roots_to_owned_members() {
        fn zero_range() -> RangeDto {
            RangeDto {
                start: PositionDto {
                    line: 0,
                    character: 0,
                },
                end: PositionDto {
                    line: 0,
                    character: 0,
                },
            }
        }

        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Office::OfficeDeskSetup".to_string(),
                    element_type: "part def".to_string(),
                    name: "OfficeDeskSetup".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Office::OfficeDeskSetup::laptop".to_string(),
                    element_type: "part".to_string(),
                    name: "laptop".to_string(),
                    uri: None,
                    parent_id: Some("Office::OfficeDeskSetup".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: Vec::new(),
        };
        let evaluated = EvaluatedView {
            id: "Office::structure".to_string(),
            name: "structure".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::from(["Office::OfficeDeskSetup".to_string()]),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_ids_for_renderer(&evaluated, &graph, "general-view");
        assert!(projected.contains("Office::OfficeDeskSetup"));
        assert!(projected.contains("Office::OfficeDeskSetup::laptop"));
    }

    #[test]
    fn structural_projection_recursively_expands_typed_part_definitions() {
        fn zero_range() -> RangeDto {
            RangeDto {
                start: PositionDto {
                    line: 0,
                    character: 0,
                },
                end: PositionDto {
                    line: 0,
                    character: 0,
                },
            }
        }

        let graph = SysmlGraphDto {
            nodes: vec![
                GraphNodeDto {
                    id: "Pkg::System".to_string(),
                    element_type: "part def".to_string(),
                    name: "System".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::System::engine".to_string(),
                    element_type: "part".to_string(),
                    name: "engine".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::System".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Engine".to_string(),
                    element_type: "part def".to_string(),
                    name: "Engine".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Engine::pump".to_string(),
                    element_type: "part".to_string(),
                    name: "pump".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Engine".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Pump".to_string(),
                    element_type: "part def".to_string(),
                    name: "Pump".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                crate::views::dto::GraphEdgeDto {
                    source: "Pkg::System::engine".to_string(),
                    target: "Pkg::Engine".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
                crate::views::dto::GraphEdgeDto {
                    source: "Pkg::Engine::pump".to_string(),
                    target: "Pkg::Pump".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
            ],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::view".to_string(),
            name: "view".to_string(),
            effective_view_type: Some("InterconnectionView".to_string()),
            exposed_ids: HashSet::from(["Pkg::System".to_string()]),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_ids_for_renderer(&evaluated, &graph, "interconnection-view");
        assert!(projected.contains("Pkg::System::engine"));
        assert!(projected.contains("Pkg::Engine"));
        assert!(projected.contains("Pkg::Engine::pump"));
        assert!(projected.contains("Pkg::Pump"));
    }
}
