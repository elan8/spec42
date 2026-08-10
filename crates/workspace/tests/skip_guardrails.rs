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

const GENERIC_SEMANTIC_GRAPH_SKIP_REASON: &str =
    "strictly parsed non-empty source produced no typed semantic graph facts";

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
fn checked_in_fixture_skip_metadata_has_a_complete_contract() {
    let mut violations = Vec::new();
    visit_markdown(Path::new(FIXTURES), &mut |path| {
        let bytes = fs::read(path).expect("read fixture");
        // `fenced_section`'s markers are LF-only; normalize so a Windows checkout
        // (CRLF line endings) doesn't make every fixture look like it has no META section.
        let fixture = String::from_utf8_lossy(&bytes).replace("\r\n", "\n");
        if let Err(message) = validate_fixture_skip_metadata(&fixture) {
            violations.push(format!("{}: {message}", path.display()));
        }
    });

    assert!(
        violations.is_empty(),
        "fixture skips require `<contract>=skip` and a concrete `<contract>_skip_reason` in META:\n{}",
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
        "# META\n~~~ini\nsemantic_graph=skip\nsemantic_graph_skip_reason=strictly parsed non-empty source produced no typed semantic graph facts\n~~~\n",
    ] {
        assert!(
            validate_semantic_graph_skip_metadata(invalid).is_err(),
            "invalid skip metadata was accepted: {invalid}"
        );
    }
}

#[test]
fn formatter_skip_metadata_rejects_missing_or_stale_reasons() {
    assert!(validate_fixture_skip_metadata(
        "# META\n~~~ini\nformatter=skip\nformatter_skip_reason=formatter recovery cannot preserve this malformed source\n~~~\n"
    )
    .is_ok());

    for invalid in [
        "# META\n~~~ini\nformatter=skip\n~~~\n",
        "# META\n~~~ini\nformatter=skip\nformatter_skip_reason=   \n~~~\n",
        "# META\n~~~ini\nformatter_skip_reason=known formatter bug\n~~~\n",
        "# META\n~~~ini\nformatter=assert\nformatter_skip_reason=known formatter bug\n~~~\n",
        "# META\n~~~ini\nformatter=skip\nformatter=skip\nformatter_skip_reason=known formatter bug\n~~~\n",
    ] {
        assert!(
            validate_fixture_skip_metadata(invalid).is_err(),
            "invalid formatter skip metadata was accepted: {invalid}"
        );
    }
}

#[test]
fn diagnostics_skip_metadata_rejects_missing_or_orphaned_reasons() {
    assert!(validate_fixture_skip_metadata(
        "# META\n~~~ini\ndiagnostics=skip\ndiagnostics_skip_reason=missing optional diagnostic capability\n~~~\n"
    )
    .is_ok());

    for invalid in [
        "# META\n~~~ini\ndiagnostics=skip\n~~~\n",
        "# META\n~~~ini\ndiagnostics=skip\ndiagnostics_skip_reason=   \n~~~\n",
        "# META\n~~~ini\ndiagnostics_skip_reason=known diagnostic gap\n~~~\n",
        "# META\n~~~ini\ndiagnostics=assert\ndiagnostics_skip_reason=known diagnostic gap\n~~~\n",
        "# META\n~~~ini\ndiagnostics=skip\ndiagnostics=skip\ndiagnostics_skip_reason=known diagnostic gap\n~~~\n",
    ] {
        assert!(
            validate_fixture_skip_metadata(invalid).is_err(),
            "invalid diagnostics skip metadata was accepted: {invalid}"
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

fn validate_fixture_skip_metadata(fixture: &str) -> Result<(), String> {
    let metadata = fenced_section(fixture, "META").ok_or("fixture is missing a META section")?;
    validate_skip_metadata(metadata, "semantic_graph", "semantic_graph_skip_reason")?;
    validate_skip_metadata(metadata, "formatter", "formatter_skip_reason")?;
    validate_skip_metadata(metadata, "diagnostics", "diagnostics_skip_reason")
}

fn validate_semantic_graph_skip_metadata(fixture: &str) -> Result<(), String> {
    let metadata = fenced_section(fixture, "META").ok_or("fixture is missing a META section")?;
    validate_skip_metadata(metadata, "semantic_graph", "semantic_graph_skip_reason")
}

fn validate_skip_metadata(metadata: &str, state_key: &str, reason_key: &str) -> Result<(), String> {
    let mut graph_statuses = Vec::new();
    let mut reasons = Vec::new();
    for line in metadata
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
    {
        let Some((key, value)) = line.split_once('=') else {
            if line == state_key || line == reason_key {
                return Err(format!("META field {line:?} requires `=` and a value"));
            }
            continue;
        };
        match key.trim() {
            key if key == state_key => graph_statuses.push(value.trim()),
            key if key == reason_key => reasons.push(value.trim()),
            _ => {}
        }
    }
    if graph_statuses.len() > 1 {
        return Err(format!("META declares {state_key} more than once"));
    }
    if reasons.len() > 1 {
        return Err(format!("META declares {reason_key} more than once"));
    }
    match (graph_statuses.as_slice(), reasons.as_slice()) {
        ([], []) => Ok(()),
        (["skip"], [reason]) if !reason.is_empty() => {
            if reason_key == "semantic_graph_skip_reason"
                && *reason == GENERIC_SEMANTIC_GRAPH_SKIP_REASON
            {
                Err(format!(
                    "{reason_key} must name the unavailable parser or semantic capability"
                ))
            } else {
                Ok(())
            }
        }
        (["skip"], []) => Err(format!("{state_key}=skip requires {reason_key}")),
        (["skip"], [_]) => Err(format!("{reason_key} must be non-empty")),
        ([], [_]) => Err(format!("{reason_key} requires {state_key}=skip")),
        ([status], _) => Err(format!(
            "{state_key} must be skip when declared, got {status:?}"
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
