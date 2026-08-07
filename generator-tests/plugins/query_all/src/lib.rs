//! Calls every query operation in a fixed order and writes a normalised transcript.
//!
//! This is the highest-value plugin in the corpus: its golden pins the observable shape of
//! the entire semantic API, so any change to ordering, metaclass strings, relationship kinds
//! or `ElementDetail` defaulting appears as a reviewable diff.

use spec42_generator_sdk::{export, model, Artifact, Guest};
use std::fmt::Write;

struct QueryAll;

/// `model_digest` and `spec42_version` change on every Spec42 release, so the transcript
/// records only whether they are present. Embedding them would make the golden churn.
fn describe_info(out: &mut String) {
    match model::info() {
        Ok(info) => {
            let _ = writeln!(
                out,
                "info: model_digest={} spec42_version={} semantic_api_version={}",
                present(&info.model_digest),
                present(&info.spec42_version),
                info.semantic_api_version
            );
        }
        Err(error) => {
            let _ = writeln!(out, "info: ERROR {error}");
        }
    }
}

fn present(value: &str) -> &'static str {
    if value.is_empty() {
        "<empty>"
    } else {
        "<present>"
    }
}

fn summarise(out: &mut String, label: &str, result: Result<Vec<model::ElementSummary>, String>) {
    match result {
        Ok(items) => {
            let _ = writeln!(out, "{label}: {} element(s)", items.len());
            for item in items {
                let _ = writeln!(
                    out,
                    "  {} [{}] name={} library={}",
                    item.qualified_name,
                    item.metaclass,
                    item.name.as_deref().unwrap_or("<none>"),
                    item.library_element
                );
            }
        }
        Err(error) => {
            let _ = writeln!(out, "{label}: ERROR {error}");
        }
    }
}

impl Guest for QueryAll {
    fn generate(_args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let mut out = String::new();
        describe_info(&mut out);

        let roots = model::roots()?;
        summarise(&mut out, "roots", Ok(roots.clone()));

        let all = model::find(None)?;
        summarise(&mut out, "find(None)", Ok(all.clone()));
        summarise(
            &mut out,
            "find(PartDefinition)",
            model::find(Some("PartDefinition")),
        );

        // Walk deterministically: `find(None)` is already ordered, so the transcript is
        // stable without sorting here.
        for element in &all {
            summarise(
                &mut out,
                &format!("children({})", element.qualified_name),
                model::children(&element.handle),
            );

            match model::element(&element.handle) {
                Ok(detail) => {
                    let _ = writeln!(
                        out,
                        "element({}): definition={} abstract={} derived={} end={} multiplicity={}",
                        element.qualified_name,
                        detail.definition,
                        detail.abstract_flag,
                        detail.derived,
                        detail.end,
                        detail
                            .multiplicity
                            .map(|value| format!(
                                "{}..{}",
                                value.lower.unwrap_or_else(|| "?".to_owned()),
                                value.upper.unwrap_or_else(|| "?".to_owned())
                            ))
                            .unwrap_or_else(|| "<none>".to_owned())
                    );
                }
                Err(error) => {
                    let _ = writeln!(out, "element({}): ERROR {error}", element.qualified_name);
                }
            }

            match model::typed_by(&element.handle) {
                Ok(Some(target)) => {
                    let _ = writeln!(
                        out,
                        "typed_by({}): {}",
                        element.qualified_name, target.qualified_name
                    );
                }
                Ok(None) => {
                    let _ = writeln!(out, "typed_by({}): <untyped>", element.qualified_name);
                }
                Err(error) => {
                    let _ = writeln!(out, "typed_by({}): ERROR {error}", element.qualified_name);
                }
            }

            match model::relationships(&element.handle) {
                Ok(items) => {
                    let _ = writeln!(
                        out,
                        "relationships({}): {} edge(s)",
                        element.qualified_name,
                        items.len()
                    );
                    for edge in items {
                        let _ = writeln!(
                            out,
                            "  {} {} -> {} implied={}",
                            edge.kind,
                            edge.source.qualified_name,
                            edge.target.qualified_name,
                            edge.implied
                        );
                    }
                }
                Err(error) => {
                    let _ = writeln!(
                        out,
                        "relationships({}): ERROR {error}",
                        element.qualified_name
                    );
                }
            }

            summarise(
                &mut out,
                &format!("effective_features({})", element.qualified_name),
                model::effective_features(&element.handle),
            );
        }

        // An unknown handle must be an error, not an empty result.
        let _ = writeln!(
            out,
            "element(<bogus>): {}",
            match model::element("h:not-a-real-handle") {
                Ok(_) => "UNEXPECTED OK".to_owned(),
                Err(_) => "error".to_owned(),
            }
        );

        Ok(vec![Artifact {
            file_path: "transcript.txt".to_owned(),
            contents: out.into_bytes(),
        }])
    }
}

export!(QueryAll);
