//! `spec42 diagram`: list a model's diagram views, or render one natively -- the view's typed
//! projection normalized by `diagram_product` and drawn by `diagram_draw`, the same path the LSP
//! serves to the editor.

use std::fs;
use std::process::ExitCode;
use std::sync::Arc;

use generator_api::{
    DiagramSemanticReference, DiagramViewSummary, GeneratorModelView, QueryLimits,
};
use serde::Serialize;

use crate::cli::{Cli, DiagramArgs, DiagramFormat};
use crate::headless_renderer::render_native_svg;
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
            let summary = select_view(&views, view)?;
            let projection = model
                .diagram_view(&summary.handle)
                .map_err(|error| format!("`{view}` could not be projected: {error}"))?;
            let product = diagram_product::diagram_product_json(&projection)?;
            let product = String::from_utf8(product).expect("serde_json writes UTF-8");
            match args.format {
                DiagramFormat::Json => product,
                DiagramFormat::Svg => render_native_svg(&product)?,
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

fn select_view<'a>(
    views: &'a [DiagramViewSummary],
    requested: &str,
) -> Result<&'a DiagramViewSummary, String> {
    let matches: Vec<&DiagramViewSummary> = views
        .iter()
        .filter(|view| qualified_name(view) == requested)
        .collect();
    match matches.as_slice() {
        [view] => Ok(view),
        [] => {
            let available: Vec<String> = views.iter().map(qualified_name).collect();
            Err(if available.is_empty() {
                format!("`{requested}` is not a diagram view: the model has no diagram views")
            } else {
                format!(
                    "`{requested}` is not a diagram view; available views: {}",
                    available.join(", ")
                )
            })
        }
        _ => Err(format!(
            "`{requested}` names {} diagram views in different documents",
            matches.len()
        )),
    }
}
