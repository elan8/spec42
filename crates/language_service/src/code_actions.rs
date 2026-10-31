//! Quick fixes for unresolved, ambiguous and implicitly redefining declarations.
//!
//! What a fix proposes is a typed repair the published model answers; where its text goes is read
//! from the syntax tree of the source that model was published from. A document whose editor text
//! has moved past that source gets no fix, because the answer's positions address older text.

use sysml_query::resolved_slice::{
    ElementKind, ImportCandidate, QueryAnswer, ReferenceRepairOutcome, RepairNamespace, RepairSlot,
    TextPosition, TextRange,
};
use sysml_query::syntax::{ParsedSource, SyntaxOutlineKind, SyntaxOutlineNode, SyntaxRange};
use url::Url;

use crate::dto::{TextEditDto, TextEditSuggestion};
use crate::text::utf16_len;
use crate::workspace::WorkspaceSnapshot;

/// The fixes for the unresolved or ambiguous reference at `position`.
///
/// An unresolved name gets one "import" fix per definition an import makes visible, preferred
/// only when there is exactly one; failing any, a typing or specialization slot gets a fix that
/// declares the missing definition. An ambiguous name gets one fix per qualified spelling that
/// names exactly one candidate.
pub fn reference_quick_fixes(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Vec<TextEditSuggestion> {
    let Some(document) = PublishedDocument::current(workspace, document_path) else {
        return Vec::new();
    };
    let Some(parsed) = workspace.parsed(&document.uri) else {
        return Vec::new();
    };
    let QueryAnswer::Resolved(repair) = document
        .model
        .edits()
        .reference_repair(document.uri.as_str(), position)
        .answer
    else {
        return Vec::new();
    };
    match repair.outcome {
        ReferenceRepairOutcome::Unresolved {
            name,
            imports,
            namespace,
            expected_definition,
        } => {
            if !imports.is_empty() {
                let preferred = imports.len() == 1;
                return imports
                    .iter()
                    .filter_map(|candidate| {
                        document.import_fix(&parsed, namespace.as_ref(), candidate, preferred)
                    })
                    .collect();
            }
            if repair.slot == RepairSlot::Other || name.is_none() {
                return Vec::new();
            }
            expected_definition
                .and_then(|kind| {
                    document.create_definition_fix(&parsed, repair.range, namespace, kind)
                })
                .into_iter()
                .collect()
        }
        ReferenceRepairOutcome::Ambiguous { candidates } => {
            let preferred = candidates.len() == 1;
            candidates
                .iter()
                .filter_map(|candidate| {
                    let qualified = document.model.element_qualified_name(candidate.target)?;
                    Some(
                        TextEditSuggestion::new(
                            format!("Qualify as `{qualified}`"),
                            vec![document.edit(repair.range, qualified)],
                        )
                        .with_preferred(preferred),
                    )
                })
                .collect()
        }
    }
}

/// The fix that writes `:>>` before a declaration whose value silently overrides the inherited
/// member its name implicitly redefines.
pub fn explicit_redefinition_quick_fix(
    workspace: &impl WorkspaceSnapshot,
    document_path: &str,
    position: TextPosition,
) -> Option<TextEditSuggestion> {
    let document = PublishedDocument::current(workspace, document_path)?;
    let QueryAnswer::Resolved(repair) = document
        .model
        .edits()
        .implicit_redefinition_repair(document.uri.as_str(), position)
        .answer
    else {
        return None;
    };
    let at = repair.identifier.start;
    Some(TextEditSuggestion::new(
        "Make the redefinition explicit with `:>>`",
        vec![document.edit(TextRange::new(at, at), ":>> ".to_string())],
    ))
}

/// One document as the publication saw it: its text and the model published from it.
struct PublishedDocument<'w> {
    uri: Url,
    path: String,
    text: &'w str,
    model: &'w sysml_query::resolved_slice::PublishedModel,
}

impl<'w> PublishedDocument<'w> {
    fn current(workspace: &'w impl WorkspaceSnapshot, document_path: &str) -> Option<Self> {
        let uri = workspace.resolve_uri_for_path(document_path)?;
        let text = workspace.document_text(&uri)?;
        if workspace.editor_text(&uri) != Some(text) {
            return None;
        }
        Some(Self {
            path: workspace.path_for_uri(&uri),
            model: workspace.published_model()?,
            uri,
            text,
        })
    }

    fn edit(&self, range: TextRange, replacement: String) -> TextEditDto {
        TextEditDto {
            path: self.path.clone(),
            range,
            replacement,
        }
    }

    fn insert(&self, at: TextPosition, text: String) -> TextEditDto {
        self.edit(TextRange::new(at, at), text)
    }

    fn line(&self, line: u32) -> &str {
        self.text.lines().nth(line as usize).unwrap_or_default()
    }

    fn position(&self, line: u32, characters: u32) -> TextPosition {
        TextPosition::new(line, characters)
    }

    fn indent(&self, line: u32) -> &str {
        let line = self.line(line);
        &line[..line.len() - line.trim_start().len()]
    }

    /// Whether only whitespace follows `characters` on `line`.
    fn rest_is_blank(&self, line: u32, characters: u32) -> bool {
        crate::text::position_to_byte_offset(self.line(line), 0, characters)
            .is_some_and(|offset| self.line(line)[offset..].chars().all(char::is_whitespace))
    }

    /// The indentation of a member of `body`: that of its first non-blank line, or one level
    /// deeper than the line that opens it.
    fn member_indent(&self, body: &SyntaxRange) -> String {
        (body.start_line + 1..body.end_line)
            .find(|line| !self.line(*line).trim().is_empty())
            .map(|line| self.indent(line).to_string())
            .unwrap_or_else(|| format!("{}    ", self.indent(body.start_line)))
    }

    /// Insert `statement` as a new member after the anchor, on its own line when the anchor ends
    /// its line and inline otherwise.
    fn insert_member_after(
        &self,
        line: u32,
        characters: u32,
        indent: &str,
        statement: &str,
    ) -> TextEditDto {
        if self.rest_is_blank(line, characters) && (line as usize) + 1 < self.text.lines().count() {
            self.insert(
                TextPosition::new(line + 1, 0),
                format!("{indent}{statement}\n"),
            )
        } else {
            self.insert(self.position(line, characters), format!(" {statement}"))
        }
    }

    fn import_fix(
        &self,
        parsed: &ParsedSource,
        namespace: Option<&RepairNamespace>,
        candidate: &ImportCandidate,
        preferred: bool,
    ) -> Option<TextEditSuggestion> {
        let path = self.model.element_qualified_name(candidate.target)?;
        let statement = format!("private import {path};");
        let edit = match namespace {
            Some(namespace) => {
                let package = package(parsed, namespace)?;
                let body = package.body_range?;
                let mut nested = Vec::new();
                for child in &package.children {
                    collect_packages(std::slice::from_ref(child), &mut nested);
                }
                let last_import = parsed
                    .imports()
                    .into_iter()
                    .filter(|import| {
                        contains(&body, &import.range)
                            && !nested.iter().any(|inner| {
                                inner
                                    .body_range
                                    .is_some_and(|inner| contains(&inner, &import.range))
                            })
                    })
                    .map(|import| import.range)
                    .max_by_key(|range| (range.end_line, range.end_character));
                match last_import {
                    Some(range) => self.insert_member_after(
                        range.end_line,
                        range.end_character,
                        self.indent(range.start_line),
                        &statement,
                    ),
                    None => self.insert_member_after(
                        body.start_line,
                        body.start_character + 1,
                        &self.member_indent(&body),
                        &statement,
                    ),
                }
            }
            None => {
                let last_import = parsed
                    .imports()
                    .into_iter()
                    .filter(|import| import.owner_package.is_none())
                    .map(|import| import.range)
                    .max_by_key(|range| (range.end_line, range.end_character));
                match last_import {
                    Some(range) => self.insert_member_after(
                        range.end_line,
                        range.end_character,
                        self.indent(range.start_line),
                        &statement,
                    ),
                    None => self.insert(TextPosition::new(0, 0), format!("{statement}\n")),
                }
            }
        };
        Some(
            TextEditSuggestion::new(format!("Import `{path}`"), vec![edit])
                .with_preferred(preferred),
        )
    }

    /// Declare the missing definition as the last member of the namespace the reference is in.
    ///
    /// The reference is one unqualified name, so its authored text is the name to declare.
    fn create_definition_fix(
        &self,
        parsed: &ParsedSource,
        reference: TextRange,
        namespace: Option<RepairNamespace>,
        kind: ElementKind,
    ) -> Option<TextEditSuggestion> {
        let keyword = definition_keyword(kind)?;
        let name = self.slice(reference)?;
        let declaration = format!("{keyword} {name};");
        let edit = match namespace {
            Some(namespace) => {
                let body = package(parsed, &namespace)?.body_range?;
                let close = body.end_character.checked_sub(1)?;
                let offset =
                    crate::text::position_to_byte_offset(self.line(body.end_line), 0, close)?;
                let mut before_close = self.line(body.end_line)[..offset].chars();
                if body.end_line > body.start_line && before_close.all(char::is_whitespace) {
                    self.insert(
                        TextPosition::new(body.end_line, 0),
                        format!("{}{declaration}\n", self.member_indent(&body)),
                    )
                } else {
                    self.insert(
                        self.position(body.end_line, close),
                        format!("{declaration} "),
                    )
                }
            }
            None => {
                let line_count = self.text.lines().count() as u32;
                let end = match line_count.checked_sub(1) {
                    Some(last) if !self.text.ends_with('\n') => {
                        TextPosition::new(last, utf16_len(self.line(last)))
                    }
                    _ => TextPosition::new(line_count, 0),
                };
                let separator = if self.text.ends_with('\n') || self.text.is_empty() {
                    ""
                } else {
                    "\n"
                };
                self.insert(end, format!("{separator}{declaration}\n"))
            }
        };
        Some(TextEditSuggestion::new(
            format!("Create `{keyword} {name}`"),
            vec![edit],
        ))
    }

    /// The authored text of a single-line range.
    fn slice(&self, range: TextRange) -> Option<&str> {
        if range.start.line != range.end.line {
            return None;
        }
        let line = self.line(range.start.line);
        let offset = |column: u32| crate::text::position_to_byte_offset(line, 0, column);
        let text = line.get(offset(range.start.character)?..offset(range.end.character)?)?;
        (!text.trim().is_empty()).then_some(text)
    }
}

/// The package or library package the repair named, matched by the line of its identifier.
///
/// Two packages declared on one line leave the match ambiguous, and no edit is placed.
fn package(parsed: &ParsedSource, namespace: &RepairNamespace) -> Option<SyntaxOutlineNode> {
    let mut found = Vec::new();
    collect_packages(&parsed.outline(), &mut found);
    found.retain(|node| node.selection_range.start_line == namespace.identifier.start.line);
    match <[_; 1]>::try_from(found) {
        Ok([package]) => Some(package),
        Err(_) => None,
    }
}

fn collect_packages(nodes: &[SyntaxOutlineNode], out: &mut Vec<SyntaxOutlineNode>) {
    for node in nodes {
        if matches!(
            node.kind,
            SyntaxOutlineKind::Package | SyntaxOutlineKind::LibraryPackage
        ) {
            out.push(node.clone());
        }
        collect_packages(&node.children, out);
    }
}

fn contains(outer: &SyntaxRange, inner: &SyntaxRange) -> bool {
    (outer.start_line, outer.start_character) <= (inner.start_line, inner.start_character)
        && (inner.end_line, inner.end_character) <= (outer.end_line, outer.end_character)
}

/// The keyword that declares a definition of `kind`, for the kinds a typing or specialization
/// slot can require.
fn definition_keyword(kind: ElementKind) -> Option<&'static str> {
    Some(match kind {
        ElementKind::PartDefinition => "part def",
        ElementKind::AttributeDefinition => "attribute def",
        ElementKind::EnumerationDefinition => "enum def",
        ElementKind::ItemDefinition => "item def",
        ElementKind::OccurrenceDefinition => "occurrence def",
        ElementKind::ActionDefinition => "action def",
        ElementKind::StateDefinition => "state def",
        ElementKind::PortDefinition => "port def",
        ElementKind::ConnectionDefinition => "connection def",
        ElementKind::InterfaceDefinition => "interface def",
        ElementKind::AllocationDefinition => "allocation def",
        ElementKind::FlowConnectionDefinition => "flow def",
        ElementKind::CalculationDefinition => "calc def",
        ElementKind::ConstraintDefinition => "constraint def",
        ElementKind::RequirementDefinition => "requirement def",
        ElementKind::ConcernDefinition => "concern def",
        ElementKind::CaseDefinition => "case def",
        ElementKind::AnalysisCaseDefinition => "analysis def",
        ElementKind::VerificationCaseDefinition => "verification def",
        ElementKind::UseCaseDefinition => "use case def",
        ElementKind::ViewDefinition => "view def",
        ElementKind::ViewpointDefinition => "viewpoint def",
        ElementKind::RenderingDefinition => "rendering def",
        ElementKind::MetadataDefinition => "metadata def",
        _ => return None,
    })
}
