//! Repository-wide test skip contracts.
//!
//! These checks use parsers for repository-owned syntax rather than free-text
//! searches, so comments and string literals do not affect the result.

use std::fs;
use std::path::{Path, PathBuf};

use syn::spanned::Spanned;
use syn::visit::Visit;
use syn::{Attribute, Expr, ExprLit, Lit, Meta};

const FIXTURES: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/tests/fixtures/sysml_compatibility"
);

#[test]
fn every_rust_ignore_attribute_has_a_non_empty_reason() {
    let root = repository_root();
    let mut violations = Vec::new();
    visit_rust_files(&root, &mut |path| {
        let source = fs::read_to_string(path).expect("read Rust source");
        let parsed = syn::parse_file(&source)
            .unwrap_or_else(|error| panic!("{}: Rust parse error: {error}", path.display()));
        let mut visitor = IgnoreAttributeVisitor::default();
        visitor.visit_file(&parsed);
        for violation in visitor.violations {
            violations.push(format!(
                "{}:{}: {}",
                path.display(),
                violation.line,
                violation.message
            ));
        }
    });

    assert!(
        violations.is_empty(),
        "every ignored Rust test must use #[ignore = \"concrete reason\"]:\n{}",
        violations.join("\n")
    );
}

#[test]
fn checked_in_semantic_graph_skip_metadata_has_a_complete_contract() {
    let mut violations = Vec::new();
    visit_markdown(Path::new(FIXTURES), &mut |path| {
        let Ok(fixture) = fs::read_to_string(path) else {
            // The corpus runner owns the explicit non-UTF-8 fixture skip.
            return;
        };
        if let Err(message) = validate_semantic_graph_skip_metadata(&fixture) {
            violations.push(format!("{}: {message}", path.display()));
        }
    });

    assert!(
        violations.is_empty(),
        "semantic graph skips require `semantic_graph=skip` and a concrete `semantic_graph_skip_reason` in META:\n{}",
        violations.join("\n")
    );
}

#[test]
fn ignore_attribute_parser_rejects_bare_and_empty_reasons() {
    let source = r####"
        #[ignore]
        fn bare() {}
        #[ignore = ""]
        fn empty() {}
        #[ignore = "   "]
        fn whitespace() {}
        #[ignore = "requires an optional external fixture"]
        fn explained() {}
        // #[ignore]
        const EXAMPLE: &str = r###"#[ignore]"###;
        const LIFETIME: &'static str = "not an attribute";
    "####;

    let parsed = syn::parse_file(source).expect("test Rust source");
    let mut visitor = IgnoreAttributeVisitor::default();
    visitor.visit_file(&parsed);
    let violations = visitor.violations;
    assert_eq!(violations.len(), 3, "{violations:#?}");
    assert!(violations[0].message.contains("bare"));
    assert!(violations[1].message.contains("empty"));
    assert!(violations[2].message.contains("empty"));
}

#[test]
fn semantic_graph_skip_metadata_rejects_missing_or_stale_reasons() {
    assert!(validate_semantic_graph_skip_metadata(
        "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph_skip_reason=parser recovery has no typed facts\n~~~\n"
    )
    .is_ok());

    for invalid in [
        "# META\n~~~ini\nsemantic_graph=skip\n~~~\n",
        "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph_skip_reason=   \n~~~\n",
        "# META\n~~~ini\nsemantic_graph_skip_reason=known parser gap\n~~~\n",
        "# META\n~~~ini\nsemantic_graph=assert\nsemantic_graph_skip_reason=known parser gap\n~~~\n",
        "# META\n~~~ini\nsemantic_graph=unsupported\n~~~\n",
    ] {
        assert!(
            validate_semantic_graph_skip_metadata(invalid).is_err(),
            "invalid skip metadata was accepted: {invalid}"
        );
    }
}

#[derive(Debug)]
struct IgnoreViolation {
    line: usize,
    message: &'static str,
}

#[derive(Default)]
struct IgnoreAttributeVisitor {
    violations: Vec<IgnoreViolation>,
}

impl<'ast> Visit<'ast> for IgnoreAttributeVisitor {
    fn visit_attribute(&mut self, attribute: &'ast Attribute) {
        if attribute.path().is_ident("ignore") {
            let message = match &attribute.meta {
                Meta::Path(_) => Some("bare #[ignore] attribute"),
                Meta::NameValue(value) => match &value.value {
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(reason),
                        ..
                    }) if !reason.value().trim().is_empty() => None,
                    Expr::Lit(ExprLit {
                        lit: Lit::Str(_), ..
                    }) => Some("empty #[ignore] reason"),
                    _ => Some("#[ignore] reason must be a non-empty string literal"),
                },
                Meta::List(_) => Some("#[ignore] reason must be a non-empty string literal"),
            };
            if let Some(message) = message {
                self.violations.push(IgnoreViolation {
                    line: attribute.span().start().line,
                    message,
                });
            }
        }
        syn::visit::visit_attribute(self, attribute);
    }
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace repository root")
        .to_path_buf()
}

fn visit_rust_files(root: &Path, visit: &mut dyn FnMut(&Path)) {
    for entry in fs::read_dir(root)
        .expect("read repository directory")
        .flatten()
    {
        let path = entry.path();
        let name = entry.file_name();
        if path.is_dir() {
            if !matches!(name.to_str(), Some(".git" | "target" | "node_modules")) {
                visit_rust_files(&path, visit);
            }
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            visit(&path);
        }
    }
}

fn visit_markdown(root: &Path, visit: &mut dyn FnMut(&Path)) {
    for entry in fs::read_dir(root)
        .expect("read fixture directory")
        .flatten()
    {
        let path = entry.path();
        if path.is_dir() {
            visit_markdown(&path, visit);
        } else if path.extension().is_some_and(|extension| extension == "md") {
            visit(&path);
        }
    }
}

fn validate_semantic_graph_skip_metadata(fixture: &str) -> Result<(), String> {
    let metadata = fenced_section(fixture, "META").ok_or("fixture is missing a META section")?;
    let mut graph_statuses = Vec::new();
    let mut reasons = Vec::new();
    for line in metadata
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let Some((key, value)) = line.split_once('=') else {
            if matches!(line, "semantic_graph" | "semantic_graph_skip_reason") {
                return Err(format!("META field {line:?} requires `=` and a value"));
            }
            continue;
        };
        match key.trim() {
            "semantic_graph" => graph_statuses.push(value.trim()),
            "semantic_graph_skip_reason" => reasons.push(value.trim()),
            _ => {}
        }
    }
    if graph_statuses.len() > 1 {
        return Err("META declares semantic_graph more than once".to_string());
    }
    if reasons.len() > 1 {
        return Err("META declares semantic_graph_skip_reason more than once".to_string());
    }
    match (graph_statuses.as_slice(), reasons.as_slice()) {
        ([], []) => Ok(()),
        (["skip"], [reason]) if !reason.is_empty() => Ok(()),
        (["skip"], []) => {
            Err("semantic_graph=skip requires semantic_graph_skip_reason".to_string())
        }
        (["skip"], [_]) => Err("semantic_graph_skip_reason must be non-empty".to_string()),
        ([], [_]) => Err("semantic_graph_skip_reason requires semantic_graph=skip".to_string()),
        ([status], _) => Err(format!(
            "semantic_graph must be skip when declared, got {status:?}"
        )),
        _ => unreachable!("duplicate META fields were rejected above"),
    }
}

fn fenced_section<'a>(fixture: &'a str, name: &str) -> Option<&'a str> {
    let marker = format!("# {name}\n");
    let start = fixture.find(&marker)? + marker.len();
    let section = &fixture[start..];
    let section = &section[..section.find("\n# ").unwrap_or(section.len())];
    let opening = section.find("~~~")?;
    let (_, contents) = section[opening + 3..].split_once('\n')?;
    let end = contents.find("\n~~~")?;
    Some(&contents[..end])
}
