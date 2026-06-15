use std::collections::{HashMap, HashSet};

use url::Url;

use crate::semantic::dto::{PositionDto, RangeDto, SysmlVisualizationViewCandidateDto};
use crate::semantic::extracted_model::ActivityDiagramDto;
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
pub struct ViewDefinitionSpec {
    #[allow(dead_code)]
    pub id: String,
    pub name: String,
    pub filters: Vec<FilterExpr>,
    pub rendering_ref: Option<String>,
    pub rendering_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ExposeSpec {
    pub target: String,
    pub filter: Option<FilterExpr>,
    pub range: RangeDto,
}

#[derive(Debug, Clone)]
pub struct ViewUsageSpec {
    pub id: String,
    pub name: String,
    pub definition_ref: Option<String>,
    pub definition_id: Option<String>,
    pub filters: Vec<FilterExpr>,
    pub exposes: Vec<ExposeSpec>,
    pub conforms_to: Vec<String>,
    pub rendering_ref: Option<String>,
    pub rendering_type: Option<String>,
    #[allow(dead_code)]
    pub range: RangeDto,
    pub issues: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ViewCatalog {
    pub definitions: HashMap<String, ViewDefinitionSpec>,
    pub usages: Vec<ViewUsageSpec>,
}

#[derive(Debug, Clone)]
pub struct EvaluatedView {
    pub id: String,
    pub name: String,
    pub effective_view_type: Option<String>,
    pub exposed_ids: HashSet<String>,
    pub conforms_to: Vec<String>,
    pub filters: Vec<FilterExpr>,
    #[allow(dead_code)]
    pub visible_ids: HashSet<String>,
    pub issues: Vec<String>,
}

/// Build a catalog of view definitions and usages from parsed workspace documents.
///
/// Callers (for example the LSP server) typically derive [`WorkspaceParsedDocument`] values
/// from their on-disk index; the graph-first workspace builder passes the documents returned
/// alongside [`crate::semantic::graph::SemanticGraph`].
pub fn build_view_catalog(
    workspace_uris: &[Url],
    documents: &[crate::semantic::workspace_graph::WorkspaceParsedDocument],
) -> ViewCatalog {
    let mut definitions = HashMap::new();
    let mut usages = Vec::new();
    for workspace_uri in workspace_uris {
        let Some(doc) = documents.iter().find(|doc| &doc.uri == workspace_uri) else {
            continue;
        };
        walk_root_namespace(
            &doc.parsed,
            doc.content.as_str(),
            None,
            &mut definitions,
            &mut usages,
        );
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
                let (filters, rendering_ref, rendering_type) = match &view_def.body {
                    ViewDefBody::Brace { elements } => {
                        let filters = elements
                            .iter()
                            .filter_map(|member| match &member.value {
                                ViewDefBodyElement::Filter(filter) => {
                                    Some(parse_filter_span(content, &filter.condition.span))
                                }
                                _ => None,
                            })
                            .collect();
                        let (rendering_ref, rendering_type) =
                            extract_rendering_from_view_def_body(elements);
                        (filters, rendering_ref, rendering_type)
                    }
                    ViewDefBody::Semicolon => (Vec::new(), None, None),
                };
                definitions.insert(
                    id.clone(),
                    ViewDefinitionSpec {
                        id,
                        name,
                        filters,
                        rendering_ref,
                        rendering_type,
                    },
                );
            }
            PackageBodyElement::ViewUsage(view_usage) => {
                let id = qualify_name(next_container.as_deref(), &view_usage.name);
                let mut filters = Vec::new();
                let mut exposes = Vec::new();
                let mut rendering_ref = None;
                let mut rendering_type = None;
                if let ViewBody::Brace { elements } = &view_usage.body {
                    for member in elements {
                        match &member.value {
                            ViewBodyElement::Filter(filter) => {
                                filters.push(parse_filter_span(content, &filter.condition.span));
                            }
                            ViewBodyElement::Expose(expose) => exposes.push(ExposeSpec {
                                target: expose.target.clone(),
                                filter: parse_expose_filter(content, &member.span),
                                range: span_to_range_dto(&member.span),
                            }),
                            ViewBodyElement::ViewRendering(rendering) => {
                                if rendering_ref.is_none() {
                                    rendering_ref = Some(rendering.value.name.clone());
                                    rendering_type = rendering.value.type_name.clone();
                                }
                            }
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
                    conforms_to: Vec::new(),
                    rendering_ref,
                    rendering_type,
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

fn extract_rendering_from_view_def_body(
    elements: &[sysml_v2_parser::ast::Node<ViewDefBodyElement>],
) -> (Option<String>, Option<String>) {
    for member in elements {
        if let ViewDefBodyElement::ViewRendering(rendering) = &member.value {
            return (
                Some(rendering.value.name.clone()),
                rendering.value.type_name.clone(),
            );
        }
    }
    (None, None)
}

fn resolve_explicit_view_type(usage: &ViewUsageSpec, _catalog: &ViewCatalog) -> Option<String> {
    if usage.definition_id.is_some() {
        return None;
    }
    let type_ref = usage.definition_ref.as_deref()?;
    if renderer_view_for_view_type(Some(type_ref)).is_some() {
        Some(type_ref.to_string())
    } else {
        None
    }
}

fn view_type_for_stdlib_rendering(
    rendering_ref: Option<&str>,
    rendering_type: Option<&str>,
) -> Option<&'static str> {
    let lookup = rendering_type.or(rendering_ref)?;
    match normalize_kind_name(lookup).as_str() {
        "asinterconnectiondiagram" => Some("InterconnectionView"),
        "astreediagram" => Some("BrowserView"),
        "aselementtable" => Some("GridView"),
        "astextualnotation" => Some("GeneralView"),
        _ => None,
    }
}

fn resolve_effective_view_type(usage: &ViewUsageSpec, catalog: &ViewCatalog) -> String {
    if let Some(explicit) = resolve_explicit_view_type(usage, catalog) {
        return explicit;
    }
    if usage.definition_id.is_none() {
        if let Some(type_ref) = usage.definition_ref.as_deref() {
            if crate::semantic::standard_views::is_non_standard_explicit_view_type(type_ref) {
                return type_ref.to_string();
            }
        }
    }
    view_type_for_stdlib_rendering(
        usage.rendering_ref.as_deref(),
        usage.rendering_type.as_deref(),
    )
    .or_else(|| {
        usage.definition_id.as_deref().and_then(|definition_id| {
            catalog.definitions.get(definition_id).and_then(|definition| {
                view_type_for_stdlib_rendering(
                    definition.rendering_ref.as_deref(),
                    definition.rendering_type.as_deref(),
                )
            })
        })
    })
    .map(str::to_string)
    .unwrap_or_else(|| "GeneralView".to_string())
}

pub fn evaluate_views(
    catalog: &ViewCatalog,
    semantic_graph: &crate::semantic::graph::SemanticGraph,
    graph: &crate::semantic::dto::SysmlGraphDto,
) -> Vec<EvaluatedView> {
    let node_by_id: HashMap<&str, &crate::semantic::dto::GraphNodeDto> = graph
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
    catalog
        .usages
        .iter()
        .map(|usage| {
            let mut issues = usage.issues.clone();
            let mut filters = usage.filters.clone();
            let mut conforms_to = usage.conforms_to.clone();
            let effective_view_type = Some(resolve_effective_view_type(usage, catalog));
            if usage.definition_id.is_none() {
                if let Some(type_ref) = usage.definition_ref.as_deref() {
                    if crate::semantic::standard_views::is_non_standard_explicit_view_type(type_ref)
                    {
                        issues.push(format!(
                            "View type '{type_ref}' is not a SysML v2 standard view definition (§9.2.20 Table 34); use GeneralView with filters, a render clause, or a local view def."
                        ));
                    }
                }
            }
            if let Some(definition_id) = usage.definition_id.as_deref() {
                if let Some(definition) = catalog.definitions.get(definition_id) {
                    filters.extend(definition.filters.clone());
                }
            }
            for expose in &usage.exposes {
                if let Some(filter) = &expose.filter {
                    filters.push(filter.clone());
                }
            }

            let view_uri = uri_for_qualified_name(semantic_graph, &usage.id)
                .or_else(|| {
                    node_by_id
                        .get(usage.id.as_str())
                        .and_then(|node| node.uri.clone())
                })
                .and_then(|uri| url::Url::parse(&uri).ok());
            let container_prefix = usage.id.rsplit_once("::").map(|(prefix, _)| prefix);

            let mut exposed_ids = HashSet::new();
            for expose in &usage.exposes {
                match crate::semantic::reference_resolution::resolve_expose_target(
                    semantic_graph,
                    view_uri.as_ref(),
                    container_prefix,
                    &expose.target,
                ) {
                    crate::semantic::reference_resolution::ExposeTargetResolution::Resolved(
                        names,
                    ) => {
                        for node_id in names {
                            if node_matches_expose_filter(
                                node_id.as_str(),
                                &node_by_id,
                                expose.filter.as_ref(),
                            ) {
                                exposed_ids.insert(node_id);
                            }
                        }
                    }
                    crate::semantic::reference_resolution::ExposeTargetResolution::Ambiguous => {
                        issues.push(format!("Expose target '{}' is ambiguous.", expose.target));
                    }
                    crate::semantic::reference_resolution::ExposeTargetResolution::Unresolved => {
                        issues.push(format!(
                            "Expose target '{}' does not resolve to any element.",
                            expose.target
                        ));
                    }
                }
            }

            if usage.exposes.is_empty() {
                issues.push("View has no expose members.".to_string());
            }
            if let Some(view_node) = node_by_id.get(usage.id.as_str()) {
                for edge in &graph.edges {
                    if edge.rel_type != "satisfy" || edge.source != view_node.id {
                        continue;
                    }
                    let Some(target) = node_by_id.get(edge.target.as_str()) else {
                        continue;
                    };
                    let target_kind = target.element_type.as_str();
                    if target_kind == "viewpoint" || target_kind == "viewpoint def" {
                        conforms_to.push(target.id.clone());
                    }
                }
            }
            conforms_to.sort();
            conforms_to.dedup();

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
                conforms_to,
                filters,
                visible_ids: closure,
                issues,
            }
        })
        .collect()
}

pub fn project_ids_for_renderer(
    evaluated: &EvaluatedView,
    graph: &crate::semantic::dto::SysmlGraphDto,
    _renderer_view: &str,
) -> HashSet<String> {
    crate::semantic::view_projection::project_ids_for_renderer(evaluated, graph)
}

fn uri_for_qualified_name(
    semantic_graph: &crate::semantic::graph::SemanticGraph,
    qualified_name: &str,
) -> Option<String> {
    semantic_graph
        .graph
        .node_weights()
        .find(|node| node.id.qualified_name == qualified_name)
        .map(|node| node.id.uri.as_str().to_string())
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

pub(crate) fn node_matches_all_filters(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filters: &[FilterExpr],
) -> bool {
    filters
        .iter()
        .all(|filter| match_filter_expr(filter, node_id, node_by_id))
}

fn node_matches_expose_filter(
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
    filter: Option<&FilterExpr>,
) -> bool {
    filter.is_none_or(|expr| match_filter_expr(expr, node_id, node_by_id))
}

fn match_filter_expr(
    filter: &FilterExpr,
    node_id: &str,
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
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
    node_by_id: &HashMap<&str, &crate::semantic::dto::GraphNodeDto>,
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
        "portdefinition" | "portdef" => "port def".to_string(),
        "connectiondefinition" | "connectiondef" => "connection def".to_string(),
        "stateusage" => "state".to_string(),
        "statedefinition" | "statedef" => "state def".to_string(),
        "metadatausage" => "metadata usage".to_string(),
        "requirementusage" => "requirement".to_string(),
        "verificationcase" => "verification".to_string(),
        "analysiscase" => "analysis".to_string(),
        "package" => "package".to_string(),
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
    _projected_workspace_graphs: &HashMap<&str, crate::semantic::dto::SysmlGraphDto>,
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
    crate::semantic::standard_views::renderer_for_standard_view_type(view_type)
}

#[cfg(test)]
mod tests {
    use super::{
        build_view_candidates, build_view_catalog, parse_filter_text, project_ids_for_renderer,
        EvaluatedView, FilterExpr,
    };
    use crate::semantic::dto::{GraphEdgeDto, GraphNodeDto, PositionDto, RangeDto, SysmlGraphDto};
    use crate::semantic::workspace_graph::WorkspaceParsedDocument;
    use std::collections::{HashMap, HashSet};
    use sysml_v2_parser::parse;
    use url::Url;

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
        let doc = WorkspaceParsedDocument {
            uri: uri.clone(),
            content: content.to_string(),
            parsed,
            parse_time_ms: 1,
            parse_cached: false,
        };

        let catalog = build_view_catalog(std::slice::from_ref(&uri), std::slice::from_ref(&doc));
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
                conforms_to: Vec::new(),
                filters: Vec::new(),
                visible_ids: HashSet::new(),
                issues: Vec::new(),
            },
            EvaluatedView {
                id: "Demo::Safety".to_string(),
                name: "Safety".to_string(),
                effective_view_type: Some("SafetyView".to_string()),
                exposed_ids: HashSet::new(),
                conforms_to: Vec::new(),
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
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        }];

        let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
        assert_eq!(candidates.len(), 1);
        assert!(candidates[0].supported);
        assert_eq!(
            candidates[0].renderer_view.as_deref(),
            Some("sequence-view")
        );
    }

    #[test]
    fn standard_view_types_map_to_shared_renderers() {
        let cases = [
            ("GeneralView", Some("general-view")),
            ("InterconnectionView", Some("interconnection-view")),
            ("ActionFlowView", Some("action-flow-view")),
            ("SequenceView", Some("sequence-view")),
            ("StateTransitionView", Some("state-transition-view")),
            ("BrowserView", Some("browser-view")),
            ("GridView", Some("grid-view")),
            ("GeometryView", Some("geometry-view")),
            ("RequirementView", None),
            ("CaseView", None),
            ("SafetyView", None),
        ];
        for (view_type, expected) in cases {
            assert_eq!(
                super::renderer_view_for_view_type(Some(view_type)),
                expected,
                "{view_type}"
            );
        }
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
            conforms_to: Vec::new(),
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
                crate::semantic::dto::GraphEdgeDto {
                    source: "Pkg::System::engine".to_string(),
                    target: "Pkg::Engine".to_string(),
                    rel_type: "typing".to_string(),
                    name: None,
                },
                crate::semantic::dto::GraphEdgeDto {
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
            conforms_to: Vec::new(),
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

    #[test]
    fn browser_view_projection_applies_expose_kind_filters_after_expansion() {
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
                    id: "Pkg::Robot".to_string(),
                    element_type: "part def".to_string(),
                    name: "Robot".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::robot".to_string(),
                    element_type: "part".to_string(),
                    name: "robot".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Robot::chassis".to_string(),
                    element_type: "part".to_string(),
                    name: "chassis".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Robot".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::Robot::powerPort".to_string(),
                    element_type: "port".to_string(),
                    name: "powerPort".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::Robot".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![crate::semantic::dto::GraphEdgeDto {
                source: "Pkg::robot".to_string(),
                target: "Pkg::Robot".to_string(),
                rel_type: "typing".to_string(),
                name: None,
            }],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::structure".to_string(),
            name: "structure".to_string(),
            effective_view_type: Some("BrowserView".to_string()),
            exposed_ids: HashSet::from(["Pkg::robot".to_string()]),
            conforms_to: Vec::new(),
            filters: vec![FilterExpr::Matches("@SysML::PartUsage".to_string())],
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_ids_for_renderer(&evaluated, &graph, "browser-view");
        assert!(projected.contains("Pkg::robot"));
        assert!(projected.contains("Pkg::Robot::chassis"));
        assert!(
            !projected.contains("Pkg::Robot::powerPort"),
            "PartUsage filter should exclude ports after expansion"
        );
    }

    #[test]
    fn requirement_view_projection_follows_traceability_links_without_structural_expansion() {
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
                    id: "Pkg::need".to_string(),
                    element_type: "requirement".to_string(),
                    name: "need".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::req".to_string(),
                    element_type: "requirement".to_string(),
                    name: "req".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::design".to_string(),
                    element_type: "action".to_string(),
                    name: "design".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::unrelatedPart".to_string(),
                    element_type: "part".to_string(),
                    name: "unrelatedPart".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![
                GraphEdgeDto {
                    source: "Pkg::need".to_string(),
                    target: "Pkg::req".to_string(),
                    rel_type: "derivation".to_string(),
                    name: None,
                },
                GraphEdgeDto {
                    source: "Pkg::design".to_string(),
                    target: "Pkg::req".to_string(),
                    rel_type: "satisfy".to_string(),
                    name: None,
                },
            ],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::trace".to_string(),
            name: "trace".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::from(["Pkg::need".to_string(), "Pkg::design".to_string()]),
            conforms_to: Vec::new(),
            filters: vec![FilterExpr::Matches("@SysML::RequirementUsage".to_string())],
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_ids_for_renderer(&evaluated, &graph, "general-view");
        assert!(projected.contains("Pkg::need"));
        assert!(projected.contains("Pkg::req"));
        assert!(projected.contains("Pkg::design"));
        assert!(
            !projected.contains("Pkg::unrelatedPart"),
            "traceability projection should not structurally expand unrelated elements"
        );
    }

    #[test]
    fn extracts_rendering_from_view_usage() {
        let uri = Url::parse("file:///C:/demo/model.sysml").expect("uri");
        let content = r#"
            package Demo {
                part def System { part child; }
                part system : System;
                view connections {
                    expose Demo::system;
                    render asInterconnectionDiagram;
                }
            }
        "#;
        let parsed = parse(content).expect("parse");
        let doc = WorkspaceParsedDocument {
            uri: uri.clone(),
            content: content.to_string(),
            parsed,
            parse_time_ms: 1,
            parse_cached: false,
        };

        let catalog = build_view_catalog(std::slice::from_ref(&uri), std::slice::from_ref(&doc));
        assert_eq!(catalog.usages.len(), 1);
        assert_eq!(
            catalog.usages[0].rendering_ref.as_deref(),
            Some("asInterconnectionDiagram")
        );
    }

    #[test]
    fn stdlib_rendering_maps_to_view_type_and_renderer() {
        assert_eq!(
            super::view_type_for_stdlib_rendering(Some("asInterconnectionDiagram"), None),
            Some("InterconnectionView")
        );
        assert_eq!(
            super::renderer_view_for_view_type(Some("InterconnectionView")),
            Some("interconnection-view")
        );
        assert_eq!(
            super::view_type_for_stdlib_rendering(Some("asTreeDiagram"), None),
            Some("BrowserView")
        );
        assert_eq!(
            super::view_type_for_stdlib_rendering(Some("asElementTable"), None),
            Some("GridView")
        );
    }

    #[test]
    fn rendering_only_view_is_supported_candidate() {
        let evaluated_views = vec![EvaluatedView {
            id: "Demo::connections".to_string(),
            name: "connections".to_string(),
            effective_view_type: Some("InterconnectionView".to_string()),
            exposed_ids: HashSet::new(),
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        }];

        let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
        assert!(candidates[0].supported);
        assert_eq!(
            candidates[0].renderer_view.as_deref(),
            Some("interconnection-view")
        );
    }

    #[test]
    fn untyped_view_without_render_falls_back_to_general_view_candidate() {
        let evaluated_views = vec![EvaluatedView {
            id: "Demo::overview".to_string(),
            name: "overview".to_string(),
            effective_view_type: Some("GeneralView".to_string()),
            exposed_ids: HashSet::new(),
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        }];

        let candidates = build_view_candidates(&evaluated_views, &HashMap::new(), &HashMap::new());
        assert!(candidates[0].supported);
        assert_eq!(candidates[0].renderer_view.as_deref(), Some("general-view"));
    }

    #[test]
    fn state_transition_projection_expands_exposed_machine_descendants() {
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
                    id: "Pkg::OrderLifecycle".to_string(),
                    element_type: "state def".to_string(),
                    name: "OrderLifecycle".to_string(),
                    uri: None,
                    parent_id: None,
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::OrderLifecycle::created".to_string(),
                    element_type: "state".to_string(),
                    name: "created".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::OrderLifecycle".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
                GraphNodeDto {
                    id: "Pkg::OrderLifecycle::paid".to_string(),
                    element_type: "state".to_string(),
                    name: "paid".to_string(),
                    uri: None,
                    parent_id: Some("Pkg::OrderLifecycle".to_string()),
                    range: zero_range(),
                    attributes: HashMap::new(),
                },
            ],
            edges: vec![crate::semantic::dto::GraphEdgeDto {
                source: "Pkg::OrderLifecycle::created".to_string(),
                target: "Pkg::OrderLifecycle::paid".to_string(),
                rel_type: "transition".to_string(),
                name: Some("to_paid".to_string()),
            }],
        };
        let evaluated = EvaluatedView {
            id: "Pkg::orderLifecycle".to_string(),
            name: "orderLifecycle".to_string(),
            effective_view_type: Some("StateTransitionView".to_string()),
            exposed_ids: HashSet::from(["Pkg::OrderLifecycle".to_string()]),
            conforms_to: Vec::new(),
            filters: Vec::new(),
            visible_ids: HashSet::new(),
            issues: Vec::new(),
        };

        let projected = project_ids_for_renderer(&evaluated, &graph, "state-transition-view");
        assert!(projected.contains("Pkg::OrderLifecycle"));
        assert!(projected.contains("Pkg::OrderLifecycle::created"));
        assert!(projected.contains("Pkg::OrderLifecycle::paid"));
    }
}
