//! Typed hover presentation shared by deterministic reports and editor Markdown.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverRelation {
    TypeOf,
    Redefines,
    Subsets,
    Specializes,
    Imports,
    Aliases,
}

impl HoverRelation {
    pub const fn label(self) -> &'static str {
        match self {
            Self::TypeOf => "Type of",
            Self::Redefines => "Redefines",
            Self::Subsets => "Subsets",
            Self::Specializes => "Specializes",
            Self::Imports => "Imports",
            Self::Aliases => "Aliases",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverResolutionState {
    Unresolved,
    Ambiguous,
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverUnitOutcome {
    Resolved {
        unit: String,
        dimensions: Vec<String>,
    },
    UnknownSymbol,
    Ambiguous(Vec<String>),
    UnsupportedExpression,
    CatalogUnavailable,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HoverBlock {
    Context {
        relation: HoverRelation,
        subject: Option<String>,
    },
    Identity {
        kind: String,
        role: Option<String>,
        name: String,
        direct_types: Vec<String>,
    },
    QualifiedName(String),
    Owner(String),
    InheritedType {
        type_name: String,
        inherited_from: String,
    },
    TypeResolution(String),
    Documentation(String),
    Source {
        identity: String,
        line: u32,
    },
    Keyword {
        keyword: String,
        description: String,
        syntax: Option<String>,
    },
    UnitLiteral {
        authored: String,
        outcome: HoverUnitOutcome,
    },
    Resolution {
        state: HoverResolutionState,
        subject: String,
        token: String,
        explanation: Option<String>,
    },
    Candidates(Vec<String>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HoverReport {
    pub blocks: Vec<HoverBlock>,
}

pub fn render_hover_markdown(report: &HoverReport) -> String {
    report
        .blocks
        .iter()
        .map(markdown_block)
        .collect::<Vec<_>>()
        .join("\n\n")
}

pub fn render_hover_sexpr(report: &HoverReport) -> String {
    let body = report
        .blocks
        .iter()
        .map(sexpr_block)
        .map(|block| format!("\n  {block}"))
        .collect::<String>();
    format!("(hover{body}\n)")
}

fn markdown_block(block: &HoverBlock) -> String {
    match block {
        HoverBlock::Context { relation, subject } => subject.as_ref().map_or_else(
            || format!("**{}**", relation.label()),
            |subject| format!("**{}** `{subject}`", relation.label()),
        ),
        HoverBlock::Identity {
            kind,
            role,
            name,
            direct_types,
        } => {
            let types = direct_types
                .iter()
                .map(|name| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ");
            format!(
                "`{}` **{}**{}",
                role.as_deref().unwrap_or(kind),
                escape_markdown(name),
                if types.is_empty() {
                    String::new()
                } else {
                    format!(": {types}")
                }
            )
        }
        HoverBlock::QualifiedName(value) => format!("`{value}`"),
        HoverBlock::Owner(value) => format!("In `{value}`"),
        HoverBlock::InheritedType {
            type_name,
            inherited_from,
        } => format!("Inherited type `{type_name}` from `{inherited_from}`"),
        HoverBlock::TypeResolution(value) => format!("Type resolution: **{value}**"),
        HoverBlock::Documentation(value) => escape_markdown(value),
        HoverBlock::Source { identity, line } => format!(
            "Defined in `{}:{line}`",
            crate::source_display::source_identity_label(identity)
        ),
        HoverBlock::Keyword {
            keyword,
            description,
            syntax,
        } => format!(
            "**{}**\n\n{}{}\n\n*See SysML v2 specification for full syntax.*",
            escape_markdown(keyword),
            description,
            syntax
                .as_ref()
                .map_or_else(String::new, |syntax| format!("\n\nSyntax: {syntax}"))
        ),
        HoverBlock::UnitLiteral { authored, outcome } => unit_markdown(authored, outcome),
        HoverBlock::Resolution {
            state,
            subject,
            token,
            explanation,
        } => {
            let state = match state {
                HoverResolutionState::Unresolved => "Unresolved",
                HoverResolutionState::Ambiguous => "Ambiguous",
                HoverResolutionState::Unsupported => "Unsupported",
                HoverResolutionState::Recovery => "Recovered",
                HoverResolutionState::Incomplete => "Incomplete",
            };
            format!(
                "**{state} {subject}** `{}`{}",
                escape_markdown(token),
                explanation
                    .as_ref()
                    .map_or_else(String::new, |text| format!("\n\n{text}"))
            )
        }
        HoverBlock::Candidates(values) => format!(
            "Candidates:{}",
            values
                .iter()
                .map(|value| format!("\n- `{value}`"))
                .collect::<String>()
        ),
    }
}

fn sexpr_block(block: &HoverBlock) -> String {
    match block {
        HoverBlock::Context { relation, subject } => format!(
            "(context (relation {}){})",
            atom(relation.label()),
            field("subject", subject.as_deref())
        ),
        HoverBlock::Identity {
            kind,
            role,
            name,
            direct_types,
        } => format!(
            "(identity (kind {}){} (name {}) (direct-types{}))",
            atom(kind),
            field("role", role.as_deref()),
            atom(name),
            atoms(direct_types)
        ),
        HoverBlock::QualifiedName(value) => format!("(qualified-name {})", atom(value)),
        HoverBlock::Owner(value) => format!("(owner {})", atom(value)),
        HoverBlock::InheritedType {
            type_name,
            inherited_from,
        } => format!(
            "(inherited-type (type {}) (from {}))",
            atom(type_name),
            atom(inherited_from)
        ),
        HoverBlock::TypeResolution(value) => format!("(type-resolution {})", atom(value)),
        HoverBlock::Documentation(value) => format!("(documentation {})", atom(value)),
        HoverBlock::Source { identity, line } => {
            format!("(source (identity {}) (line {line}))", atom(identity))
        }
        HoverBlock::Keyword {
            keyword,
            description,
            syntax,
        } => format!(
            "(keyword (name {}) (description {}){})",
            atom(keyword),
            atom(description),
            field("syntax", syntax.as_deref())
        ),
        HoverBlock::UnitLiteral { authored, outcome } => unit_sexpr(authored, outcome),
        HoverBlock::Resolution {
            state,
            subject,
            token,
            explanation,
        } => format!(
            "(resolution (state {}) (subject {}) (token {}){})",
            atom(match state {
                HoverResolutionState::Unresolved => "unresolved",
                HoverResolutionState::Ambiguous => "ambiguous",
                HoverResolutionState::Unsupported => "unsupported",
                HoverResolutionState::Recovery => "recovery",
                HoverResolutionState::Incomplete => "incomplete",
            }),
            atom(subject),
            atom(token),
            field("explanation", explanation.as_deref())
        ),
        HoverBlock::Candidates(values) => format!("(candidates{})", atoms(values)),
    }
}

fn unit_markdown(authored: &str, outcome: &HoverUnitOutcome) -> String {
    let mut lines = vec![format!("**Unit literal** `[{authored}]`"), String::new()];
    match outcome {
        HoverUnitOutcome::Resolved { unit, dimensions } => {
            lines.push(format!("*{unit}*"));
            lines.extend(dimensions.iter().map(|dimension| format!("Measured in `{dimension}`")));
        }
        HoverUnitOutcome::UnknownSymbol => lines.push("No unit with this symbol is declared in the admitted measurement catalog.".into()),
        HoverUnitOutcome::Ambiguous(candidates) => {
            lines.push("Several admitted units carry this symbol:".into());
            lines.extend(candidates.iter().map(|candidate| format!("- `{candidate}`")));
        }
        HoverUnitOutcome::UnsupportedExpression => lines.push("This is a unit expression rather than a single unit symbol, which Spec42 does not decompose.".into()),
        HoverUnitOutcome::CatalogUnavailable => lines.push("No measurement catalog is admitted to this workspace, so unit symbols cannot be resolved.".into()),
    }
    lines.join("\n")
}

fn unit_sexpr(authored: &str, outcome: &HoverUnitOutcome) -> String {
    let outcome = match outcome {
        HoverUnitOutcome::Resolved { unit, dimensions } => format!(
            "(resolved (unit {}) (dimensions{}))",
            atom(unit),
            atoms(dimensions)
        ),
        HoverUnitOutcome::UnknownSymbol => "(unknown-symbol)".into(),
        HoverUnitOutcome::Ambiguous(candidates) => format!("(ambiguous{})", atoms(candidates)),
        HoverUnitOutcome::UnsupportedExpression => "(unsupported-expression)".into(),
        HoverUnitOutcome::CatalogUnavailable => "(catalog-unavailable)".into(),
    };
    format!(
        "(unit-literal (authored {}) (outcome {outcome}))",
        atom(authored)
    )
}

fn atoms(values: &[String]) -> String {
    values
        .iter()
        .map(|value| format!(" {}", atom(value)))
        .collect()
}
fn field(name: &str, value: Option<&str>) -> String {
    value.map_or_else(String::new, |value| format!(" ({name} {})", atom(value)))
}
fn atom(value: &str) -> String {
    format!(
        "\"{}\"",
        value
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n")
    )
}
fn escape_markdown(value: &str) -> String {
    value
        .chars()
        .flat_map(|character| match character {
            '\\' | '`' | '*' | '_' | '{' | '}' | '[' | ']' | '(' | ')' | '<' | '>' | '#' | '+'
            | '-' | '.' | '!' | '|' => vec!['\\', character],
            _ => vec![character],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn both_writers_consume_the_same_typed_report() {
        let report = HoverReport {
            blocks: vec![HoverBlock::Context {
                relation: HoverRelation::TypeOf,
                subject: Some("P::car".into()),
            }],
        };
        assert!(render_hover_markdown(&report).contains("**Type of**"));
        assert!(render_hover_sexpr(&report).contains("(relation \"Type of\")"));
    }

    #[test]
    fn source_identity_is_lossless_in_report_and_line_aware_in_markdown() {
        let identity = "file:///a/very/long/workspace/path/to/model.sysml";
        let report = HoverReport {
            blocks: vec![HoverBlock::Source {
                identity: identity.into(),
                line: 42,
            }],
        };
        assert!(render_hover_sexpr(&report).contains(identity));
        assert!(render_hover_markdown(&report).contains("model.sysml:42"));
    }

    #[test]
    fn candidate_markdown_is_a_compact_list() {
        let report = HoverReport {
            blocks: vec![HoverBlock::Candidates(vec!["A::T".into(), "B::T".into()])],
        };
        assert_eq!(
            render_hover_markdown(&report),
            "Candidates:\n- `A::T`\n- `B::T`"
        );
    }
}
