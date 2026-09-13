//! JSON serialisation of the typed `PublishedModelProjection` for `spec42 model-export
//! --format json`.
//!
//! The projection itself is an in-process value owned by `sysml_query`: elements are `SymbolId`s,
//! text bodies are `TextId` handles. This module is the boundary layer that materialises a
//! `SymbolToken` for every identity and the interned text for every handle, so a headless
//! consumer (CI, a script, an agent) gets one settled document. It adds nothing to the
//! projection -- every field here has a typed producer on the publication.

use serde::Serialize;
use serde_json::{json, Value};
use sysml_query::resolved_slice::{
    ConnectedElement, ConnectorEndpoint, ElementDetails, EvaluatedScalar, ExpressionNode,
    ExpressionNodeKind, MetadataAnnotationValue, MultiplicityBound, MultiplicityFacts,
    ProjectedElement, ProjectionEnvelope, PublishedConnector, PublishedExpression,
    PublishedMetadataAnnotation, PublishedModel, RelationshipFamily, RelationshipTarget,
    SourceLocation, SymbolId, SymbolToken,
};

/// The JSON form of one publication's whole-model projection.
#[derive(Debug, Serialize)]
pub struct ModelProjectionJson {
    pub schema_version: u32,
    pub envelope: EnvelopeJson,
    pub elements: Vec<Value>,
    pub connectors: Vec<Value>,
    pub truncation: TruncationJson,
}

#[derive(Debug, Serialize)]
pub struct EnvelopeJson {
    pub phase: &'static str,
    pub complete: bool,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub obstacles: Vec<&'static str>,
    pub has_evaluation: bool,
    pub source_digest: String,
    pub model_digest: String,
    pub admitted: AdmittedJson,
}

#[derive(Debug, Serialize)]
pub struct AdmittedJson {
    pub standard_library: usize,
    pub library: usize,
    pub external: usize,
}

#[derive(Debug, Serialize)]
pub struct TruncationJson {
    pub elements_total: usize,
    pub elements_returned: usize,
    pub elements_incomplete: usize,
}

/// Serialises `model.projection().model(max_nodes)` for the given publication.
///
/// `model_projection` composes [`sysml_query::resolved_slice::QueryAnswer::Resolved`]-only
/// queries (`all_elements`), so it always resolves; there is no non-`Resolved` answer for this
/// caller to fall back on.
pub fn model_projection_json(model: &PublishedModel, max_nodes: usize) -> ModelProjectionJson {
    let sysml_query::resolved_slice::QueryAnswer::Resolved(projection) =
        model.projection().model(max_nodes).answer
    else {
        unreachable!(
            "model_projection composes only Resolved-answer queries (all_elements), so this \
             is unreachable"
        )
    };

    ModelProjectionJson {
        schema_version: projection.schema_version,
        envelope: envelope_json(&projection.envelope),
        elements: projection
            .elements
            .iter()
            .map(|element| element_json(model, element))
            .collect(),
        connectors: projection
            .connectors
            .iter()
            .map(|connector| connector_json(model, connector))
            .collect(),
        truncation: TruncationJson {
            elements_total: projection.truncation.elements_total,
            elements_returned: projection.truncation.elements_returned,
            elements_incomplete: projection.truncation.elements_incomplete,
        },
    }
}

fn envelope_json(envelope: &ProjectionEnvelope) -> EnvelopeJson {
    let phase = envelope.phase.as_str();
    let obstacles = envelope
        .completeness
        .obstacles()
        .map(sysml_query::resolved_slice::PublicationObstacle::as_str)
        .collect();
    EnvelopeJson {
        phase,
        complete: envelope.completeness.is_complete(),
        obstacles,
        has_evaluation: envelope.has_evaluation,
        source_digest: envelope.source_digest.to_string(),
        model_digest: envelope.model_digest.to_string(),
        admitted: AdmittedJson {
            standard_library: envelope.admitted.standard_library,
            library: envelope.admitted.library,
            external: envelope.admitted.external,
        },
    }
}

fn token(model: &PublishedModel, symbol: SymbolId) -> Option<String> {
    model.symbol_token(symbol).map(SymbolToken::into_string)
}

fn location_json(model: &PublishedModel, location: &SourceLocation) -> Value {
    json!({
        "uri": model.document_identity(location.document).unwrap_or_default(),
        "range": {
            "start": { "line": location.range.start.line, "character": location.range.start.character },
            "end": { "line": location.range.end.line, "character": location.range.end.character },
        },
    })
}

fn target_json(model: &PublishedModel, target: &RelationshipTarget) -> Value {
    match target {
        RelationshipTarget::Resolved(symbol) => json!({
            "status": "resolved",
            "token": token(model, *symbol),
            "qualified_name": model.qualified_name(*symbol),
        }),
        RelationshipTarget::Ambiguous(candidates) => json!({
            "status": "ambiguous",
            "candidates": candidates
                .iter()
                .map(|symbol| json!({
                    "token": token(model, *symbol),
                    "qualified_name": model.qualified_name(*symbol),
                }))
                .collect::<Vec<_>>(),
        }),
        RelationshipTarget::Unresolved => json!({ "status": "unresolved" }),
        RelationshipTarget::Unsupported => json!({ "status": "unsupported" }),
    }
}

fn family_json(model: &PublishedModel, family: &RelationshipFamily) -> Value {
    json!({
        "outcome": family.outcome.as_str(),
        "targets": symbol_tokens(model, family.targets.iter().map(|entry| entry.identity)),
        "candidates": symbol_tokens(model, family.candidates.iter().map(|entry| entry.identity)),
    })
}

fn symbol_tokens(model: &PublishedModel, symbols: impl Iterator<Item = SymbolId>) -> Vec<Value> {
    symbols
        .map(|symbol| {
            json!({
                "token": token(model, symbol),
                "qualified_name": model.qualified_name(symbol),
            })
        })
        .collect()
}

fn connected_json(model: &PublishedModel, connected: &[ConnectedElement]) -> Vec<Value> {
    connected
        .iter()
        .map(|entry| {
            json!({
                "kind": entry.kind,
                "provenance": provenance(entry.provenance),
                "peer": {
                    "token": token(model, entry.peer.identity),
                    "qualified_name": model.qualified_name(entry.peer.identity),
                },
            })
        })
        .collect()
}

fn provenance(provenance: sysml_query::resolved_slice::RelationshipProvenance) -> &'static str {
    match provenance {
        sysml_query::resolved_slice::RelationshipProvenance::Authored => "authored",
        sysml_query::resolved_slice::RelationshipProvenance::Implied => "implied",
    }
}

fn multiplicity_json(multiplicity: MultiplicityFacts) -> Option<Value> {
    match multiplicity {
        MultiplicityFacts::Absent => None,
        MultiplicityFacts::Declared {
            lower,
            upper,
            ordered,
            nonunique,
        } => Some(json!({
            "lower": bound_json(lower),
            "upper": bound_json(upper),
            "ordered": ordered,
            "nonunique": nonunique,
        })),
    }
}

fn bound_json(bound: MultiplicityBound) -> Value {
    match bound {
        MultiplicityBound::Unbounded => json!("unbounded"),
        MultiplicityBound::Literal(value) => json!(value),
        MultiplicityBound::Expression => json!("expression"),
    }
}

fn scalar_json(scalar: &EvaluatedScalar) -> Value {
    match scalar {
        EvaluatedScalar::Boolean(value) => json!(value),
        EvaluatedScalar::Integer(value) => json!(value),
        EvaluatedScalar::Real(value) => json!(value),
        EvaluatedScalar::String(value) => json!(value),
        EvaluatedScalar::Quantity { magnitude, unit } => json!({
            "magnitude": scalar_json(magnitude),
            "unit": unit,
        }),
    }
}

fn expression_json(model: &PublishedModel, expression: &PublishedExpression) -> Value {
    json!({
        "outcome": expression.outcome.as_str(),
        "root": expression.root,
        "nodes": expression
            .nodes
            .iter()
            .map(|node| expression_node_json(model, node))
            .collect::<Vec<_>>(),
    })
}

fn expression_node_json(model: &PublishedModel, node: &ExpressionNode) -> Value {
    let mut value = match &node.kind {
        ExpressionNodeKind::Literal(scalar) => json!({
            "kind": "literal",
            "value": scalar_json(scalar),
        }),
        ExpressionNodeKind::FeatureReference { symbol, authored } => json!({
            "kind": "feature-reference",
            "authored": authored.as_ref(),
            "token": symbol.and_then(|symbol| token(model, symbol)),
        }),
        ExpressionNodeKind::Operator { operator, operands } => json!({
            "kind": "operator",
            "operator": operator.as_str(),
            "operands": operands.iter().collect::<Vec<_>>(),
        }),
        ExpressionNodeKind::Unsupported { children } => json!({
            "kind": "unsupported",
            "children": children.iter().collect::<Vec<_>>(),
        }),
    };
    value["location"] = location_json(model, &node.location);
    value
}

fn annotation_json(model: &PublishedModel, annotation: &PublishedMetadataAnnotation) -> Value {
    json!({
        "form": annotation.form.as_str(),
        "definition": target_json(model, &annotation.definition),
        "about": annotation
            .about
            .iter()
            .map(|target| target_json(model, target))
            .collect::<Vec<_>>(),
        "body": annotation
            .body
            .iter()
            .map(|value| annotation_value_json(model, value))
            .collect::<Vec<_>>(),
        "location": location_json(model, &annotation.location),
    })
}

fn annotation_value_json(model: &PublishedModel, value: &MetadataAnnotationValue) -> Value {
    json!({
        "redefines": target_json(model, &value.redefined_feature),
        "value": expression_json(model, &value.value),
        "nested": value
            .nested
            .iter()
            .map(|nested| annotation_value_json(model, nested))
            .collect::<Vec<_>>(),
    })
}

fn connector_json(model: &PublishedModel, connector: &PublishedConnector) -> Value {
    json!({
        "token": token(model, connector.identity),
        "qualified_name": model.qualified_name(connector.identity),
        "kind": connector.kind.as_str(),
        "declared_type": target_json(model, &connector.declared_type),
        "location": location_json(model, &connector.location),
        "ends": connector
            .ends
            .iter()
            .map(|end| {
                let endpoint = match &end.endpoint {
                    ConnectorEndpoint::Feature(target) => json!({
                        "kind": "feature",
                        "target": target_json(model, target),
                    }),
                    ConnectorEndpoint::FeatureChain { root, terminal, authored, path } => json!({
                        "kind": "feature-chain",
                        "root": target_json(model, root),
                        "terminal": target_json(model, terminal),
                        "authored": authored.as_ref(),
                        "path": path.iter().map(|hop| target_json(model, hop)).collect::<Vec<_>>(),
                    }),
                    ConnectorEndpoint::Unconnected => json!({ "kind": "unconnected" }),
                };
                json!({
                    "declaration": end.declaration.and_then(|symbol| token(model, symbol)),
                    "multiplicity": multiplicity_json(end.multiplicity),
                    "endpoint": endpoint,
                })
            })
            .collect::<Vec<_>>(),
    })
}

fn element_json(model: &PublishedModel, element: &ProjectedElement) -> Value {
    let details: &ElementDetails = &element.details;
    let inspection = &details.inspection;
    json!({
        "token": token(model, element.identity),
        "source": "workspace",
        "kind": inspection.kind.as_str(),
        "name": inspection.name.as_deref(),
        "short_name": inspection.short_name.as_deref(),
        "qualified_name": model.qualified_name(element.identity),
        "owner": details.owner.as_ref().and_then(|owner| token(model, owner.identity)),
        "location": location_json(model, &inspection.location),
        "membership": {
            "kind": membership_kind(inspection.membership.kind),
            "visibility": visibility(inspection.membership.visibility),
            "provenance": visibility_provenance(inspection.membership.provenance),
        },
        "modifiers": inspection
            .modifiers
            .iter()
            .map(|modifier| modifier.as_str())
            .collect::<Vec<_>>(),
        "multiplicity": multiplicity_json(inspection.multiplicity),
        "direction": inspection.direction.map(direction),
        "documentation": inspection
            .documentation
            .iter()
            .map(|entry| json!({
                "form": annotation_form(entry.form),
                "text": model.text(entry.text).unwrap_or_default(),
            }))
            .collect::<Vec<_>>(),
        "relationships": {
            "typing": family_json(model, &details.typing),
            "specialization": family_json(model, &details.specialization),
            "subsetting": family_json(model, &details.subsetting),
            "redefinition": family_json(model, &details.redefinition),
            "effective_types": details
                .effective_typing
                .types
                .iter()
                .map(|entry| json!({
                    "token": token(model, entry.element.identity),
                    "qualified_name": model.qualified_name(entry.element.identity),
                    "provenance": provenance(entry.provenance),
                }))
                .collect::<Vec<_>>(),
            "metadata": symbol_tokens(model, details.metadata.iter().map(|entry| entry.identity)),
            "incoming": connected_json(model, &details.incoming),
            "outgoing": connected_json(model, &details.outgoing),
        },
        "expression": expression_json(model, &element.expression),
        "metadata_annotations": element
            .metadata_annotations
            .iter()
            .map(|annotation| annotation_json(model, annotation))
            .collect::<Vec<_>>(),
    })
}

fn membership_kind(kind: sysml_query::resolved_slice::MembershipKind) -> &'static str {
    match kind {
        sysml_query::resolved_slice::MembershipKind::Owning => "owning",
        sysml_query::resolved_slice::MembershipKind::Feature => "feature",
        sysml_query::resolved_slice::MembershipKind::Import => "import",
        sysml_query::resolved_slice::MembershipKind::Alias => "alias",
    }
}

fn visibility(visibility: sysml_query::resolved_slice::Visibility) -> &'static str {
    match visibility {
        sysml_query::resolved_slice::Visibility::Public => "public",
        sysml_query::resolved_slice::Visibility::Private => "private",
        sysml_query::resolved_slice::Visibility::Protected => "protected",
    }
}

fn visibility_provenance(
    provenance: sysml_query::resolved_slice::VisibilityProvenance,
) -> &'static str {
    match provenance {
        sysml_query::resolved_slice::VisibilityProvenance::Authored => "authored",
        sysml_query::resolved_slice::VisibilityProvenance::Default => "default",
    }
}

fn direction(direction: sysml_query::resolved_slice::FeatureDirection) -> &'static str {
    match direction {
        sysml_query::resolved_slice::FeatureDirection::In => "in",
        sysml_query::resolved_slice::FeatureDirection::Out => "out",
        sysml_query::resolved_slice::FeatureDirection::InOut => "inout",
    }
}

fn annotation_form(form: sysml_query::resolved_slice::AnnotationForm) -> &'static str {
    match form {
        sysml_query::resolved_slice::AnnotationForm::Documentation => "doc",
        sysml_query::resolved_slice::AnnotationForm::Comment => "comment",
        sysml_query::resolved_slice::AnnotationForm::TextualRepresentation => "rep",
    }
}
