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


mod catalog_build;
mod evaluate;
mod filter_match;
mod filter_parser;
pub(crate) use catalog_build::*;
pub use evaluate::*;
pub(crate) use filter_match::*;
pub(crate) use filter_parser::*;

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

pub(crate) fn parse_filter_text(text: &str) -> FilterExpr {
    let tokens = tokenize_filter(text);
    let mut parser = FilterParser { tokens, index: 0 };
    parser.parse_expr()
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
mod tests;
