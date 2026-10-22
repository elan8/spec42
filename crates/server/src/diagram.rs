//! `spec42 diagram`: list a model's diagram views, or render one natively -- the view's typed
//! projection normalized by `diagram_product` and drawn by `diagram_draw`, the same path the LSP
//! serves to the editor.

use std::collections::BTreeSet;
use std::fs;
use std::process::ExitCode;
use std::sync::Arc;

use generator_api::{
    DiagramSemanticReference, DiagramViewSummary, GeneratorModelView, QueryLimits,
};
use serde::Serialize;

use crate::cli::{Cli, DiagramArgs, DiagramFormat};
use crate::headless_renderer::render_nested_svg;
use crate::host_snapshot::load_snapshot_for_paths;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ViewListing<'a> {
    qualified_name: &'a str,
    kind: &'static str,
    name: &'a str,
    document: &'a str,
}

pub fn run_diagram(cli: &Cli, args: &DiagramArgs) -> Result<ExitCode, String> {
    let snapshot = load_snapshot_for_paths(cli, &args.path, args.workspace_root.as_deref(), false)?;
    let publication = snapshot.published_model_arc();
    let model = GeneratorModelView::new(
        Arc::clone(&publication),
        publication.publication().model_digest().to_string(),
        env!("CARGO_PKG_VERSION"),
        QueryLimits::default(),
    )
    .map_err(|error| format!("the model cannot be projected: {error}"))?;
    let mut views = model
        .diagram_views()
        .map_err(|error| format!("the diagram view catalog failed: {error}"))?;
    views.sort_by_key(qualified_name);

    let output = match &args.view {
        None => list_views(&views, args.format)?,
        Some(view) => {
            let summary = select_view(
                &views,
                &Selection {
                    qualified_name: view,
                    kind: args.kind.as_deref(),
                    document: args.document.as_deref(),
                },
            )?;
            let projection = if summary.kind == generator_api::DiagramViewKind::BrowserView {
                browser_projection(&model, &summary.handle, args.depth)
            } else {
                model.diagram_view(&summary.handle)
            }
            .map_err(|error| format!("`{view}` could not be projected: {error}"))?;
            let product = diagram_product::diagram_product_json(&projection)?;
            let product = String::from_utf8(product).expect("serde_json writes UTF-8");
            if args.stats {
                product_stats(&product)?
            } else {
                match args.format {
                    DiagramFormat::Json => product,
                    DiagramFormat::Svg => render_nested_svg(&product, args.depth)?,
                }
            }
        }
    };
    match &args.output {
        Some(path) => fs::write(path, output)
            .map_err(|error| format!("could not write {}: {error}", path.display()))?,
        None => print!("{output}"),
    }
    Ok(ExitCode::SUCCESS)
}

/// Where a diagram product's bytes go. Sizes are of compact JSON, the form the language server
/// sends; the pretty form is what `--format json` writes.
fn product_stats(product: &str) -> Result<String, String> {
    use std::collections::BTreeMap;
    use std::fmt::Write as _;

    let value: serde_json::Value =
        serde_json::from_str(product).map_err(|error| format!("invalid product: {error}"))?;
    let compact = |value: &serde_json::Value| serde_json::to_string(value).map_or(0, |s| s.len());
    let count = |value: &serde_json::Value| value.as_array().map_or(0, Vec::len);
    let sized = |object: &serde_json::Value| {
        let mut entries = object
            .as_object()
            .into_iter()
            .flatten()
            .map(|(key, value)| (key.clone(), compact(value), count(value)))
            .collect::<Vec<_>>();
        entries.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        entries
    };
    let mut out = String::new();
    let _ = writeln!(out, "schemaVersion: {}", value["schemaVersion"]);
    let _ = writeln!(out, "pretty bytes:  {}", product.len());
    let _ = writeln!(out, "compact bytes: {}", compact(&value));
    for (title, object) in [("sections", &value), ("projection", &value["projection"])] {
        let _ = writeln!(out, "\n{title}:");
        for (key, bytes, items) in sized(object) {
            let items = if items > 0 {
                format!("{items:>8} items")
            } else {
                String::new()
            };
            let _ = writeln!(out, "  {key:<16} {bytes:>12} bytes {items}");
        }
    }
    let mut tally = |title: &str, keys: Vec<String>| {
        let mut counts = BTreeMap::<String, usize>::new();
        for key in keys {
            *counts.entry(key).or_default() += 1;
        }
        let mut counts = counts.into_iter().collect::<Vec<_>>();
        counts.sort_by(|left, right| right.1.cmp(&left.1).then_with(|| left.0.cmp(&right.0)));
        let _ = writeln!(out, "\n{title}:");
        for (key, n) in counts {
            let _ = writeln!(out, "  {n:>8}  {key}");
        }
    };
    let field =
        |item: &serde_json::Value, name: &str| item[name].as_str().unwrap_or("-").to_owned();
    let items = |value: &serde_json::Value| value.as_array().cloned().unwrap_or_default();
    tally(
        "relationships by provenance and kind",
        items(&value["projection"]["relationships"])
            .iter()
            .map(|item| format!("{} {}", field(item, "provenance"), field(item, "kind")))
            .collect(),
    );
    tally(
        "nodes by metaclass",
        items(&value["projection"]["nodes"])
            .iter()
            .map(|item| field(item, "metaclass"))
            .collect(),
    );
    tally(
        "documents by source domain",
        items(&value["documents"])
            .iter()
            .map(|item| field(item, "sourceDomain"))
            .collect(),
    );
    Ok(out)
}

/// A browser projection loads its roots' members; `--depth` opens each further level by asking
/// for the members of every row that still has some to load, as an editor does on expand.
fn browser_projection(
    model: &GeneratorModelView,
    handle: &str,
    depth: diagram_draw::Nesting,
) -> Result<generator_api::DiagramViewProjection, generator_api::ModelQueryError> {
    // A recursive type has an unbounded instance tree; stop opening after this many levels.
    const DEEPEST: usize = 64;
    let levels = match depth {
        diagram_draw::Nesting::Levels(levels) => levels.saturating_sub(1),
        diagram_draw::Nesting::All => DEEPEST,
    };
    let mut expanded = Vec::<String>::new();
    let mut projection = model.diagram_view(handle)?;
    for _ in 0..levels.min(DEEPEST) {
        let generator_api::DiagramViewMetadata::Browser { rows, .. } = &projection.metadata else {
            break;
        };
        let unloaded = rows
            .iter()
            .filter(|row| row.members == generator_api::DiagramBrowserMembers::Unloaded)
            .map(|row| row.key.clone())
            .filter(|key| !expanded.contains(key))
            .collect::<Vec<_>>();
        if unloaded.is_empty() {
            break;
        }
        expanded.extend(unloaded);
        projection = model.diagram_view_expanding(handle, &expanded)?;
    }
    Ok(projection)
}

fn qualified_name(view: &DiagramViewSummary) -> String {
    match &view.reference {
        DiagramSemanticReference::Qualified { qualified_name, .. } => qualified_name.clone(),
        _ => view.name.clone(),
    }
}

fn list_views(views: &[DiagramViewSummary], format: DiagramFormat) -> Result<String, String> {
    let names: Vec<String> = views.iter().map(qualified_name).collect();
    match format {
        DiagramFormat::Json => {
            let listing: Vec<ViewListing> = views
                .iter()
                .zip(&names)
                .map(|(view, qualified_name)| ViewListing {
                    qualified_name,
                    kind: diagram_product::kind_id(view.kind),
                    name: &view.name,
                    document: &view.source.uri,
                })
                .collect();
            let mut json = serde_json::to_string_pretty(&listing)
                .map_err(|error| format!("could not serialize the view list: {error}"))?;
            json.push('\n');
            Ok(json)
        }
        DiagramFormat::Svg => Ok(views
            .iter()
            .zip(&names)
            .map(|(view, qualified_name)| {
                format!(
                    "{:<22} {qualified_name}\n",
                    diagram_product::kind_id(view.kind)
                )
            })
            .collect()),
    }
}

/// What `--view`, `--kind` and `--document` ask for.
struct Selection<'a> {
    qualified_name: &'a str,
    kind: Option<&'a str>,
    document: Option<&'a str>,
}

/// Whether `document` (as given on the command line) names the view's document `uri`: the listed
/// URI itself, or a path suffix of it on a `/` boundary.
fn document_matches(uri: &str, document: &str) -> bool {
    let document = document.replace('\\', "/");
    let document = document.trim_start_matches("./");
    uri == document || uri.ends_with(&format!("/{document}"))
}

fn select_view<'a>(
    views: &'a [DiagramViewSummary],
    selection: &Selection,
) -> Result<&'a DiagramViewSummary, String> {
    let requested = selection.qualified_name;
    let named: Vec<&DiagramViewSummary> = views
        .iter()
        .filter(|view| qualified_name(view) == requested)
        .collect();
    if named.is_empty() {
        let available: Vec<String> = views.iter().map(qualified_name).collect();
        return Err(if available.is_empty() {
            format!("`{requested}` is not a diagram view: the model has no diagram views")
        } else {
            format!(
                "`{requested}` is not a diagram view; available views: {}",
                available.join(", ")
            )
        });
    }
    let matches: Vec<&DiagramViewSummary> = named
        .iter()
        .copied()
        .filter(|view| {
            selection
                .kind
                .is_none_or(|kind| diagram_product::kind_id(view.kind) == kind)
                && selection
                    .document
                    .is_none_or(|document| document_matches(&view.source.uri, document))
        })
        .collect();
    let describe = |views: &[&DiagramViewSummary]| {
        views
            .iter()
            .map(|view| {
                format!(
                    "{} in {}",
                    diagram_product::kind_id(view.kind),
                    view.source.uri
                )
            })
            .collect::<Vec<_>>()
            .join("; ")
    };
    match matches.as_slice() {
        [view] => Ok(view),
        [] => Err(format!(
            "no `{requested}` view matches the given --kind/--document; `{requested}` is: {}",
            describe(&named)
        )),
        several => {
            let kinds: BTreeSet<_> = several
                .iter()
                .map(|view| diagram_product::kind_id(view.kind))
                .collect();
            let documents: BTreeSet<_> = several.iter().map(|view| &view.source.uri).collect();
            let hint = match (kinds.len() > 1, documents.len() > 1) {
                (true, true) => "--kind and --document",
                (true, false) => "--kind",
                _ => "--document",
            };
            Err(format!(
                "`{requested}` names {} diagram views ({}); select one with {hint}",
                several.len(),
                describe(several)
            ))
        }
    }
}
