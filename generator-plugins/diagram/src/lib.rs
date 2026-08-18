use serde::Serialize;
use serde_json::{json, Value};
use spec42_generator_sdk::{export, model, Artifact, Guest};

const SCHEMA_VERSION: u32 = 1;
const ARTIFACT_PATH: &str = "diagram.json";

#[derive(Clone, Copy)]
struct ViewDefinition {
    id: &'static str,
    title: &'static str,
    query: &'static str,
}

const VIEWS: [ViewDefinition; 8] = [
    ViewDefinition {
        id: "general-view",
        title: "General View",
        query: "general_view",
    },
    ViewDefinition {
        id: "interconnection-view",
        title: "Interconnection View",
        query: "interconnection_view",
    },
    ViewDefinition {
        id: "action-flow-view",
        title: "Action Flow View",
        query: "action_flow_view",
    },
    ViewDefinition {
        id: "state-transition-view",
        title: "State Transition View",
        query: "state_transition_view",
    },
    ViewDefinition {
        id: "sequence-view",
        title: "Sequence View",
        query: "sequence_view",
    },
    ViewDefinition {
        id: "browser-view",
        title: "Browser View",
        query: "browser_view",
    },
    ViewDefinition {
        id: "grid-view",
        title: "Grid View",
        query: "grid_view",
    },
    ViewDefinition {
        id: "geometry-view",
        title: "Geometry View",
        query: "geometry_view",
    },
];

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagramProduct {
    schema_version: u32,
    model_digest: String,
    view: ViewIdentity,
    completeness: Completeness,
    prepared_view: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ViewIdentity {
    id: &'static str,
    name: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Completeness {
    status: &'static str,
    reasons: Vec<IncompleteReason>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IncompleteReason {
    code: &'static str,
    message: String,
    required_query: &'static str,
}

struct DiagramGenerator;

impl Guest for DiagramGenerator {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let requested = args.first().map(String::as_str).unwrap_or("general-view");
        let view = VIEWS
            .iter()
            .find(|candidate| candidate.id == requested)
            .ok_or_else(|| format!("unsupported diagram view `{requested}`"))?;
        let info = model::info()?;
        let product = if view.id == "state-transition-view" {
            state_transition_product(*view, &info.model_digest, args.get(1))?
        } else {
            incomplete_product(*view, info.model_digest)
        };
        let contents = serde_json::to_vec_pretty(&product)
            .map_err(|error| format!("could not serialize diagram product: {error}"))?;
        Ok(vec![Artifact {
            file_path: ARTIFACT_PATH.to_owned(),
            contents,
        }])
    }
}

fn incomplete_product(view: ViewDefinition, model_digest: String) -> DiagramProduct {
    let message = format!(
        "{} is available in the diagram product, but its typed `{}` query is not implemented yet.",
        view.title, view.query
    );
    DiagramProduct {
        schema_version: SCHEMA_VERSION,
        model_digest,
        view: ViewIdentity {
            id: view.id,
            name: view.title.to_owned(),
        },
        completeness: Completeness {
            status: "incomplete",
            reasons: vec![IncompleteReason {
                code: "diagram.query.unsupported",
                message: message.clone(),
                required_query: view.query,
            }],
        },
        prepared_view: json!({
            "title": view.title,
            "view": view.id,
            "nodes": [],
            "edges": [],
            "meta": { "emptyStateMessage": message }
        }),
    }
}

fn state_transition_product(
    view: ViewDefinition,
    model_digest: &str,
    handle: Option<&String>,
) -> Result<DiagramProduct, String> {
    let handle =
        handle.ok_or_else(|| "state-transition-view requires a typed catalog handle".to_owned())?;
    let projection = model::state_transition_view(handle)?;
    if projection.model_digest != model_digest {
        return Err("state-transition projection does not belong to the active model".to_owned());
    }
    let nodes = projection
        .nodes
        .iter()
        .map(|node| {
            let kind = match node.kind {
                model::StateTransitionNodeKind::Initial => "initial",
                model::StateTransitionNodeKind::State => "state",
                model::StateTransitionNodeKind::Final => "final",
            };
            json!({
                "id": node.semantic_id,
                "label": node.label,
                "kind": kind,
                "uri": node.source.uri,
                "range": source_range(&node.source.range),
                "attributes": { "semanticId": node.semantic_id }
            })
        })
        .collect::<Vec<_>>();
    let edges = projection
        .transitions
        .iter()
        .map(|edge| {
            json!({
                "id": edge.semantic_id,
                "source": edge.source,
                "target": edge.target,
                "label": edge.label.clone().unwrap_or_default(),
                "edgeKind": "transition",
                "attributes": {
                    "relationType": "transition",
                    "selfLoop": edge.source == edge.target,
                    "semanticId": edge.semantic_id,
                    "provenance": edge.provenance,
                    "trigger": edge.trigger,
                    "guard": edge.guard,
                    "effect": edge.effect,
                    "sourceReference": edge.source_reference
                }
            })
        })
        .collect::<Vec<_>>();
    let (status, reasons) = match projection.completeness {
        model::ProjectionCompleteness::Complete => ("complete", Vec::new()),
        model::ProjectionCompleteness::Incomplete { reasons } => (
            "incomplete",
            reasons
                .into_iter()
                .map(|reason| IncompleteReason {
                    code: "diagram.projection.incomplete",
                    message: format!("{}: {}", reason.code, reason.message),
                    required_query: view.query,
                })
                .collect(),
        ),
    };
    Ok(DiagramProduct {
        schema_version: SCHEMA_VERSION,
        model_digest: model_digest.to_owned(),
        view: ViewIdentity {
            id: view.id,
            name: projection.view.name.clone(),
        },
        completeness: Completeness { status, reasons },
        prepared_view: json!({
            "title": projection.view.name,
            "view": view.id,
            "nodes": nodes,
            "edges": edges,
            "meta": {
                "layoutDirection": "horizontal",
                "selectedDiagramId": projection.view.semantic_id,
                "parentContext": projection.machine.label
            }
        }),
    })
}

fn source_range(range: &model::SourceRange) -> Value {
    json!({
        "start": { "line": range.start_line, "character": range.start_character },
        "end": { "line": range.end_line, "character": range.end_character }
    })
}

export!(DiagramGenerator);
