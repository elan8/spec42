use std::collections::BTreeMap;

use serde::Serialize;
use serde_json::{json, Value};
use spec42_generator_sdk::{export, model, Artifact, Guest};

const SCHEMA_VERSION: u32 = 5;
const ARTIFACT_PATH: &str = "diagram.json";

type DocumentIndex = usize;
type SourceIndex = usize;
type ReferenceIndex = usize;
type NodeIndex = usize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagramProduct {
    schema_version: u32,
    model_digest: String,
    documents: Vec<DocumentRecord>,
    sources: Vec<SourceRecord>,
    references: Vec<Value>,
    selected_view: SelectedView,
    completeness: Completeness,
    projection: Value,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DocumentRecord {
    uri: String,
    source_domain: &'static str,
}

#[derive(Serialize)]
struct SourceRecord {
    document: DocumentIndex,
    range: [u32; 4],
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SelectedView {
    reference: ReferenceIndex,
    kind: &'static str,
    name: String,
    source: SourceIndex,
}

#[derive(Serialize)]
struct Completeness {
    status: &'static str,
    reasons: Vec<Value>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum ReferenceKey {
    Qualified(u8, String, String),
    Tooling(u8, String),
    Anchor(u8, String, Option<String>, String, [u32; 4]),
    Relationship(u8, String, String, String, u32),
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct SourceKey(String, [u32; 4]);

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct OccurrenceKey(Vec<ReferenceKey>);

struct NormalizedProduct {
    documents: Vec<DocumentRecord>,
    document_indexes: BTreeMap<(String, u8), DocumentIndex>,
    sources: Vec<SourceRecord>,
    source_indexes: BTreeMap<SourceKey, SourceIndex>,
    references: Vec<model::DiagramSemanticReference>,
    reference_indexes: BTreeMap<ReferenceKey, ReferenceIndex>,
    node_indexes: BTreeMap<OccurrenceKey, NodeIndex>,
}

struct DiagramGenerator;

impl Guest for DiagramGenerator {
    fn generate(args: Vec<String>) -> Result<Vec<Artifact>, String> {
        let handle = args
            .first()
            .ok_or_else(|| "diagram generation requires a typed catalog handle".to_owned())?;
        let info = model::info()?;
        let typed = model::diagram_view(handle)?;
        if typed.model_digest != info.model_digest {
            return Err("diagram projection does not belong to the active model".to_owned());
        }
        let normalized = NormalizedProduct::new(&typed)?;
        let kind = kind_id(typed.view.kind);
        let reasons = typed
            .incomplete_reasons
            .iter()
            .map(|reason| normalized.incomplete_reason(reason))
            .collect::<Result<Vec<_>, _>>()?;
        let references = normalized
            .references
            .iter()
            .map(|reference| normalized.semantic_reference(reference))
            .collect::<Result<Vec<_>, _>>()?;
        let selected_view = SelectedView {
            reference: normalized.reference(&typed.view.reference)?,
            kind,
            name: typed.view.name.clone(),
            source: normalized.source(&typed.view.source)?,
        };
        let projection = normalized.projection(&typed)?;
        let product = DiagramProduct {
            schema_version: SCHEMA_VERSION,
            model_digest: info.model_digest,
            documents: normalized.documents,
            sources: normalized.sources,
            references,
            selected_view,
            completeness: Completeness {
                status: if reasons.is_empty() {
                    "complete"
                } else {
                    "incomplete"
                },
                reasons,
            },
            projection,
        };
        let mut contents = serde_json::to_vec_pretty(&product)
            .map_err(|error| format!("could not serialize diagram product: {error}"))?;
        contents.push(b'\n');
        Ok(vec![Artifact {
            file_path: ARTIFACT_PATH.to_owned(),
            contents,
        }])
    }
}

impl NormalizedProduct {
    fn new(typed: &model::DiagramViewProjection) -> Result<Self, String> {
        let mut references = BTreeMap::<ReferenceKey, model::DiagramSemanticReference>::new();
        let mut sources = BTreeMap::<SourceKey, model::SourceReference>::new();
        collect_reference(&mut references, &typed.view.reference);
        collect_source(&mut sources, &typed.view.source);
        for reference in &typed.exposed_roots {
            collect_occurrence_references(&mut references, reference);
        }
        for element in &typed.elements {
            collect_reference(&mut references, &element.reference);
            match &element.typing {
                model::DiagramElementTyping::Resolved(values)
                | model::DiagramElementTyping::Partial(values) => values
                    .iter()
                    .for_each(|value| collect_reference(&mut references, &value.reference)),
                model::DiagramElementTyping::Ambiguous(values) => values
                    .iter()
                    .for_each(|value| collect_reference(&mut references, value)),
                _ => {}
            }
            if let Some(owner) = &element.owner {
                collect_occurrence_references(&mut references, owner);
            }
            collect_source(&mut sources, &element.source);
            for compartment in &element.compartments {
                for member in &compartment.members {
                    collect_occurrence_references(&mut references, member);
                }
            }
        }
        for relationship in &typed.relationships {
            collect_reference(&mut references, &relationship.reference);
            collect_reference(&mut references, &relationship.source_element);
            match &relationship.target {
                model::DiagramRelationshipTarget::Resolved(value) => {
                    collect_reference(&mut references, &value.reference);
                    collect_endpoint_occurrences(&mut references, &value.occurrence)
                }
                model::DiagramRelationshipTarget::Ambiguous(values) => values
                    .iter()
                    .for_each(|value| {
                        collect_reference(&mut references, &value.reference);
                        collect_endpoint_occurrences(&mut references, &value.occurrence);
                    }),
                model::DiagramRelationshipTarget::Unresolved
                | model::DiagramRelationshipTarget::Unsupported => {}
            }
            if let Some(source) = &relationship.source {
                collect_source(&mut sources, source);
            }
        }
        for edge in &typed.edges {
            collect_reference(&mut references, &edge.reference);
            collect_reference(&mut references, &edge.source_element);
            collect_reference(&mut references, &edge.target_element);
            if let Some(source) = &edge.source {
                collect_source(&mut sources, source);
            }
        }
        collect_metadata_references(&mut references, &typed.metadata);
        collect_scene_sources(&mut sources, &typed.scene);
        for reason in &typed.incomplete_reasons {
            collect_reason_reference(&mut references, reason);
        }
        for reference in references.values() {
            if let model::DiagramSemanticReference::SourceAnchor {
                document,
                source_domain,
                range,
                ..
            } = reference
            {
                collect_source(
                    &mut sources,
                    &model::SourceReference {
                        uri: document.clone(),
                        range: range.clone(),
                    },
                );
                let _ = source_domain;
            }
        }

        let mut documents = BTreeMap::<(String, u8), model::DiagramSourceDomain>::new();
        for reference in references.values() {
            if let Some((uri, domain)) = reference_document(reference) {
                documents.insert((uri.to_owned(), domain_rank(domain)), domain);
            }
        }
        for source in sources.values() {
            let domain = references
                .values()
                .find_map(|reference| {
                    reference_document(reference)
                        .and_then(|(uri, domain)| (uri == source.uri).then_some(domain))
                })
                .unwrap_or(model::DiagramSourceDomain::Workspace);
            documents.insert((source.uri.clone(), domain_rank(domain)), domain);
        }
        let document_indexes: BTreeMap<(String, u8), DocumentIndex> = documents
            .keys()
            .cloned()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect();
        let document_records = documents
            .into_iter()
            .map(|((uri, _), domain)| DocumentRecord {
                uri,
                source_domain: source_domain(domain),
            })
            .collect::<Vec<_>>();

        let source_indexes = sources
            .keys()
            .cloned()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect::<BTreeMap<_, _>>();
        let source_records = sources
            .values()
            .map(|source| {
                let domain = document_records
                    .iter()
                    .find(|document| document.uri == source.uri)
                    .map(|document| source_domain_rank(document.source_domain))
                    .ok_or_else(|| format!("source document `{}` was not interned", source.uri))?;
                let document = *document_indexes
                    .get(&(source.uri.clone(), domain))
                    .ok_or_else(|| format!("source document `{}` has no index", source.uri))?;
                Ok(SourceRecord {
                    document,
                    range: range_array(&source.range),
                })
            })
            .collect::<Result<Vec<_>, String>>()?;
        let reference_indexes = references
            .keys()
            .cloned()
            .enumerate()
            .map(|(index, key)| (key, index))
            .collect();
        let reference_values = references.into_values().collect::<Vec<_>>();
        let node_indexes = typed
            .elements
            .iter()
            .enumerate()
            .map(|(index, element)| (occurrence_key(&element.occurrence), index))
            .collect();
        Ok(Self {
            documents: document_records,
            document_indexes,
            sources: source_records,
            source_indexes,
            references: reference_values,
            reference_indexes,
            node_indexes,
        })
    }

    fn projection(&self, typed: &model::DiagramViewProjection) -> Result<Value, String> {
        Ok(json!({
            "kind": kind_id(typed.view.kind),
            "exposedRoots": typed.exposed_roots.iter().map(|value| self.node(value)).collect::<Result<Vec<_>, _>>()?,
            "nodes": typed.elements.iter().map(|value| self.element(value)).collect::<Result<Vec<_>, _>>()?,
            "relationships": typed.relationships.iter().map(|value| self.relationship(value)).collect::<Result<Vec<_>, _>>()?,
            "edges": typed.edges.iter().map(|value| self.edge(value)).collect::<Result<Vec<_>, _>>()?,
            "metadata": self.metadata(&typed.metadata)?,
            "scene": self.scene(&typed.scene)?,
        }))
    }

    fn scene(&self, value: &model::DiagramScene) -> Result<Value, String> {
        let marker = |kind: &'static str| json!({ "kind": kind });
        Ok(match value {
            model::DiagramScene::General => marker("general"),
            model::DiagramScene::Interconnection => marker("interconnection"),
            model::DiagramScene::ActionFlow => marker("action-flow"),
            model::DiagramScene::Sequence => marker("sequence"),
            model::DiagramScene::Browser => marker("browser"),
            model::DiagramScene::Grid => marker("grid"),
            model::DiagramScene::Geometry => marker("geometry"),
            model::DiagramScene::StateTransition(scene) => {
                let vertex_indexes = scene
                    .vertices
                    .iter()
                    .enumerate()
                    .map(|(index, vertex)| (vertex.semantic_id.as_str(), index))
                    .collect::<BTreeMap<_, _>>();
                let transitions =
                    scene
                        .transitions
                        .iter()
                        .enumerate()
                        .map(|(index, transition)| {
                            let source = vertex_indexes
                                .get(transition.source.as_str())
                                .ok_or_else(|| {
                                    format!(
                                        "state transition source `{}` is not a scene vertex",
                                        transition.source
                                    )
                                })?;
                            let target = vertex_indexes
                                .get(transition.target.as_str())
                                .ok_or_else(|| {
                                    format!(
                                        "state transition target `{}` is not a scene vertex",
                                        transition.target
                                    )
                                })?;
                            Ok(json!({
                                "id": format!("transition-{index}"),
                                "label": transition.label,
                                "source": source,
                                "target": target,
                                "trigger": scene_trigger(&transition.trigger, self)?,
                                "guard": scene_feature(&transition.guard, self)?,
                                "effect": scene_feature(&transition.effect, self)?,
                                "provenance": provenance(&transition.provenance),
                                "navigation": self.source(&transition.source_reference)?,
                            }))
                        })
                        .collect::<Result<Vec<Value>, String>>()?;
                json!({
                    "kind": "state-transition",
                    "frame": scene.machine.as_ref().map(|machine| Ok::<Value, String>(json!({
                        "id": "state-machine",
                        "label": machine.label,
                        "navigation": self.source(&machine.source)?,
                    }))).transpose()?,
                    "vertices": scene.vertices.iter().enumerate().map(|(index, vertex)| Ok(json!({
                        "id": format!("state-{index}"),
                        "label": vertex.label,
                        "kind": state_vertex_kind(&vertex.kind),
                        "navigation": self.source(&vertex.source)?,
                    }))).collect::<Result<Vec<Value>, String>>()?,
                    "transitions": transitions,
                })
            }
        })
    }

    fn element(&self, value: &model::DiagramElement) -> Result<Value, String> {
        let typing = match &value.typing {
            model::DiagramElementTyping::Absent => json!({ "status": "absent" }),
            model::DiagramElementTyping::Resolved(values) => {
                json!({ "status": "resolved", "types": values.iter().map(|value| Ok(json!({ "reference": self.reference(&value.reference)?, "label": value.label }))).collect::<Result<Vec<Value>, String>>()? })
            }
            model::DiagramElementTyping::Partial(values) => {
                json!({ "status": "partial", "types": values.iter().map(|value| Ok(json!({ "reference": self.reference(&value.reference)?, "label": value.label }))).collect::<Result<Vec<Value>, String>>()? })
            }
            model::DiagramElementTyping::Ambiguous(targets) => {
                json!({ "status": "ambiguous", "candidates": targets.iter().map(|target| self.reference(target)).collect::<Result<Vec<_>, _>>()? })
            }
            model::DiagramElementTyping::Unresolved => json!({ "status": "unresolved" }),
            model::DiagramElementTyping::Unsupported => json!({ "status": "unsupported" }),
            model::DiagramElementTyping::Recovery => json!({ "status": "recovery" }),
            model::DiagramElementTyping::Incomplete => json!({ "status": "incomplete" }),
        };
        Ok(json!({
            "reference": self.reference(&value.reference)?,
            "metaclass": value.metaclass.as_str(),
            "notationRole": notation_role(value.notation_role),
            "name": value.name,
            "typing": typing,
            "owner": value.owner.as_ref().map(|owner| self.node(owner)).transpose()?,
            "source": self.source(&value.source)?,
            "compartments": value.compartments.iter().map(|compartment| Ok(json!({
                "kind": compartment_kind(compartment.kind),
                "provenance": compartment_provenance(compartment.provenance),
                "members": compartment.members.iter().map(|member| self.node(member)).collect::<Result<Vec<_>, _>>()?,
            }))).collect::<Result<Vec<Value>, String>>()?,
        }))
    }

    fn relationship(&self, value: &model::DiagramRelationship) -> Result<Value, String> {
        let target = match &value.target {
            model::DiagramRelationshipTarget::Resolved(endpoint) => match &endpoint.occurrence {
                model::DiagramEndpointOccurrence::Resolved(occurrence) => {
                    json!({ "status": "resolved", "node": self.node(occurrence)? })
                }
                model::DiagramEndpointOccurrence::Ambiguous(occurrences) => json!({
                    "status": "ambiguous",
                    "candidates": occurrences.iter().map(|value| self.node(value)).collect::<Result<Vec<_>, _>>()?
                }),
                model::DiagramEndpointOccurrence::OutsideProjection => {
                    json!({ "status": "resolved", "reference": self.reference(&endpoint.reference)? })
                }
            },
            model::DiagramRelationshipTarget::Ambiguous(values) => json!({
                "status": "ambiguous",
                "candidates": values.iter().map(|value| self.reference(&value.reference)).collect::<Result<Vec<_>, _>>()?
            }),
            model::DiagramRelationshipTarget::Unresolved => json!({ "status": "unresolved" }),
            model::DiagramRelationshipTarget::Unsupported => json!({ "status": "unsupported" }),
        };
        Ok(json!({
            "reference": self.reference(&value.reference)?,
            "source": self.node(&value.source_occurrence)?,
            "kind": value.kind.as_str(),
            "target": target,
            "provenance": provenance(&value.provenance),
            "navigation": value.source.as_ref().map(|source| self.source(source)).transpose()?,
        }))
    }

    fn edge(&self, value: &model::DiagramEdge) -> Result<Value, String> {
        Ok(json!({
            "reference": self.reference(&value.reference)?,
            "source": self.node(&value.source_occurrence)?,
            "target": self.node(&value.target_occurrence)?,
            "origin": self.node(&value.origin_occurrence)?,
            "kind": edge_kind(&value.kind),
            "provenance": provenance(&value.provenance),
            "navigation": value.source.as_ref().map(|source| self.source(source)).transpose()?,
        }))
    }

    fn metadata(&self, value: &model::DiagramViewMetadata) -> Result<Value, String> {
        let nodes = |values: &[model::DiagramOccurrenceIdentity]| {
            values
                .iter()
                .map(|value| self.node(value))
                .collect::<Result<Vec<_>, _>>()
        };
        Ok(match value {
            model::DiagramViewMetadata::General { roots } => json!({ "roots": nodes(roots)? }),
            model::DiagramViewMetadata::Interconnection {
                parts,
                ports,
                connectors,
            } => {
                json!({ "parts": nodes(parts)?, "ports": nodes(ports)?, "connectors": nodes(connectors)? })
            }
            model::DiagramViewMetadata::ActionFlow {
                actions,
                control_nodes,
            } => json!({ "actions": nodes(actions)?, "controlNodes": nodes(control_nodes)? }),
            model::DiagramViewMetadata::StateTransition {
                states,
                initial_nodes,
                final_nodes,
            } => {
                json!({ "states": nodes(states)?, "initialNodes": nodes(initial_nodes)?, "finalNodes": nodes(final_nodes)? })
            }
            model::DiagramViewMetadata::Sequence {
                participants,
                messages,
            } => json!({ "participants": nodes(participants)?, "messages": nodes(messages)? }),
            model::DiagramViewMetadata::Browser { roots } => json!({ "roots": nodes(roots)? }),
            model::DiagramViewMetadata::Grid {
                rows,
                columns,
                cells,
            } => json!({
                "rows": nodes(rows)?,
                "columns": columns.iter().map(|column| column.as_str()).collect::<Vec<_>>(),
                "cells": cells.iter().map(|cell| Ok(json!({
                    "row": self.node(&cell.row)?,
                    "column": cell.column.as_str(),
                    "relationship": self.reference(&cell.relationship)?,
                }))).collect::<Result<Vec<Value>, String>>()?,
            }),
            model::DiagramViewMetadata::Geometry {
                elements,
                primitives,
            } => json!({ "elements": nodes(elements)?, "primitives": nodes(primitives)? }),
        })
    }

    fn incomplete_reason(&self, value: &model::DiagramIncompleteReason) -> Result<Value, String> {
        Ok(match value {
            model::DiagramIncompleteReason::ParseRecovery => json!({ "code": "parse-recovery" }),
            model::DiagramIncompleteReason::UnsupportedSyntax => {
                json!({ "code": "unsupported-syntax" })
            }
            model::DiagramIncompleteReason::NonConverged => json!({ "code": "non-converged" }),
            model::DiagramIncompleteReason::ExposureUnresolved { exposure } => {
                json!({ "code": "exposure-unresolved", "exposure": self.reference(exposure)? })
            }
            model::DiagramIncompleteReason::ExposureAmbiguous { exposure } => {
                json!({ "code": "exposure-ambiguous", "exposure": self.reference(exposure)? })
            }
            model::DiagramIncompleteReason::ExposureUnsupported { exposure } => {
                json!({ "code": "exposure-unsupported", "exposure": self.reference(exposure)? })
            }
            model::DiagramIncompleteReason::RelationshipUnresolved { relationship_kind } => {
                json!({ "code": "relationship-unresolved", "relationshipKind": relationship_kind })
            }
            model::DiagramIncompleteReason::RelationshipAmbiguous { relationship_kind } => {
                json!({ "code": "relationship-ambiguous", "relationshipKind": relationship_kind })
            }
            model::DiagramIncompleteReason::RelationshipUnsupported { relationship_kind } => {
                json!({ "code": "relationship-unsupported", "relationshipKind": relationship_kind })
            }
            model::DiagramIncompleteReason::ViewFilterUnresolved => {
                json!({ "code": "view-filter-unresolved" })
            }
            model::DiagramIncompleteReason::ViewFilterAmbiguous => {
                json!({ "code": "view-filter-ambiguous" })
            }
            model::DiagramIncompleteReason::ViewFilterUnsupported => {
                json!({ "code": "view-filter-unsupported" })
            }
            model::DiagramIncompleteReason::GeometryFactsUnavailable => {
                json!({ "code": "geometry-facts-unavailable" })
            }
        })
    }

    fn semantic_reference(&self, value: &model::DiagramSemanticReference) -> Result<Value, String> {
        Ok(match value {
            model::DiagramSemanticReference::Qualified {
                document,
                qualified_name,
                source_domain,
            } => json!({
                "kind": "qualified-name",
                "document": self.document(document, *source_domain)?,
                "qualifiedName": qualified_name,
            }),
            model::DiagramSemanticReference::ToolingElementId {
                element_id,
                source_domain,
            } => json!({
                "kind": "tooling-element-id",
                "elementId": element_id,
                "sourceDomain": source_domain_name(*source_domain),
            }),
            model::DiagramSemanticReference::SourceAnchor {
                document,
                owner_qualified_name,
                metaclass,
                source_domain,
                range,
            } => json!({
                "kind": "source-anchor",
                "source": self.source_key(document, range)?,
                "ownerQualifiedName": owner_qualified_name,
                "metaclass": metaclass.as_str(),
                "sourceDomain": source_domain_name(*source_domain),
            }),
            model::DiagramSemanticReference::Relationship {
                document,
                source_qualified_name,
                relationship_kind,
                ordinal,
                source_domain,
            } => json!({
                "kind": "relationship",
                "source": self.reference(&model::DiagramSemanticReference::Qualified {
                    document: document.clone(),
                    qualified_name: source_qualified_name.clone(),
                    source_domain: *source_domain,
                })?,
                "relationshipKind": relationship_kind.as_str(),
                "ordinal": ordinal,
            }),
        })
    }

    fn document(
        &self,
        uri: &str,
        domain: model::DiagramSourceDomain,
    ) -> Result<DocumentIndex, String> {
        self.document_indexes
            .get(&(uri.to_owned(), domain_rank(domain)))
            .copied()
            .ok_or_else(|| format!("diagram document `{uri}` was not interned"))
    }

    fn source(&self, value: &model::SourceReference) -> Result<SourceIndex, String> {
        self.source_key(&value.uri, &value.range)
    }

    fn source_key(&self, uri: &str, range: &model::SourceRange) -> Result<SourceIndex, String> {
        self.source_indexes
            .get(&SourceKey(uri.to_owned(), range_array(range)))
            .copied()
            .ok_or_else(|| format!("diagram source `{uri}` was not interned"))
    }

    fn reference(&self, value: &model::DiagramSemanticReference) -> Result<ReferenceIndex, String> {
        self.reference_indexes
            .get(&reference_key(value))
            .copied()
            .ok_or_else(|| "diagram semantic reference was not interned".to_owned())
    }

    fn node(&self, value: &model::DiagramOccurrenceIdentity) -> Result<NodeIndex, String> {
        self.node_indexes
            .get(&occurrence_key(value))
            .copied()
            .ok_or_else(|| {
                "diagram occurrence is outside the projected node set".to_owned()
            })
    }
}

fn occurrence_key(value: &model::DiagramOccurrenceIdentity) -> OccurrenceKey {
    OccurrenceKey(value.semantic_path.iter().map(reference_key).collect())
}

fn collect_occurrence_references(
    values: &mut BTreeMap<ReferenceKey, model::DiagramSemanticReference>,
    occurrence: &model::DiagramOccurrenceIdentity,
) {
    for reference in &occurrence.semantic_path {
        collect_reference(values, reference);
    }
}

fn collect_endpoint_occurrences(
    values: &mut BTreeMap<ReferenceKey, model::DiagramSemanticReference>,
    occurrence: &model::DiagramEndpointOccurrence,
) {
    match occurrence {
        model::DiagramEndpointOccurrence::Resolved(value) => {
            collect_occurrence_references(values, value)
        }
        model::DiagramEndpointOccurrence::Ambiguous(values_to_collect) => {
            for value in values_to_collect {
                collect_occurrence_references(values, value);
            }
        }
        model::DiagramEndpointOccurrence::OutsideProjection => {}
    }
}

fn collect_reference(
    values: &mut BTreeMap<ReferenceKey, model::DiagramSemanticReference>,
    value: &model::DiagramSemanticReference,
) {
    if let model::DiagramSemanticReference::Relationship {
        document,
        source_qualified_name,
        source_domain,
        ..
    } = value
    {
        let source = model::DiagramSemanticReference::Qualified {
            document: document.clone(),
            qualified_name: source_qualified_name.clone(),
            source_domain: *source_domain,
        };
        values.entry(reference_key(&source)).or_insert(source);
    }
    values
        .entry(reference_key(value))
        .or_insert_with(|| value.clone());
}

fn collect_source(
    values: &mut BTreeMap<SourceKey, model::SourceReference>,
    value: &model::SourceReference,
) {
    values
        .entry(SourceKey(value.uri.clone(), range_array(&value.range)))
        .or_insert_with(|| value.clone());
}

fn collect_scene_sources(
    values: &mut BTreeMap<SourceKey, model::SourceReference>,
    scene: &model::DiagramScene,
) {
    let model::DiagramScene::StateTransition(scene) = scene else {
        return;
    };
    if let Some(machine) = &scene.machine {
        collect_source(values, &machine.source);
    }
    for vertex in &scene.vertices {
        collect_source(values, &vertex.source);
    }
    for transition in &scene.transitions {
        collect_source(values, &transition.source_reference);
        if let model::TransitionTrigger::Accept { source, .. } = &transition.trigger {
            collect_source(values, source);
        }
        for feature in [&transition.guard, &transition.effect] {
            if let model::ProjectionFeature::Supported { source, .. } = feature {
                collect_source(values, source);
            }
        }
    }
}

fn state_vertex_kind(value: &model::StateTransitionNodeKind) -> &'static str {
    match value {
        model::StateTransitionNodeKind::Initial => "initial",
        model::StateTransitionNodeKind::State => "state",
        model::StateTransitionNodeKind::Final => "final",
    }
}

fn scene_trigger(
    value: &model::TransitionTrigger,
    normalized: &NormalizedProduct,
) -> Result<Value, String> {
    Ok(match value {
        model::TransitionTrigger::None => json!({ "status": "absent" }),
        model::TransitionTrigger::Accept {
            label,
            target,
            source,
        } => json!({
            "status": "accept",
            "label": label,
            "target": target.as_ref().map(|target| json!({ "id": target.semantic_id, "label": target.label })),
            "navigation": normalized.source(source)?,
        }),
        model::TransitionTrigger::Unsupported { reason } => json!({
            "status": "unsupported",
            "code": reason.code,
            "message": reason.message,
        }),
        model::TransitionTrigger::Unresolved => json!({ "status": "unresolved" }),
        model::TransitionTrigger::Ambiguous => json!({ "status": "ambiguous" }),
    })
}

fn scene_feature(
    value: &model::ProjectionFeature,
    normalized: &NormalizedProduct,
) -> Result<Value, String> {
    Ok(match value {
        model::ProjectionFeature::Absent => json!({ "status": "absent" }),
        model::ProjectionFeature::Supported { label, source } => json!({
            "status": "supported",
            "label": label,
            "navigation": normalized.source(source)?,
        }),
        model::ProjectionFeature::Unsupported { reason } => json!({
            "status": "unsupported",
            "code": reason.code,
            "message": reason.message,
        }),
        model::ProjectionFeature::Unresolved => json!({ "status": "unresolved" }),
        model::ProjectionFeature::Ambiguous => json!({ "status": "ambiguous" }),
        model::ProjectionFeature::Recovery => json!({ "status": "recovery" }),
    })
}

fn collect_reason_reference(
    values: &mut BTreeMap<ReferenceKey, model::DiagramSemanticReference>,
    reason: &model::DiagramIncompleteReason,
) {
    match reason {
        model::DiagramIncompleteReason::ExposureUnresolved { exposure }
        | model::DiagramIncompleteReason::ExposureAmbiguous { exposure }
        | model::DiagramIncompleteReason::ExposureUnsupported { exposure } => {
            collect_reference(values, exposure)
        }
        _ => {}
    }
}

fn collect_metadata_references(
    values: &mut BTreeMap<ReferenceKey, model::DiagramSemanticReference>,
    metadata: &model::DiagramViewMetadata,
) {
    let mut add = |occurrences: &[model::DiagramOccurrenceIdentity]| {
        occurrences
            .iter()
            .for_each(|value| collect_occurrence_references(values, value))
    };
    match metadata {
        model::DiagramViewMetadata::General { roots }
        | model::DiagramViewMetadata::Browser { roots } => add(roots),
        model::DiagramViewMetadata::Interconnection {
            parts,
            ports,
            connectors,
        } => {
            add(parts);
            add(ports);
            add(connectors);
        }
        model::DiagramViewMetadata::ActionFlow {
            actions,
            control_nodes,
        } => {
            add(actions);
            add(control_nodes);
        }
        model::DiagramViewMetadata::StateTransition {
            states,
            initial_nodes,
            final_nodes,
        } => {
            add(states);
            add(initial_nodes);
            add(final_nodes);
        }
        model::DiagramViewMetadata::Sequence {
            participants,
            messages,
        } => {
            add(participants);
            add(messages);
        }
        model::DiagramViewMetadata::Grid { rows, cells, .. } => {
            add(rows);
            cells.iter().for_each(|cell| {
                collect_occurrence_references(values, &cell.row);
                collect_reference(values, &cell.relationship);
            });
        }
        model::DiagramViewMetadata::Geometry {
            elements,
            primitives,
        } => {
            add(elements);
            add(primitives);
        }
    }
}

fn reference_key(value: &model::DiagramSemanticReference) -> ReferenceKey {
    match value {
        model::DiagramSemanticReference::Qualified {
            document,
            qualified_name,
            source_domain,
        } => ReferenceKey::Qualified(
            domain_rank(*source_domain),
            document.clone(),
            qualified_name.clone(),
        ),
        model::DiagramSemanticReference::ToolingElementId {
            element_id,
            source_domain,
        } => ReferenceKey::Tooling(domain_rank(*source_domain), element_id.clone()),
        model::DiagramSemanticReference::SourceAnchor {
            document,
            owner_qualified_name,
            metaclass,
            source_domain,
            range,
        } => ReferenceKey::Anchor(
            domain_rank(*source_domain),
            document.clone(),
            owner_qualified_name.clone(),
            metaclass.as_str().to_owned(),
            range_array(range),
        ),
        model::DiagramSemanticReference::Relationship {
            document,
            source_qualified_name,
            relationship_kind,
            ordinal,
            source_domain,
        } => ReferenceKey::Relationship(
            domain_rank(*source_domain),
            document.clone(),
            source_qualified_name.clone(),
            relationship_kind.as_str().to_owned(),
            *ordinal,
        ),
    }
}

fn reference_document(
    value: &model::DiagramSemanticReference,
) -> Option<(&str, model::DiagramSourceDomain)> {
    match value {
        model::DiagramSemanticReference::Qualified {
            document,
            source_domain,
            ..
        }
        | model::DiagramSemanticReference::SourceAnchor {
            document,
            source_domain,
            ..
        }
        | model::DiagramSemanticReference::Relationship {
            document,
            source_domain,
            ..
        } => Some((document, *source_domain)),
        model::DiagramSemanticReference::ToolingElementId { .. } => None,
    }
}

fn range_array(value: &model::SourceRange) -> [u32; 4] {
    [
        value.start_line,
        value.start_character,
        value.end_line,
        value.end_character,
    ]
}

fn domain_rank(value: model::DiagramSourceDomain) -> u8 {
    match value {
        model::DiagramSourceDomain::Workspace => 0,
        model::DiagramSourceDomain::StandardLibrary => 1,
        model::DiagramSourceDomain::Library => 2,
        model::DiagramSourceDomain::External => 3,
    }
}

fn source_domain_rank(value: &str) -> u8 {
    match value {
        "workspace" => 0,
        "standard-library" => 1,
        "library" => 2,
        "external" => 3,
        _ => unreachable!(),
    }
}

fn source_domain(value: model::DiagramSourceDomain) -> &'static str {
    source_domain_name(value)
}
fn source_domain_name(value: model::DiagramSourceDomain) -> &'static str {
    match value {
        model::DiagramSourceDomain::Workspace => "workspace",
        model::DiagramSourceDomain::StandardLibrary => "standard-library",
        model::DiagramSourceDomain::Library => "library",
        model::DiagramSourceDomain::External => "external",
    }
}

fn provenance(value: &model::RelationshipProvenance) -> &'static str {
    match value {
        model::RelationshipProvenance::Authored => "authored",
        model::RelationshipProvenance::Implied => "implied",
    }
}

fn edge_kind(value: &model::DiagramEdgeKind) -> &str {
    match value {
        model::DiagramEdgeKind::Containment => "containment",
        model::DiagramEdgeKind::Connector => "connector",
        model::DiagramEdgeKind::Flow => "flow",
        model::DiagramEdgeKind::Succession => "succession",
        model::DiagramEdgeKind::Transition => "transition",
        model::DiagramEdgeKind::InitialState => "initial-state",
        model::DiagramEdgeKind::Relationship(kind) => kind.as_str(),
    }
}

fn notation_role(value: model::DiagramNotationRole) -> &'static str {
    match value {
        model::DiagramNotationRole::Definition => "definition",
        model::DiagramNotationRole::Usage => "usage",
        model::DiagramNotationRole::ReferenceUsage => "reference-usage",
        model::DiagramNotationRole::Namespace => "namespace",
        model::DiagramNotationRole::Annotation => "annotation",
        model::DiagramNotationRole::Unsupported => "unsupported",
    }
}

fn compartment_kind(value: model::DiagramCompartmentKind) -> &'static str {
    match value {
        model::DiagramCompartmentKind::Attributes => "attributes",
        model::DiagramCompartmentKind::Parts => "parts",
        model::DiagramCompartmentKind::Ports => "ports",
        model::DiagramCompartmentKind::Items => "items",
        model::DiagramCompartmentKind::Constraints => "constraints",
        model::DiagramCompartmentKind::Requirements => "requirements",
        model::DiagramCompartmentKind::Actions => "actions",
        model::DiagramCompartmentKind::States => "states",
        model::DiagramCompartmentKind::Calculations => "calculations",
        model::DiagramCompartmentKind::Connections => "connections",
        model::DiagramCompartmentKind::Interfaces => "interfaces",
        model::DiagramCompartmentKind::Occurrences => "occurrences",
    }
}

fn compartment_provenance(value: model::DiagramCompartmentProvenance) -> &'static str {
    match value {
        model::DiagramCompartmentProvenance::Direct => "direct",
        model::DiagramCompartmentProvenance::Inherited => "inherited",
    }
}

fn kind_id(kind: model::DiagramViewKind) -> &'static str {
    match kind {
        model::DiagramViewKind::GeneralView => "general-view",
        model::DiagramViewKind::InterconnectionView => "interconnection-view",
        model::DiagramViewKind::ActionFlowView => "action-flow-view",
        model::DiagramViewKind::StateTransitionView => "state-transition-view",
        model::DiagramViewKind::SequenceView => "sequence-view",
        model::DiagramViewKind::BrowserView => "browser-view",
        model::DiagramViewKind::GridView => "grid-view",
        model::DiagramViewKind::GeometryView => "geometry-view",
    }
}

export!(DiagramGenerator);

#[cfg(test)]
mod tests {
    use super::*;

    fn qualified(name: &str) -> model::DiagramSemanticReference {
        model::DiagramSemanticReference::Qualified {
            document: "file:///workspace/model.sysml".to_owned(),
            qualified_name: name.to_owned(),
            source_domain: model::DiagramSourceDomain::Workspace,
        }
    }

    fn occurrence(name: &str) -> model::DiagramOccurrenceIdentity {
        model::DiagramOccurrenceIdentity {
            semantic_path: vec![qualified(name)],
        }
    }

    fn source(line: u32) -> model::SourceReference {
        model::SourceReference {
            uri: "file:///workspace/model.sysml".to_owned(),
            range: model::SourceRange {
                start_line: line,
                start_character: 0,
                end_line: line,
                end_character: 1,
            },
        }
    }

    fn projection(elements: Vec<model::DiagramElement>) -> model::DiagramViewProjection {
        model::DiagramViewProjection {
            schema_version: 1,
            model_digest: "blake3:model".to_owned(),
            view: model::DiagramViewSummary {
                handle: "opaque".to_owned(),
                reference: qualified("P::view"),
                kind: model::DiagramViewKind::GeneralView,
                name: "view".to_owned(),
                source: source(0),
            },
            completeness: model::ProjectionCompleteness::Complete,
            incomplete_reasons: Vec::new(),
            exposed_roots: vec![occurrence("P::root")],
            elements,
            relationships: Vec::new(),
            edges: Vec::new(),
            metadata: model::DiagramViewMetadata::General {
                roots: vec![occurrence("P::root")],
            },
            scene: model::DiagramScene::General,
        }
    }

    #[test]
    fn normalization_interns_references_and_sources_once() {
        let product = projection(vec![model::DiagramElement {
            occurrence: occurrence("P::root"),
            reference: qualified("P::root"),
            metaclass: model::Metaclass::PartDefinition,
            notation_role: model::DiagramNotationRole::Definition,
            name: Some("root".to_owned()),
            typing: model::DiagramElementTyping::Absent,
            owner: None,
            source: source(1),
            compartments: Vec::new(),
        }]);
        let normalized = NormalizedProduct::new(&product).expect("normalized product");
        assert_eq!(normalized.documents.len(), 1);
        assert_eq!(normalized.sources.len(), 2);
        assert_eq!(normalized.references.len(), 2);
        assert_eq!(normalized.reference(&qualified("P::root")).unwrap(), 0);
        assert_eq!(normalized.node(&occurrence("P::root")).unwrap(), 0);
        assert_eq!(
            normalized.element(&product.elements[0]).unwrap()["notationRole"],
            "definition"
        );
    }

    #[test]
    fn reference_table_order_does_not_depend_on_node_insertion_order() {
        let element = |name: &str, line| model::DiagramElement {
            occurrence: occurrence(name),
            reference: qualified(name),
            metaclass: model::Metaclass::PartDefinition,
            notation_role: model::DiagramNotationRole::Definition,
            name: Some(name.to_owned()),
            typing: model::DiagramElementTyping::Absent,
            owner: None,
            source: source(line),
            compartments: Vec::new(),
        };
        let forward =
            NormalizedProduct::new(&projection(vec![element("P::a", 1), element("P::b", 2)]))
                .unwrap();
        let reverse =
            NormalizedProduct::new(&projection(vec![element("P::b", 2), element("P::a", 1)]))
                .unwrap();
        assert_eq!(forward.reference_indexes, reverse.reference_indexes);
        assert_eq!(forward.source_indexes, reverse.source_indexes);
        assert_eq!(forward.document_indexes, reverse.document_indexes);
    }

    #[test]
    fn state_scene_serializes_vertices_and_transitions_independently_of_the_generic_graph() {
        let mut product = projection(Vec::new());
        product.view.kind = model::DiagramViewKind::StateTransitionView;
        product.exposed_roots.clear();
        product.metadata = model::DiagramViewMetadata::StateTransition {
            states: Vec::new(),
            initial_nodes: Vec::new(),
            final_nodes: Vec::new(),
        };
        product.scene = model::DiagramScene::StateTransition(model::StateTransitionScene {
            machine: Some(model::StateMachineSummary {
                semantic_id: "P::Machine".to_owned(),
                label: "Machine".to_owned(),
                source: source(1),
            }),
            vertices: vec![
                model::StateTransitionNode {
                    semantic_id: "P::Machine#initial".to_owned(),
                    label: String::new(),
                    kind: model::StateTransitionNodeKind::Initial,
                    source: source(2),
                },
                model::StateTransitionNode {
                    semantic_id: "P::Machine::idle".to_owned(),
                    label: "idle".to_owned(),
                    kind: model::StateTransitionNodeKind::State,
                    source: source(3),
                },
            ],
            transitions: vec![model::StateTransitionEdge {
                semantic_id: "P::Machine#entry".to_owned(),
                label: None,
                source: "P::Machine#initial".to_owned(),
                target: "P::Machine::idle".to_owned(),
                trigger: model::TransitionTrigger::None,
                guard: model::ProjectionFeature::Absent,
                effect: model::ProjectionFeature::Absent,
                provenance: model::RelationshipProvenance::Authored,
                source_reference: source(2),
            }],
        });
        let normalized = NormalizedProduct::new(&product).unwrap();
        let scene = normalized.projection(&product).unwrap()["scene"].clone();
        assert_eq!(scene["kind"], "state-transition");
        assert_eq!(scene["vertices"].as_array().unwrap().len(), 2);
        assert_eq!(scene["transitions"][0]["source"], 0);
        assert_eq!(scene["transitions"][0]["target"], 1);
    }

    #[test]
    fn normalized_product_scales_to_thousands_of_graph_records() {
        let elements = (0..1_000)
            .map(|index| model::DiagramElement {
                occurrence: occurrence(&format!("P::n{index:04}")),
                reference: qualified(&format!("P::n{index:04}")),
                metaclass: model::Metaclass::PartUsage,
                notation_role: model::DiagramNotationRole::Usage,
                name: Some(format!("n{index}")),
                typing: model::DiagramElementTyping::Absent,
                owner: None,
                source: source(index),
                compartments: Vec::new(),
            })
            .collect::<Vec<_>>();
        let mut product = projection(elements);
        product.exposed_roots = vec![occurrence("P::n0000")];
        product.metadata = model::DiagramViewMetadata::General {
            roots: product.exposed_roots.clone(),
        };
        product.edges = (0..10_000)
            .map(|ordinal| {
                let source_index = ordinal % 1_000;
                let target_index = (ordinal + 1) % 1_000;
                model::DiagramEdge {
                    reference: model::DiagramSemanticReference::Relationship {
                        document: "file:///workspace/model.sysml".to_owned(),
                        source_qualified_name: format!("P::n{source_index:04}"),
                        relationship_kind: model::RelationshipKind::Flow,
                        ordinal,
                        source_domain: model::DiagramSourceDomain::Workspace,
                    },
                    source_element: qualified(&format!("P::n{source_index:04}")),
                    target_element: qualified(&format!("P::n{target_index:04}")),
                    source_occurrence: occurrence(&format!("P::n{source_index:04}")),
                    target_occurrence: occurrence(&format!("P::n{target_index:04}")),
                    kind: model::DiagramEdgeKind::Flow,
                    provenance: model::RelationshipProvenance::Authored,
                    source: None,
                }
            })
            .collect();
        let normalized = NormalizedProduct::new(&product).expect("large normalized product");
        let projection = normalized
            .projection(&product)
            .expect("large graph projection");
        let references = normalized
            .references
            .iter()
            .map(|reference| normalized.semantic_reference(reference))
            .collect::<Result<Vec<_>, _>>()
            .expect("large reference table");
        let bytes = serde_json::to_vec(&(references, projection)).expect("large JSON");
        assert_eq!(normalized.documents.len(), 1);
        assert_eq!(normalized.node_indexes.len(), 1_000);
        assert_eq!(normalized.reference_indexes.len(), 11_001);
        assert!(
            bytes.len() < 2_500_000,
            "normalized graph was {} bytes",
            bytes.len()
        );
    }
}
