use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spec42_generator_protocol::{
    DiagramCompartment, DiagramCompartmentKind, DiagramCompartmentProvenance, DiagramEdge,
    DiagramEdgeKind, DiagramElement, DiagramEndpointOccurrence, DiagramIncompleteReason,
    DiagramNotationRole, DiagramOccurrenceIdentity, DiagramRelationship,
    DiagramRelationshipEndpoint, DiagramRelationshipTarget, DiagramScene, DiagramSemanticReference,
    DiagramSourceDomain, DiagramViewKind, DiagramViewMetadata, DiagramViewProjection,
    DiagramViewSummary, ElementIdentity, ProjectionCompleteness, ProjectionFeature,
    SequenceEndpoint, SequenceMessage, SequenceOrder, SequenceScene, SourceReference,
    StateMachineIdentity, StateMachineSummary, StateTransitionEdge, StateTransitionNode,
    StateTransitionNodeKind, StateTransitionScene, StateTransitionViewProjection,
    StateTransitionViewSummary, TransitionTrigger,
};
use spec42_generator_protocol::{Metaclass, RelationshipKind as ApiRelationshipKind};
use sysml_query::resolved_slice::{
    AnnotationForm, ElementKind, ElementModifier, ElementSource, EvaluatedScalar,
    MultiplicityBound, MultiplicityFacts, QueryAnswer, QueryOutcome, RelationshipProvenance,
    RelationshipTarget, SymbolEntry, SymbolId, SymbolToken,
};
use thiserror::Error;

pub use spec42_generator_protocol::SEMANTIC_API_VERSION as GENERATOR_SEMANTIC_API_VERSION;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryLimits {
    pub max_results: usize,
}

impl Default for QueryLimits {
    fn default() -> Self {
        Self {
            max_results: 50_000,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRange {
    pub start_line: u32,
    pub start_character: u32,
    pub end_line: u32,
    pub end_character: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementSummary {
    pub handle: String,
    pub semantic_id: String,
    pub metaclass: Metaclass,
    pub name: Option<String>,
    pub qualified_name: String,
    pub library_element: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MultiplicitySummary {
    pub lower: Option<String>,
    pub upper: Option<String>,
    pub ordered: bool,
    pub unique: Option<bool>,
    pub implied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ElementDetail {
    pub summary: ElementSummary,
    pub owner: Option<ElementSummary>,
    pub declared_name: Option<String>,
    pub effective_name: Option<String>,
    pub source_uri: String,
    pub source_range: SourceRange,
    pub definition: bool,
    pub documentation: Option<String>,
    pub short_name: Option<String>,
    pub direction: Option<String>,
    pub derived: bool,
    pub constant: bool,
    pub abstract_: bool,
    pub variation: bool,
    pub individual: bool,
    pub conjugated: bool,
    pub composite: Option<bool>,
    pub reference: Option<bool>,
    pub end: bool,
    pub ordered: Option<bool>,
    pub unique: Option<bool>,
    pub multiplicity: Option<MultiplicitySummary>,
    pub evaluated_value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationshipSummary {
    pub kind: ApiRelationshipKind,
    pub source: ElementSummary,
    pub target: ElementSummary,
    pub implied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RequirementUsageTypingSummary {
    Resolved {
        definition: ElementSummary,
        provenance: TypingProvenanceSummary,
    },
    RecoveredResolved {
        definition: ElementSummary,
        provenance: TypingProvenanceSummary,
    },
    RecoveredMissing,
    RecoveredUnresolved,
    RecoveredAmbiguous {
        candidates: Vec<ElementSummary>,
    },
    RecoveredUnsupported,
    Missing,
    Unresolved,
    Ambiguous {
        candidates: Vec<ElementSummary>,
    },
    Unsupported,
    Recovery,
    Incomplete,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SatisfyEndpointSummary {
    Resolved(ElementSummary),
    Ambiguous(Vec<ElementSummary>),
    Unresolved,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatisfyPolaritySummary {
    Satisfied,
    NotSatisfied,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SatisfyRelationshipSummary {
    pub semantic_id: String,
    pub requirement: SatisfyEndpointSummary,
    pub satisfying_element: SatisfyEndpointSummary,
    pub polarity: SatisfyPolaritySummary,
    pub provenance: TypingProvenanceSummary,
    pub recovered: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VerificationRequirementSummary {
    Resolved(ElementSummary),
    Ambiguous(Vec<ElementSummary>),
    Unresolved,
    Unsupported,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VerificationOutcomeSummary {
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequirementVerificationSummary {
    pub semantic_id: String,
    pub verification_case: ElementSummary,
    pub requirement: VerificationRequirementSummary,
    pub provenance: TypingProvenanceSummary,
    pub outcome: VerificationOutcomeSummary,
    pub recovered: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypingProvenanceSummary {
    Authored,
    Implied,
}

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ModelQueryError {
    #[error("unknown or expired element handle `{0}`")]
    UnknownHandle(String),
    #[error("query returned {actual} elements; the configured limit is {limit}")]
    ResultLimit { actual: usize, limit: usize },
    #[error("immutable semantic query is unsupported: {0}")]
    Unsupported(String),
    #[error("immutable semantic query is unresolved: {0}")]
    Unresolved(String),
    #[error("immutable semantic query is ambiguous: {0}")]
    Ambiguous(String),
    #[error("immutable semantic publication is incomplete")]
    Incomplete,
}

#[derive(Debug, Clone)]
struct RegisteredElement {
    entry: SymbolEntry,
    source: ElementSource,
}

/// Generator adapter over one coherent immutable semantic publication.
pub struct GeneratorModelView {
    model: Arc<sysml_query::resolved_slice::PublishedModel>,
    model_digest: String,
    spec42_version: String,
    query_limits: QueryLimits,
    completeness: sysml_query::resolved_slice::PublicationCompleteness,
    by_identity: HashMap<SymbolId, RegisteredElement>,
    handles: Mutex<HashMap<String, SymbolId>>,
}

impl std::fmt::Debug for GeneratorModelView {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GeneratorModelView")
            .field("model_digest", &self.model_digest)
            .field("query_limits", &self.query_limits)
            .finish_non_exhaustive()
    }
}

impl GeneratorModelView {
    pub fn new(
        model: Arc<sysml_query::resolved_slice::PublishedModel>,
        model_digest: impl Into<String>,
        spec42_version: impl Into<String>,
        query_limits: QueryLimits,
    ) -> Result<Self, ModelQueryError> {
        let completeness = model.publication().completeness();
        let mut by_identity = HashMap::new();
        let QueryAnswer::Resolved(elements) = model.inspection().all_elements().answer else {
            return Err(ModelQueryError::Incomplete);
        };
        for element in elements {
            by_identity.insert(
                element.entry.identity,
                RegisteredElement {
                    entry: element.entry,
                    source: element.source,
                },
            );
        }
        Ok(Self {
            model,
            model_digest: model_digest.into(),
            spec42_version: spec42_version.into(),
            query_limits,
            completeness,
            by_identity,
            handles: Mutex::new(HashMap::new()),
        })
    }

    /// The exact completeness of the publication admitted to this view.
    pub fn publication_completeness(&self) -> sysml_query::resolved_slice::PublicationCompleteness {
        self.completeness
    }

    pub fn model_digest(&self) -> String {
        self.model_digest.clone()
    }
    pub fn spec42_version(&self) -> &str {
        &self.spec42_version
    }
    pub fn semantic_api_version(&self) -> &'static str {
        GENERATOR_SEMANTIC_API_VERSION
    }

    pub fn roots(&self) -> Result<Vec<ElementSummary>, ModelQueryError> {
        self.summaries(
            self.by_identity
                .values()
                .filter(|value| {
                    value.source == ElementSource::Workspace && value.entry.owner.is_none()
                })
                .map(|value| value.entry.identity),
        )
    }

    pub fn find(&self, metaclass: Option<&str>) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let requested = metaclass.map(Metaclass::parse);
        if requested.as_ref().is_some_and(Metaclass::is_unrecognized) {
            return Err(ModelQueryError::Unsupported(format!(
                "unknown metaclass `{}`",
                metaclass.unwrap_or_default()
            )));
        }
        self.summaries(
            self.by_identity
                .values()
                .filter(|value| {
                    value.source == ElementSource::Workspace
                        && requested
                            .as_ref()
                            .is_none_or(|kind| api_metaclass(value.entry.kind) == *kind)
                })
                .map(|value| value.entry.identity),
        )
    }

    pub fn children(&self, handle: &str) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let parent = self.resolve_handle(handle)?;
        self.summaries(
            self.by_identity
                .values()
                .filter(|value| value.entry.owner == Some(parent))
                .map(|value| value.entry.identity),
        )
    }

    pub fn element(&self, handle: &str) -> Result<ElementDetail, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let inspection = outcome(
            self.model.inspection().inspect(identity),
            "element inspection",
        )?;
        let summary = self.summary(identity)?;
        let owner = inspection
            .owner
            .as_ref()
            .map(|owner| self.summary(owner))
            .transpose()?;
        let has = |modifier| inspection.modifiers.contains(&modifier);
        let multiplicity = match inspection.multiplicity {
            MultiplicityFacts::Absent => None,
            MultiplicityFacts::Declared {
                lower,
                upper,
                ordered,
                nonunique,
            } => Some(MultiplicitySummary {
                lower: bound(lower)?,
                upper: bound(upper)?,
                ordered,
                unique: Some(!nonunique),
                implied: false,
            }),
        };
        Ok(ElementDetail {
            summary,
            owner,
            declared_name: inspection.name.as_deref().map(str::to_owned),
            // The immutable inspection contract does not yet publish an effective-name fact.
            // Absence is honest; copying the authored name here would turn it into competing
            // semantic truth for aliases and other normalized names.
            effective_name: None,
            source_uri: self
                .model
                .document_identity(inspection.location.document)
                .unwrap_or_default()
                .to_owned(),
            source_range: source_range(inspection.declaration_range),
            definition: inspection.kind.as_str().ends_with("Definition"),
            documentation: inspection
                .documentation
                .iter()
                .find(|doc| doc.form == AnnotationForm::Documentation)
                .map(|doc| self.model.text(doc.text).unwrap_or_default().to_owned()),
            short_name: inspection.short_name.as_deref().map(str::to_owned),
            direction: inspection.direction.map(|direction| {
                match direction {
                    sysml_query::resolved_slice::FeatureDirection::In => "in",
                    sysml_query::resolved_slice::FeatureDirection::Out => "out",
                    sysml_query::resolved_slice::FeatureDirection::InOut => "inout",
                }
                .to_owned()
            }),
            derived: has(ElementModifier::Derived),
            constant: has(ElementModifier::Constant),
            abstract_: has(ElementModifier::Abstract),
            variation: has(ElementModifier::Variation),
            individual: has(ElementModifier::Individual),
            conjugated: false,
            composite: has(ElementModifier::Composite).then_some(true),
            reference: has(ElementModifier::Reference).then_some(true),
            end: has(ElementModifier::End),
            ordered: multiplicity
                .as_ref()
                .map(|value| value.ordered)
                .or_else(|| has(ElementModifier::Ordered).then_some(true)),
            unique: multiplicity.as_ref().and_then(|value| value.unique),
            multiplicity,
            evaluated_value: inspection.evaluation.value().map(scalar),
        })
    }

    pub fn typed_by(&self, handle: &str) -> Result<Option<ElementSummary>, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let types = outcome(self.model.types().direct_types(identity), "direct typing")?;
        match types.as_ref() {
            [] => Ok(None),
            [one] => self.summary(one.symbol).map(Some),
            _ => Err(ModelQueryError::Ambiguous(format!(
                "element `{}` has {} direct types",
                self.token(identity),
                types.len()
            ))),
        }
    }

    pub fn requirement_usage_typing(
        &self,
        handle: &str,
    ) -> Result<RequirementUsageTypingSummary, ModelQueryError> {
        use sysml_query::resolved_slice::{QueryAnswer, RequirementUsageTyping as Owned};
        use RequirementUsageTypingSummary as Wire;
        let identity = self.resolve_handle(handle)?;
        let outcome = self.model.types().requirement_usage_typing(identity);
        let recovered = !outcome.completeness.is_complete();
        Ok(match outcome.answer {
            QueryAnswer::Resolved(Owned::Missing) if recovered => Wire::RecoveredMissing,
            QueryAnswer::Resolved(Owned::Missing) => Wire::Missing,
            QueryAnswer::Resolved(Owned::Resolved(reference)) if recovered => {
                Wire::RecoveredResolved {
                    definition: self.summary(reference.symbol)?,
                    provenance: match reference.provenance {
                        RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                        RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                    },
                }
            }
            QueryAnswer::Resolved(Owned::Resolved(reference)) => Wire::Resolved {
                definition: self.summary(reference.symbol)?,
                provenance: match reference.provenance {
                    RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                    RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                },
            },
            QueryAnswer::Resolved(Owned::Ambiguous(values)) if recovered => {
                Wire::RecoveredAmbiguous {
                    candidates: values
                        .iter()
                        .map(|value| self.summary(value))
                        .collect::<Result<Vec<_>, _>>()?,
                }
            }
            QueryAnswer::Resolved(Owned::Ambiguous(values)) => Wire::Ambiguous {
                candidates: values
                    .iter()
                    .map(|value| self.summary(value))
                    .collect::<Result<Vec<_>, _>>()?,
            },
            QueryAnswer::Resolved(Owned::Unresolved) if recovered => Wire::RecoveredUnresolved,
            QueryAnswer::Resolved(Owned::Unresolved) | QueryAnswer::Unresolved => Wire::Unresolved,
            QueryAnswer::Resolved(Owned::Unsupported) if recovered => Wire::RecoveredUnsupported,
            QueryAnswer::Resolved(Owned::Unsupported) | QueryAnswer::Unsupported => {
                Wire::Unsupported
            }
            QueryAnswer::Recovery => Wire::Recovery,
            QueryAnswer::Ambiguous(values) => Wire::Ambiguous {
                candidates: values
                    .iter()
                    .flat_map(|value| match value {
                        Owned::Resolved(reference) => self.summary(reference.symbol).ok(),
                        _ => None,
                    })
                    .collect(),
            },
            QueryAnswer::Incomplete => Wire::Incomplete,
        })
    }

    pub fn relationships(&self, handle: &str) -> Result<Vec<RelationshipSummary>, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let inspection = outcome(
            self.model.inspection().inspect(identity),
            "element relationships",
        )?;
        let source = self.summary(identity)?;
        let mut values = Vec::new();
        for relationship in &inspection.relationships {
            let target = match &relationship.target {
                RelationshipTarget::Resolved(target) => self.summary(target)?,
                RelationshipTarget::Ambiguous(_) => {
                    return Err(ModelQueryError::Ambiguous(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        self.token(identity)
                    )))
                }
                RelationshipTarget::Unresolved => {
                    return Err(ModelQueryError::Unresolved(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        self.token(identity)
                    )))
                }
                RelationshipTarget::Unsupported => {
                    return Err(ModelQueryError::Unsupported(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        self.token(identity)
                    )))
                }
            };
            values.push(RelationshipSummary {
                kind: ApiRelationshipKind::parse(generator_relationship_kind(relationship.kind)),
                source: source.clone(),
                target,
                implied: relationship.provenance == RelationshipProvenance::Implied,
            });
        }
        values.sort_by(|a, b| {
            a.kind
                .as_str()
                .cmp(b.kind.as_str())
                .then_with(|| summary_order(&a.target, &b.target))
        });
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    pub fn satisfy_relationships(
        &self,
    ) -> Result<Vec<SatisfyRelationshipSummary>, ModelQueryError> {
        use sysml_query::resolved_slice::{
            QueryAnswer, SatisfyEndpoint as OwnedEndpoint, SatisfyPolarity as OwnedPolarity,
        };
        // `Recovered` and `UnsupportedWith` both carry every settled relationship of a
        // publication that is not complete; `recovered` reports that incompleteness to the
        // guest, exactly as the generic `outcome` helper below admits both for every other
        // query. Only a query with no values at all is an error.
        let query = self.model.inspection().satisfy_relationships();
        let recovered = !query.completeness.is_complete();
        let relationships = match query.answer {
            QueryAnswer::Resolved(values) => values,
            QueryAnswer::Unsupported => {
                return Err(ModelQueryError::Unsupported("satisfy relationships".into()))
            }
            QueryAnswer::Unresolved => {
                return Err(ModelQueryError::Unresolved("satisfy relationships".into()))
            }
            QueryAnswer::Ambiguous(_) => {
                return Err(ModelQueryError::Ambiguous("satisfy relationships".into()))
            }
            QueryAnswer::Recovery => {
                return Err(ModelQueryError::Unresolved(
                    "satisfy relationships are in parser recovery".into(),
                ))
            }
            QueryAnswer::Incomplete => return Err(ModelQueryError::Incomplete),
        };
        let endpoint = |value: &OwnedEndpoint| -> Result<SatisfyEndpointSummary, ModelQueryError> {
            Ok(match value {
                OwnedEndpoint::Resolved(identity) => {
                    SatisfyEndpointSummary::Resolved(self.summary(identity)?)
                }
                OwnedEndpoint::Ambiguous(values) => SatisfyEndpointSummary::Ambiguous(
                    values
                        .iter()
                        .map(|value| self.summary(value))
                        .collect::<Result<Vec<_>, _>>()?,
                ),
                OwnedEndpoint::Unresolved => SatisfyEndpointSummary::Unresolved,
                OwnedEndpoint::Unsupported => SatisfyEndpointSummary::Unsupported,
            })
        };
        let values = relationships
            .iter()
            .map(|relationship| {
                Ok(SatisfyRelationshipSummary {
                    semantic_id: self.token(relationship.identity),
                    requirement: endpoint(&relationship.requirement)?,
                    satisfying_element: endpoint(&relationship.satisfying_element)?,
                    polarity: match relationship.polarity {
                        OwnedPolarity::Satisfied => SatisfyPolaritySummary::Satisfied,
                        OwnedPolarity::NotSatisfied => SatisfyPolaritySummary::NotSatisfied,
                    },
                    provenance: match relationship.provenance {
                        RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                        RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                    },
                    recovered,
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    pub fn requirement_verifications(
        &self,
    ) -> Result<Vec<RequirementVerificationSummary>, ModelQueryError> {
        use sysml_query::resolved_slice::{
            QueryAnswer, VerificationOutcome as OwnedOutcome,
            VerificationRequirement as OwnedRequirement,
        };
        let query = self.model.inspection().requirement_verifications();
        let recovered = !query.completeness.is_complete();
        let relationships = match query.answer {
            QueryAnswer::Resolved(values) => values,
            QueryAnswer::Unsupported => {
                return Err(ModelQueryError::Unsupported(
                    "requirement verifications".into(),
                ))
            }
            QueryAnswer::Unresolved | QueryAnswer::Recovery => {
                return Err(ModelQueryError::Unresolved(
                    "requirement verifications".into(),
                ))
            }
            QueryAnswer::Ambiguous(_) => {
                return Err(ModelQueryError::Ambiguous(
                    "requirement verifications".into(),
                ))
            }
            QueryAnswer::Incomplete => return Err(ModelQueryError::Incomplete),
        };
        let endpoint =
            |value: &OwnedRequirement| -> Result<VerificationRequirementSummary, ModelQueryError> {
                Ok(match value {
                    OwnedRequirement::Resolved(identity) => {
                        VerificationRequirementSummary::Resolved(self.summary(identity)?)
                    }
                    OwnedRequirement::Ambiguous(values) => {
                        VerificationRequirementSummary::Ambiguous(
                            values
                                .iter()
                                .map(|v| self.summary(v))
                                .collect::<Result<Vec<_>, _>>()?,
                        )
                    }
                    OwnedRequirement::Unresolved => VerificationRequirementSummary::Unresolved,
                    OwnedRequirement::Unsupported => VerificationRequirementSummary::Unsupported,
                })
            };
        let values = relationships
            .iter()
            .map(|value| {
                Ok(RequirementVerificationSummary {
                    semantic_id: self.token(value.identity),
                    verification_case: self.summary(value.verification_case)?,
                    requirement: endpoint(&value.requirement)?,
                    provenance: match value.provenance {
                        RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                        RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                    },
                    outcome: match value.outcome {
                        OwnedOutcome::Unsupported => VerificationOutcomeSummary::Unsupported,
                    },
                    recovered,
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    pub fn effective_features(&self, handle: &str) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let features = outcome(
            self.model.inspection().effective_features(identity),
            "effective features",
        )?;
        let values = features
            .iter()
            .map(|feature| self.summary(feature.identity))
            .collect::<Result<Vec<_>, _>>()?;
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    /// Catalogs authored standard view usages from the immutable typing facts. Presentation
    /// clients use this to offer only diagram kinds actually authored in a document.
    pub fn diagram_views(&self) -> Result<Vec<DiagramViewSummary>, ModelQueryError> {
        let catalog = outcome(self.model.diagrams().catalog(), "diagram view catalog")?;
        let mut values = catalog
            .iter()
            .map(|entry| {
                let handle = handle_from_semantic_id(&self.token(entry.semantic_id));
                self.handles
                    .lock()
                    .expect("generator handle index poisoned")
                    .insert(handle.clone(), entry.semantic_id);
                Ok(DiagramViewSummary {
                    handle,
                    kind: diagram_kind(entry.kind),
                    reference: self.diagram_reference(entry.semantic_id)?,
                    name: self
                        .model
                        .diagrams()
                        .view_name(entry.semantic_id)
                        .unwrap_or_default()
                        .to_owned(),
                    source: source_reference(
                        &entry.source,
                        self.model
                            .document_identity(entry.source.document)
                            .unwrap_or_default(),
                    ),
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        values.sort_by(|a, b| {
            a.name
                .cmp(&b.name)
                .then_with(|| a.source.uri.cmp(&b.source.uri))
        });
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    pub fn diagram_view(&self, handle: &str) -> Result<DiagramViewProjection, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let projection = outcome(self.model.diagrams().view(identity), "diagram view")?;
        let view = DiagramViewSummary {
            handle: handle.to_owned(),
            reference: self.diagram_reference(projection.view.semantic_id)?,
            kind: diagram_kind(projection.view.kind),
            name: self
                .model
                .diagrams()
                .view_name(projection.view.semantic_id)
                .unwrap_or_default()
                .to_owned(),
            source: source_reference(
                &projection.view.source,
                self.model
                    .document_identity(projection.view.source.document)
                    .unwrap_or_default(),
            ),
        };
        let elements = projection
            .elements
            .iter()
            .map(|element| Ok(DiagramElement {
                occurrence: self.diagram_occurrence(&element.occurrence_id)?,
                reference: self
                    .diagram_reference(element.semantic_id)
                    .expect("published diagram element"),
                metaclass: api_metaclass(element.kind),
                notation_role: diagram_notation_role(element.kind),
                name: element.name.as_deref().map(str::to_owned),
                typing: match &element.typing {
                    sysml_query::resolved_slice::DiagramElementTyping::Absent => spec42_generator_protocol::DiagramElementTyping::Absent,
                    sysml_query::resolved_slice::DiagramElementTyping::Resolved(targets) => spec42_generator_protocol::DiagramElementTyping::Resolved(targets.iter().map(|target| {
                        let target_entry = self.by_identity.get(target).ok_or_else(|| ModelQueryError::Unresolved("diagram typing target is absent from the publication".into()))?;
                        Ok(spec42_generator_protocol::DiagramElementType { reference: self.diagram_reference(*target)?, label: display_label(&self.model, &target_entry.entry) })
                    }).collect::<Result<Vec<_>, ModelQueryError>>()?),
                    sysml_query::resolved_slice::DiagramElementTyping::Partial(targets) => spec42_generator_protocol::DiagramElementTyping::Partial(targets.iter().map(|target| {
                        let target_entry = self.by_identity.get(target).ok_or_else(|| ModelQueryError::Unresolved("diagram typing target is absent from the publication".into()))?;
                        Ok(spec42_generator_protocol::DiagramElementType { reference: self.diagram_reference(*target)?, label: display_label(&self.model, &target_entry.entry) })
                    }).collect::<Result<Vec<_>, ModelQueryError>>()?),
                    sysml_query::resolved_slice::DiagramElementTyping::Ambiguous(targets) => spec42_generator_protocol::DiagramElementTyping::Ambiguous(targets.iter().map(|target| self.diagram_reference(*target)).collect::<Result<Vec<_>, _>>()?),
                    sysml_query::resolved_slice::DiagramElementTyping::Unresolved => spec42_generator_protocol::DiagramElementTyping::Unresolved,
                    sysml_query::resolved_slice::DiagramElementTyping::Unsupported => spec42_generator_protocol::DiagramElementTyping::Unsupported,
                    sysml_query::resolved_slice::DiagramElementTyping::Recovery => spec42_generator_protocol::DiagramElementTyping::Recovery,
                    sysml_query::resolved_slice::DiagramElementTyping::Incomplete => spec42_generator_protocol::DiagramElementTyping::Incomplete,
                },
                owner: element
                    .owner
                    .as_ref()
                    .map(|owner| self.diagram_occurrence(owner))
                    .transpose()?,
                source: source_reference(&element.source, self.model.document_identity(element.source.document).unwrap_or_default()),
                compartments: element.compartments.iter().map(|compartment| {
                    Ok(DiagramCompartment {
                        kind: match compartment.kind {
                            sysml_query::resolved_slice::DiagramCompartmentKind::Attributes => DiagramCompartmentKind::Attributes,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Parts => DiagramCompartmentKind::Parts,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Ports => DiagramCompartmentKind::Ports,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Items => DiagramCompartmentKind::Items,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Constraints => DiagramCompartmentKind::Constraints,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Requirements => DiagramCompartmentKind::Requirements,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Actions => DiagramCompartmentKind::Actions,
                            sysml_query::resolved_slice::DiagramCompartmentKind::States => DiagramCompartmentKind::States,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Calculations => DiagramCompartmentKind::Calculations,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Connections => DiagramCompartmentKind::Connections,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Interfaces => DiagramCompartmentKind::Interfaces,
                            sysml_query::resolved_slice::DiagramCompartmentKind::Occurrences => DiagramCompartmentKind::Occurrences,
                        },
                        provenance: match compartment.provenance {
                            sysml_query::resolved_slice::DiagramCompartmentProvenance::Direct => DiagramCompartmentProvenance::Direct,
                            sysml_query::resolved_slice::DiagramCompartmentProvenance::Inherited => DiagramCompartmentProvenance::Inherited,
                        },
                        members: compartment.members.iter().map(|member| self.diagram_occurrence(member)).collect::<Result<Vec<_>, _>>()?,
                    })
                }).collect::<Result<Vec<_>, ModelQueryError>>()?,
            }))
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        let relationships = projection
            .relationships
            .iter()
            .enumerate()
            .map(|(ordinal, relationship)| {
                let kind = spec42_generator_protocol::RelationshipKind::parse(
                    generator_relationship_kind(relationship.kind.name()),
                );
                Ok(DiagramRelationship {
                    reference: self.diagram_relationship_reference(
                        relationship.source_semantic_id,
                        kind.clone(),
                        ordinal,
                    )?,
                    source_element: self.diagram_reference(relationship.source_semantic_id)?,
                    source_occurrence: self.diagram_occurrence(&relationship.source)?,
                    kind,
                    target: match &relationship.target {
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Resolved(
                            target,
                        ) => DiagramRelationshipTarget::Resolved(
                            self.diagram_relationship_endpoint(target)?,
                        ),
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Ambiguous(
                            values,
                        ) => DiagramRelationshipTarget::Ambiguous(
                            values
                                .iter()
                                .map(|value| self.diagram_relationship_endpoint(value))
                                .collect::<Result<Vec<_>, _>>()?,
                        ),
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Unresolved => {
                            DiagramRelationshipTarget::Unresolved
                        }
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Unsupported => {
                            DiagramRelationshipTarget::Unsupported
                        }
                    },
                    provenance: relationship_provenance(relationship.provenance),
                    source: relationship.source_location.as_ref().map(|location| {
                        source_reference(
                            location,
                            self.model
                                .document_identity(location.document)
                                .unwrap_or_default(),
                        )
                    }),
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        let edges = projection
            .edges
            .iter()
            .enumerate()
            .map(|(ordinal, edge)| {
                let kind = match &edge.kind {
                    sysml_query::resolved_slice::DiagramEdgeKind::Containment => {
                        DiagramEdgeKind::Containment
                    }
                    sysml_query::resolved_slice::DiagramEdgeKind::Connector => {
                        DiagramEdgeKind::Connector
                    }
                    sysml_query::resolved_slice::DiagramEdgeKind::Flow => DiagramEdgeKind::Flow,
                    sysml_query::resolved_slice::DiagramEdgeKind::Succession => {
                        DiagramEdgeKind::Succession
                    }
                    sysml_query::resolved_slice::DiagramEdgeKind::Transition => {
                        DiagramEdgeKind::Transition
                    }
                    sysml_query::resolved_slice::DiagramEdgeKind::InitialState => {
                        DiagramEdgeKind::InitialState
                    }
                    sysml_query::resolved_slice::DiagramEdgeKind::Relationship(kind) => {
                        DiagramEdgeKind::Relationship(
                            spec42_generator_protocol::RelationshipKind::parse(kind),
                        )
                    }
                };
                let reference_kind = match &kind {
                    DiagramEdgeKind::Containment => {
                        spec42_generator_protocol::RelationshipKind::Containment
                    }
                    DiagramEdgeKind::Connector => {
                        spec42_generator_protocol::RelationshipKind::Connection
                    }
                    DiagramEdgeKind::Flow => spec42_generator_protocol::RelationshipKind::Flow,
                    DiagramEdgeKind::Succession => {
                        spec42_generator_protocol::RelationshipKind::Succession
                    }
                    DiagramEdgeKind::Transition => {
                        spec42_generator_protocol::RelationshipKind::Transition
                    }
                    DiagramEdgeKind::InitialState => {
                        spec42_generator_protocol::RelationshipKind::InitialState
                    }
                    DiagramEdgeKind::Relationship(kind) => kind.clone(),
                };
                let origin_occurrence = projection
                    .elements
                    .get(edge.origin as usize)
                    .map(|element| &element.occurrence_id)
                    .ok_or(ModelQueryError::Incomplete)?;
                Ok(DiagramEdge {
                    reference: self.diagram_relationship_reference(
                        edge.source_semantic_id,
                        reference_kind,
                        ordinal,
                    )?,
                    source_element: self.diagram_reference(edge.source_semantic_id)?,
                    target_element: self.diagram_reference(edge.target_semantic_id)?,
                    source_occurrence: self.diagram_occurrence(&edge.source)?,
                    target_occurrence: self.diagram_occurrence(&edge.target)?,
                    origin_occurrence: self.diagram_occurrence(origin_occurrence)?,
                    kind,
                    provenance: relationship_provenance(edge.provenance),
                    source: edge.source_location.as_ref().map(|location| {
                        source_reference(
                            location,
                            self.model
                                .document_identity(location.document)
                                .unwrap_or_default(),
                        )
                    }),
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        let roots = projection
            .exposed_roots
            .iter()
            .cloned()
            .map(|root| {
                let occurrence = self.model.diagrams().root_occurrence(root).ok_or_else(|| {
                    ModelQueryError::Unresolved(
                        "exposed diagram root is absent from the publication".into(),
                    )
                })?;
                self.diagram_occurrence(&occurrence)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let reasons = projection
            .incomplete_reasons
            .iter()
            .cloned()
            .map(|reason| self.diagram_incomplete_reason(reason))
            .collect::<Result<Vec<_>, _>>()?;
        let scene_feature =
            |feature: &sysml_query::resolved_slice::DiagramTransitionFeature| match feature {
                sysml_query::resolved_slice::DiagramTransitionFeature::Absent => {
                    ProjectionFeature::Absent
                }
                sysml_query::resolved_slice::DiagramTransitionFeature::Resolved {
                    label,
                    source,
                    ..
                } => ProjectionFeature::Supported {
                    label: label.to_string(),
                    source: source_reference(
                        source,
                        self.model
                            .document_identity(source.document)
                            .unwrap_or_default(),
                    ),
                },
                sysml_query::resolved_slice::DiagramTransitionFeature::Unresolved => {
                    ProjectionFeature::Unresolved
                }
                sysml_query::resolved_slice::DiagramTransitionFeature::Ambiguous => {
                    ProjectionFeature::Ambiguous
                }
                sysml_query::resolved_slice::DiagramTransitionFeature::Unsupported => {
                    ProjectionFeature::Unsupported {
                        reason: unsupported("scene-feature", "the semantic feature is unsupported"),
                    }
                }
            };
        let scene = match &projection.scene {
            sysml_query::resolved_slice::DiagramScene::General => DiagramScene::General,
            sysml_query::resolved_slice::DiagramScene::Interconnection => {
                DiagramScene::Interconnection
            }
            sysml_query::resolved_slice::DiagramScene::ActionFlow => DiagramScene::ActionFlow,
            sysml_query::resolved_slice::DiagramScene::Sequence(sequence) => {
                let endpoint = |value: &sysml_query::resolved_slice::DiagramSequenceEndpoint| {
                    Ok(match value {
                        sysml_query::resolved_slice::DiagramSequenceEndpoint::Resolved(value) => {
                            SequenceEndpoint::Resolved(self.diagram_occurrence(value)?)
                        }
                        sysml_query::resolved_slice::DiagramSequenceEndpoint::Ambiguous => {
                            SequenceEndpoint::Ambiguous
                        }
                        sysml_query::resolved_slice::DiagramSequenceEndpoint::Unresolved => {
                            SequenceEndpoint::Unresolved
                        }
                        sysml_query::resolved_slice::DiagramSequenceEndpoint::Unsupported => {
                            SequenceEndpoint::Unsupported
                        }
                        sysml_query::resolved_slice::DiagramSequenceEndpoint::OutsideLifeline => {
                            SequenceEndpoint::OutsideLifeline
                        }
                    })
                };
                DiagramScene::Sequence(SequenceScene {
                    lifelines: sequence
                        .lifelines
                        .iter()
                        .map(|value| self.diagram_occurrence(value))
                        .collect::<Result<Vec<_>, _>>()?,
                    messages: sequence
                        .messages
                        .iter()
                        .map(|message| {
                            let occurrence = projection
                                .elements
                                .get(message.origin as usize)
                                .ok_or(ModelQueryError::Incomplete)?
                                .occurrence_id
                                .clone();
                            Ok(SequenceMessage {
                                occurrence: self.diagram_occurrence(&occurrence)?,
                                label: message.label.as_deref().map(str::to_owned),
                                source: endpoint(&message.source)?,
                                target: endpoint(&message.target)?,
                                order: match message.order {
                                    sysml_query::resolved_slice::DiagramSequenceOrder::Resolved(
                                        order,
                                    ) => SequenceOrder::Resolved(order),
                                    sysml_query::resolved_slice::DiagramSequenceOrder::Cyclic => {
                                        SequenceOrder::Cyclic
                                    }
                                },
                                provenance: relationship_provenance(message.provenance),
                                source_reference: source_reference(
                                    &message.source_location,
                                    self.model
                                        .document_identity(message.source_location.document)
                                        .unwrap_or_default(),
                                ),
                            })
                        })
                        .collect::<Result<Vec<_>, ModelQueryError>>()?,
                })
            }
            sysml_query::resolved_slice::DiagramScene::Browser => DiagramScene::Browser,
            sysml_query::resolved_slice::DiagramScene::Grid => DiagramScene::Grid,
            sysml_query::resolved_slice::DiagramScene::Geometry => DiagramScene::Geometry,
            sysml_query::resolved_slice::DiagramScene::StateTransition(state) => {
                let machine = state
                    .machine
                    .as_ref()
                    .map(|identity| {
                        let entry = self.by_identity.get(identity).ok_or_else(|| {
                            ModelQueryError::Unresolved(
                                "state-machine scene frame is absent from publication".into(),
                            )
                        })?;
                        Ok(StateMachineSummary {
                            semantic_id: self.token(identity),
                            label: display_label(&self.model, &entry.entry),
                            source: source_reference(
                                &entry.entry.location,
                                self.model
                                    .document_identity(entry.entry.location.document)
                                    .unwrap_or_default(),
                            ),
                        })
                    })
                    .transpose()?;
                let vertices = state
                    .vertices
                    .iter()
                    .map(|vertex| StateTransitionNode {
                        semantic_id: self.token(vertex.semantic_id),
                        label: self
                            .model
                            .symbol_name(vertex.semantic_id)
                            .unwrap_or_default()
                            .to_owned(),
                        kind: match vertex.kind {
                            sysml_query::resolved_slice::DiagramStateVertexKind::Initial => {
                                StateTransitionNodeKind::Initial
                            }
                            sysml_query::resolved_slice::DiagramStateVertexKind::State => {
                                StateTransitionNodeKind::State
                            }
                            sysml_query::resolved_slice::DiagramStateVertexKind::Final => {
                                StateTransitionNodeKind::Final
                            }
                        },
                        source: source_reference(
                            &vertex.source,
                            self.model
                                .document_identity(vertex.source.document)
                                .unwrap_or_default(),
                        ),
                    })
                    .collect();
                let transitions = state
                    .transitions
                    .iter()
                    .map(|transition| StateTransitionEdge {
                        semantic_id: projection
                            .transition_scene_id(transition)
                            .unwrap_or_default(),
                        label: transition.label.as_deref().map(str::to_owned),
                        source: self.token(transition.source),
                        target: self.token(transition.target),
                        trigger: match &transition.trigger {
                            sysml_query::resolved_slice::DiagramTransitionFeature::Absent => {
                                TransitionTrigger::None
                            }
                            sysml_query::resolved_slice::DiagramTransitionFeature::Resolved {
                                label,
                                target,
                                source,
                            } => TransitionTrigger::Accept {
                                label: label.to_string(),
                                target: Some(ElementIdentity {
                                    semantic_id: self.token(target),
                                    label: label.to_string(),
                                }),
                                source: source_reference(
                                    source,
                                    self.model
                                        .document_identity(source.document)
                                        .unwrap_or_default(),
                                ),
                            },
                            sysml_query::resolved_slice::DiagramTransitionFeature::Unresolved => {
                                TransitionTrigger::Unresolved
                            }
                            sysml_query::resolved_slice::DiagramTransitionFeature::Ambiguous => {
                                TransitionTrigger::Ambiguous
                            }
                            sysml_query::resolved_slice::DiagramTransitionFeature::Unsupported => {
                                TransitionTrigger::Unsupported {
                                    reason: unsupported(
                                        "trigger",
                                        "the transition trigger is unsupported",
                                    ),
                                }
                            }
                        },
                        guard: scene_feature(&transition.guard),
                        effect: scene_feature(&transition.effect),
                        provenance: relationship_provenance(transition.provenance),
                        source_reference: source_reference(
                            &transition.source_location,
                            self.model
                                .document_identity(transition.source_location.document)
                                .unwrap_or_default(),
                        ),
                    })
                    .collect();
                DiagramScene::StateTransition(StateTransitionScene {
                    machine,
                    vertices,
                    transitions,
                })
            }
        };
        let metadata = diagram_metadata(view.kind, &roots, &elements, &relationships, &scene);
        Ok(DiagramViewProjection {
            schema_version: 1,
            model_digest: self.model_digest.clone(),
            view,
            completeness: if reasons.is_empty() {
                ProjectionCompleteness::Complete
            } else {
                ProjectionCompleteness::Incomplete {
                    reasons: reasons
                        .iter()
                        .map(|reason| unsupported("diagram", &format!("{reason:?}")))
                        .collect(),
                }
            },
            incomplete_reasons: reasons,
            exposed_roots: roots,
            elements,
            relationships,
            edges,
            metadata,
            scene,
        })
    }

    /// Catalog authored state-transition views whose type and single exposed machine are resolved.
    pub fn state_transition_views(
        &self,
    ) -> Result<Vec<StateTransitionViewSummary>, ModelQueryError> {
        let standard_view = self.standard_state_transition_view()?;
        let mut values = Vec::new();
        for registered in self.by_identity.values().filter(|value| {
            value.source == ElementSource::Workspace && value.entry.kind == ElementKind::ViewUsage
        }) {
            let types = outcome(
                self.model.types().direct_types(registered.entry.identity),
                "state-transition view typing",
            )?;
            if !types.iter().any(|ty| ty.symbol == standard_view) {
                continue;
            }
            let machine = self.exposed_machine(registered.entry.identity)?;
            values.push(self.view_summary(registered.entry.identity, machine)?);
        }
        values.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    fn standard_state_transition_view(&self) -> Result<SymbolId, ModelQueryError> {
        let matches = self
            .by_identity
            .values()
            .filter(|entry| {
                entry.source == ElementSource::StandardLibrary
                    && entry.entry.kind == ElementKind::ViewDefinition
                    && entry.entry.name.as_deref() == Some("StateTransitionView")
            })
            .map(|entry| entry.entry.identity)
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [identity] => Ok(*identity),
            [] => Err(ModelQueryError::Incomplete),
            _ => Err(ModelQueryError::Ambiguous(
                "standard library contains multiple StateTransitionView definitions".into(),
            )),
        }
    }

    pub fn state_transition_view(
        &self,
        handle: &str,
    ) -> Result<StateTransitionViewProjection, ModelQueryError> {
        let view_id = self.resolve_handle(handle)?;
        let view_entry = self
            .by_identity
            .get(&view_id)
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))?;
        if view_entry.entry.kind != ElementKind::ViewUsage {
            return Err(ModelQueryError::Unsupported(
                "selected element is not a view usage".into(),
            ));
        }
        let machine_id = self.exposed_machine(view_id)?;
        let machine_entry = self.by_identity.get(&machine_id).ok_or_else(|| {
            ModelQueryError::Unresolved("exposed state machine is absent from publication".into())
        })?;
        if machine_entry.entry.kind != ElementKind::StateDefinition {
            return Err(ModelQueryError::Unsupported(
                "state-transition view must expose one state definition".into(),
            ));
        }
        let view = self.view_summary(view_id, machine_id)?;
        let machine_inspection = self.inspection(machine_id, "state machine")?;
        let machine = StateMachineSummary {
            semantic_id: self.token(machine_id),
            label: display_label(&self.model, &machine_entry.entry),
            source: inspection_source(
                &machine_inspection,
                self.model
                    .document_identity(machine_inspection.location.document)
                    .unwrap_or_default(),
            ),
        };
        let children = self
            .by_identity
            .values()
            .filter(|entry| entry.entry.owner.as_ref() == Some(&machine_id))
            .collect::<Vec<_>>();
        let mut nodes = Vec::new();
        let mut transitions = Vec::new();
        for child in &children {
            let inspection = self.inspection(child.entry.identity, "state-machine member")?;
            match child.entry.kind {
                ElementKind::StateUsage | ElementKind::FinalState => {
                    nodes.push(StateTransitionNode {
                        semantic_id: self.token(child.entry.identity),
                        label: display_label(&self.model, &child.entry),
                        kind: if child.entry.kind == ElementKind::FinalState {
                            StateTransitionNodeKind::Final
                        } else {
                            StateTransitionNodeKind::State
                        },
                        source: inspection_source(
                            &inspection,
                            self.model
                                .document_identity(inspection.location.document)
                                .unwrap_or_default(),
                        ),
                    })
                }
                ElementKind::SuccessionAsUsage => {
                    if let Some(target) = resolved_relationship(&inspection, "initialState")? {
                        let initial_id = format!("{}#initial", self.token(child.entry.identity));
                        nodes.push(StateTransitionNode {
                            semantic_id: initial_id.clone(),
                            label: String::new(),
                            kind: StateTransitionNodeKind::Initial,
                            source: inspection_source(
                                &inspection,
                                self.model
                                    .document_identity(inspection.location.document)
                                    .unwrap_or_default(),
                            ),
                        });
                        transitions.push(StateTransitionEdge {
                            semantic_id: format!("{}#edge", self.token(child.entry.identity)),
                            label: None,
                            source: initial_id,
                            target: self.token(target),
                            trigger: TransitionTrigger::None,
                            guard: ProjectionFeature::Absent,
                            effect: ProjectionFeature::Absent,
                            provenance: spec42_generator_protocol::RelationshipProvenance::Authored,
                            source_reference: inspection_source(
                                &inspection,
                                self.model
                                    .document_identity(inspection.location.document)
                                    .unwrap_or_default(),
                            ),
                        });
                    }
                }
                ElementKind::TransitionUsage => {
                    let source = resolved_relationship(&inspection, "transitionSource")?
                        .ok_or_else(|| ModelQueryError::Unresolved("transition source".into()))?;
                    let target = resolved_relationship(&inspection, "transitionTarget")?
                        .ok_or_else(|| ModelQueryError::Unresolved("transition target".into()))?;
                    let trigger = match resolved_relationship(&inspection, "transitionTrigger")? {
                        None => TransitionTrigger::None,
                        Some(trigger) => {
                            let target_entry = self.by_identity.get(&trigger).ok_or_else(|| {
                                ModelQueryError::Unresolved("transition trigger target".into())
                            })?;
                            TransitionTrigger::Accept {
                                label: display_label(&self.model, &target_entry.entry),
                                target: Some(ElementIdentity {
                                    semantic_id: self.token(trigger),
                                    label: display_label(&self.model, &target_entry.entry),
                                }),
                                source: inspection_source(
                                    &inspection,
                                    self.model
                                        .document_identity(inspection.location.document)
                                        .unwrap_or_default(),
                                ),
                            }
                        }
                    };
                    let has_guard = inspection
                        .relationships
                        .iter()
                        .any(|r| r.kind == "transitionGuard");
                    let has_effect = inspection
                        .relationships
                        .iter()
                        .any(|r| r.kind == "transitionEffect");
                    transitions.push(StateTransitionEdge {
                        semantic_id: self.token(child.entry.identity),
                        label: child.entry.name.as_deref().map(str::to_owned),
                        source: self.token(source),
                        target: self.token(target),
                        trigger,
                        guard: if has_guard {
                            ProjectionFeature::Unsupported {
                                reason: unsupported(
                                    "guard",
                                    "transition guards are outside projection schema v1",
                                ),
                            }
                        } else {
                            ProjectionFeature::Absent
                        },
                        effect: if has_effect {
                            ProjectionFeature::Unsupported {
                                reason: unsupported(
                                    "effect",
                                    "transition effects are outside projection schema v1",
                                ),
                            }
                        } else {
                            ProjectionFeature::Absent
                        },
                        provenance: spec42_generator_protocol::RelationshipProvenance::Authored,
                        source_reference: inspection_source(
                            &inspection,
                            self.model
                                .document_identity(inspection.location.document)
                                .unwrap_or_default(),
                        ),
                    });
                }
                _ => {}
            }
        }
        nodes.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        transitions.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        self.enforce_limit(nodes.len().saturating_add(transitions.len()))?;
        let mut reasons = Vec::new();
        for edge in &transitions {
            if matches!(edge.guard, ProjectionFeature::Unsupported { .. }) {
                reasons.push(unsupported(
                    "guard",
                    "transition guards are outside projection schema v1",
                ));
            }
            if matches!(edge.effect, ProjectionFeature::Unsupported { .. }) {
                reasons.push(unsupported(
                    "effect",
                    "transition effects are outside projection schema v1",
                ));
            }
        }
        Ok(StateTransitionViewProjection {
            schema_version: 1,
            model_digest: self.model_digest.clone(),
            view,
            machine,
            nodes,
            transitions,
            completeness: if reasons.is_empty() {
                ProjectionCompleteness::Complete
            } else {
                ProjectionCompleteness::Incomplete { reasons }
            },
        })
    }

    fn inspection(
        &self,
        identity: SymbolId,
        subject: &str,
    ) -> Result<sysml_query::resolved_slice::ElementInspection, ModelQueryError> {
        outcome(self.model.inspection().inspect(identity), subject)
    }

    fn exposed_machine(&self, view: SymbolId) -> Result<SymbolId, ModelQueryError> {
        let mut targets = Vec::new();
        for child in self
            .by_identity
            .values()
            .filter(|entry| entry.entry.owner == Some(view))
        {
            if child.entry.kind != ElementKind::Expose {
                continue;
            }
            let inspection = self.inspection(child.entry.identity, "view exposure")?;
            if let Some(target) = resolved_relationship(&inspection, "viewExpose")? {
                targets.push(target);
            }
        }
        match targets.as_slice() {
            [one] => Ok(*one),
            [] => Err(ModelQueryError::Unsupported(
                "state-transition view exposes no state machine".into(),
            )),
            _ => Err(ModelQueryError::Ambiguous(
                "state-transition view exposes multiple roots".into(),
            )),
        }
    }

    fn view_summary(
        &self,
        view: SymbolId,
        machine: SymbolId,
    ) -> Result<StateTransitionViewSummary, ModelQueryError> {
        let view_entry = &self.by_identity[&view].entry;
        let machine_entry = self
            .by_identity
            .get(&machine)
            .ok_or_else(|| ModelQueryError::Unresolved("exposed machine".into()))?;
        let inspection = self.inspection(view, "state-transition view")?;
        let handle = handle_from_semantic_id(&self.token(view));
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .insert(handle.clone(), view);
        Ok(StateTransitionViewSummary {
            handle,
            semantic_id: self.token(view),
            name: display_label(&self.model, view_entry),
            exposed_machine: StateMachineIdentity {
                semantic_id: self.token(machine),
                label: display_label(&self.model, &machine_entry.entry),
            },
            source: inspection_source(
                &inspection,
                self.model
                    .document_identity(inspection.location.document)
                    .unwrap_or_default(),
            ),
        })
    }

    fn diagram_reference(
        &self,
        identity: SymbolId,
    ) -> Result<DiagramSemanticReference, ModelQueryError> {
        let registered = self.by_identity.get(&identity).ok_or_else(|| {
            ModelQueryError::Unresolved(format!(
                "diagram reference target `{}` is absent from the publication",
                self.token(identity)
            ))
        })?;
        let source_domain = diagram_source_domain(registered.source);
        if registered.entry.name.is_some() {
            Ok(DiagramSemanticReference::Qualified {
                document: self
                    .model
                    .document_identity(registered.entry.location.document)
                    .unwrap_or_default()
                    .to_owned(),
                qualified_name: self
                    .model
                    .qualified_name(registered.entry.identity)
                    .unwrap_or_default()
                    .to_string(),
                source_domain,
            })
        } else {
            Ok(DiagramSemanticReference::SourceAnchor {
                document: self
                    .model
                    .document_identity(registered.entry.location.document)
                    .unwrap_or_default()
                    .to_owned(),
                owner_qualified_name: registered
                    .entry
                    .owner
                    .as_ref()
                    .and_then(|owner| self.by_identity.get(owner))
                    .map(|owner| {
                        self.model
                            .qualified_name(owner.entry.identity)
                            .unwrap_or_default()
                            .to_string()
                    }),
                metaclass: api_metaclass(registered.entry.kind),
                source_domain,
                range: protocol_source_range(registered.entry.declaration_range),
            })
        }
    }

    fn diagram_occurrence(
        &self,
        occurrence: &sysml_query::resolved_slice::DiagramOccurrenceIdentity,
    ) -> Result<DiagramOccurrenceIdentity, ModelQueryError> {
        Ok(DiagramOccurrenceIdentity {
            semantic_path: occurrence
                .semantic_path
                .iter()
                .map(|identity| self.diagram_reference(*identity))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn diagram_relationship_endpoint(
        &self,
        endpoint: &sysml_query::resolved_slice::DiagramRelationshipEndpoint,
    ) -> Result<DiagramRelationshipEndpoint, ModelQueryError> {
        Ok(DiagramRelationshipEndpoint {
            reference: self.diagram_reference(endpoint.semantic_id)?,
            occurrence: match &endpoint.occurrence {
                sysml_query::resolved_slice::DiagramEndpointOccurrence::Resolved(value) => {
                    DiagramEndpointOccurrence::Resolved(self.diagram_occurrence(value)?)
                }
                sysml_query::resolved_slice::DiagramEndpointOccurrence::Ambiguous(values) => {
                    DiagramEndpointOccurrence::Ambiguous(
                        values
                            .iter()
                            .map(|value| self.diagram_occurrence(value))
                            .collect::<Result<Vec<_>, _>>()?,
                    )
                }
                sysml_query::resolved_slice::DiagramEndpointOccurrence::OutsideProjection => {
                    DiagramEndpointOccurrence::OutsideProjection
                }
            },
        })
    }

    fn diagram_relationship_reference(
        &self,
        source: SymbolId,
        relationship_kind: spec42_generator_protocol::RelationshipKind,
        ordinal: usize,
    ) -> Result<DiagramSemanticReference, ModelQueryError> {
        let registered = self.by_identity.get(&source).ok_or_else(|| {
            ModelQueryError::Unresolved("diagram relationship source is absent".into())
        })?;
        Ok(DiagramSemanticReference::Relationship {
            document: self
                .model
                .document_identity(registered.entry.location.document)
                .unwrap_or_default()
                .to_owned(),
            source_qualified_name: self
                .model
                .qualified_name(registered.entry.identity)
                .unwrap_or_default()
                .to_string(),
            relationship_kind,
            ordinal: u32::try_from(ordinal).map_err(|_| ModelQueryError::ResultLimit {
                actual: ordinal,
                limit: u32::MAX as usize,
            })?,
            source_domain: diagram_source_domain(registered.source),
        })
    }

    fn diagram_incomplete_reason(
        &self,
        reason: sysml_query::resolved_slice::DiagramIncompleteReason,
    ) -> Result<DiagramIncompleteReason, ModelQueryError> {
        use sysml_query::resolved_slice::DiagramIncompleteReason as Owned;
        Ok(match reason {
            Owned::ParseRecovery => DiagramIncompleteReason::ParseRecovery,
            Owned::UnsupportedSyntax => DiagramIncompleteReason::UnsupportedSyntax,
            Owned::NonConverged => DiagramIncompleteReason::NonConverged,
            Owned::ExposureUnresolved { exposure } => DiagramIncompleteReason::ExposureUnresolved {
                exposure: self.diagram_reference(exposure)?,
            },
            Owned::ExposureAmbiguous { exposure } => DiagramIncompleteReason::ExposureAmbiguous {
                exposure: self.diagram_reference(exposure)?,
            },
            Owned::ExposureUnsupported { exposure } => {
                DiagramIncompleteReason::ExposureUnsupported {
                    exposure: self.diagram_reference(exposure)?,
                }
            }
            Owned::RelationshipUnresolved { relationship } => {
                DiagramIncompleteReason::RelationshipUnresolved {
                    relationship_kind: relationship.name().to_owned(),
                }
            }
            Owned::RelationshipAmbiguous { relationship } => {
                DiagramIncompleteReason::RelationshipAmbiguous {
                    relationship_kind: relationship.name().to_owned(),
                }
            }
            Owned::RelationshipUnsupported { relationship } => {
                DiagramIncompleteReason::RelationshipUnsupported {
                    relationship_kind: relationship.name().to_owned(),
                }
            }
            Owned::ViewFilterUnresolved => DiagramIncompleteReason::ViewFilterUnresolved,
            Owned::ViewFilterAmbiguous => DiagramIncompleteReason::ViewFilterAmbiguous,
            Owned::ViewFilterUnsupported => DiagramIncompleteReason::ViewFilterUnsupported,
            Owned::GeometryFactsUnavailable => DiagramIncompleteReason::GeometryFactsUnavailable,
            Owned::SequenceMessageEndpointOutsideLifeline => {
                DiagramIncompleteReason::SequenceMessageEndpointOutsideLifeline
            }
            Owned::SequenceOrderingCycle => DiagramIncompleteReason::SequenceOrderingCycle,
        })
    }

    pub fn is_valid_handle(&self, handle: &str) -> bool {
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .contains_key(handle)
    }

    /// The boundary token for one handle, as the generator protocol spells it.
    ///
    /// A `SymbolId` addresses an element of the publication this view wraps and means nothing
    /// outside it; every `semanticId` the protocol carries is the token instead.
    fn token(&self, symbol: impl std::borrow::Borrow<SymbolId>) -> String {
        self.model
            .symbol_token(*symbol.borrow())
            .map(SymbolToken::into_string)
            .unwrap_or_default()
    }

    fn resolve_handle(&self, handle: &str) -> Result<SymbolId, ModelQueryError> {
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .get(handle)
            .cloned()
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))
    }

    fn summary(
        &self,
        identity: impl std::borrow::Borrow<SymbolId>,
    ) -> Result<ElementSummary, ModelQueryError> {
        let identity = *identity.borrow();
        let registered = self.by_identity.get(&identity).ok_or_else(|| {
            ModelQueryError::Unresolved(format!(
                "element identity `{}` is not in the publication",
                self.token(identity)
            ))
        })?;
        let semantic_id = self.token(identity);
        let handle = handle_from_semantic_id(&semantic_id);
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .insert(handle.clone(), identity);
        Ok(ElementSummary {
            handle,
            semantic_id,
            metaclass: api_metaclass(registered.entry.kind),
            name: registered.entry.name.as_deref().map(str::to_owned),
            qualified_name: self
                .model
                .qualified_name(registered.entry.identity)
                .unwrap_or_default()
                .to_string(),
            library_element: registered.source != ElementSource::Workspace,
        })
    }

    fn summaries(
        &self,
        identities: impl Iterator<Item = SymbolId>,
    ) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let mut values = identities
            .map(|identity| self.summary(identity))
            .collect::<Result<Vec<_>, _>>()?;
        values.sort_by(summary_order);
        self.enforce_limit(values.len())?;
        Ok(values)
    }
    fn enforce_limit(&self, actual: usize) -> Result<(), ModelQueryError> {
        if actual > self.query_limits.max_results {
            Err(ModelQueryError::ResultLimit {
                actual,
                limit: self.query_limits.max_results,
            })
        } else {
            Ok(())
        }
    }
}

fn display_label(
    model: &sysml_query::resolved_slice::PublishedModel,
    entry: &SymbolEntry,
) -> String {
    entry
        .display_label(model.qualified_name(entry.identity).unwrap_or_default())
        .to_owned()
}

/// The boundary form of an inspection's source position.
///
/// `document` is the identity the caller materialised from the location's [`DocumentId`]: the
/// generator protocol leaves this process, so the URI is spelled out rather than carried as a
/// publication-scoped handle.
fn inspection_source(
    inspection: &sysml_query::resolved_slice::ElementInspection,
    document: &str,
) -> SourceReference {
    let range = source_range(inspection.declaration_range);
    SourceReference {
        uri: document.to_owned(),
        range: spec42_generator_protocol::SourceRange {
            start_line: range.start_line,
            start_character: range.start_character,
            end_line: range.end_line,
            end_character: range.end_character,
        },
    }
}

/// The boundary form of one source location; see [`inspection_source`] for `document`.
fn source_reference(
    location: &sysml_query::resolved_slice::SourceLocation,
    document: &str,
) -> SourceReference {
    let range = source_range(location.range);
    SourceReference {
        uri: document.to_owned(),
        range: spec42_generator_protocol::SourceRange {
            start_line: range.start_line,
            start_character: range.start_character,
            end_line: range.end_line,
            end_character: range.end_character,
        },
    }
}

fn diagram_source_domain(source: ElementSource) -> DiagramSourceDomain {
    match source {
        ElementSource::Workspace => DiagramSourceDomain::Workspace,
        ElementSource::StandardLibrary => DiagramSourceDomain::StandardLibrary,
        ElementSource::Library => DiagramSourceDomain::Library,
        ElementSource::External => DiagramSourceDomain::External,
    }
}

fn protocol_source_range(
    range: sysml_query::resolved_slice::TextRange,
) -> spec42_generator_protocol::SourceRange {
    spec42_generator_protocol::SourceRange {
        start_line: range.start.line,
        start_character: range.start.character,
        end_line: range.end.line,
        end_character: range.end.character,
    }
}

fn diagram_kind(kind: sysml_query::resolved_slice::DiagramViewKind) -> DiagramViewKind {
    use sysml_query::resolved_slice::DiagramViewKind as Owned;
    match kind {
        Owned::General => DiagramViewKind::GeneralView,
        Owned::Interconnection => DiagramViewKind::InterconnectionView,
        Owned::ActionFlow => DiagramViewKind::ActionFlowView,
        Owned::StateTransition => DiagramViewKind::StateTransitionView,
        Owned::Sequence => DiagramViewKind::SequenceView,
        Owned::Browser => DiagramViewKind::BrowserView,
        Owned::Grid => DiagramViewKind::GridView,
        Owned::Geometry => DiagramViewKind::GeometryView,
    }
}

fn relationship_provenance(
    provenance: RelationshipProvenance,
) -> spec42_generator_protocol::RelationshipProvenance {
    match provenance {
        RelationshipProvenance::Authored => {
            spec42_generator_protocol::RelationshipProvenance::Authored
        }
        RelationshipProvenance::Implied => {
            spec42_generator_protocol::RelationshipProvenance::Implied
        }
    }
}

fn diagram_metadata(
    kind: DiagramViewKind,
    roots: &[DiagramOccurrenceIdentity],
    elements: &[DiagramElement],
    relationships: &[DiagramRelationship],
    scene: &DiagramScene,
) -> DiagramViewMetadata {
    let ids = |classes: &[Metaclass]| {
        elements
            .iter()
            .filter(|element| classes.contains(&element.metaclass))
            .map(|element| element.occurrence.clone())
            .collect::<Vec<_>>()
    };
    match kind {
        DiagramViewKind::GeneralView => DiagramViewMetadata::General {
            roots: roots.to_vec(),
        },
        DiagramViewKind::InterconnectionView => DiagramViewMetadata::Interconnection {
            parts: ids(&[Metaclass::PartDefinition, Metaclass::PartUsage]),
            ports: ids(&[
                Metaclass::PortDefinition,
                Metaclass::PortUsage,
                Metaclass::ConjugatedPortDefinition,
            ]),
            connectors: ids(&[
                Metaclass::ConnectionDefinition,
                Metaclass::ConnectionUsage,
                Metaclass::BindingConnectorUsage,
            ]),
        },
        DiagramViewKind::ActionFlowView => DiagramViewMetadata::ActionFlow {
            actions: ids(&[Metaclass::ActionDefinition, Metaclass::ActionUsage]),
            control_nodes: ids(&[
                Metaclass::DecisionNodeUsage,
                Metaclass::MergeNodeUsage,
                Metaclass::ForkNodeUsage,
                Metaclass::JoinNodeUsage,
            ]),
        },
        DiagramViewKind::StateTransitionView => DiagramViewMetadata::StateTransition {
            states: ids(&[Metaclass::StateDefinition, Metaclass::StateUsage]),
            initial_nodes: relationships
                .iter()
                .filter(|relationship| {
                    relationship.kind == spec42_generator_protocol::RelationshipKind::InitialState
                })
                .map(|relationship| relationship.source_occurrence.clone())
                .collect(),
            final_nodes: ids(&[Metaclass::FinalState]),
        },
        DiagramViewKind::SequenceView => DiagramViewMetadata::Sequence {
            participants: match scene {
                DiagramScene::Sequence(scene) => scene.lifelines.clone(),
                _ => Vec::new(),
            },
            messages: match scene {
                DiagramScene::Sequence(scene) => scene
                    .messages
                    .iter()
                    .map(|message| message.occurrence.clone())
                    .collect(),
                _ => Vec::new(),
            },
        },
        DiagramViewKind::BrowserView => DiagramViewMetadata::Browser {
            roots: roots.to_vec(),
        },
        DiagramViewKind::GridView => DiagramViewMetadata::Grid {
            rows: elements
                .iter()
                .map(|element| element.occurrence.clone())
                .collect(),
            columns: relationships
                .iter()
                .map(|relationship| relationship.kind.clone())
                .collect::<std::collections::BTreeSet<_>>()
                .into_iter()
                .collect(),
            cells: Vec::new(),
        },
        DiagramViewKind::GeometryView => DiagramViewMetadata::Geometry {
            elements: elements
                .iter()
                .map(|element| element.occurrence.clone())
                .collect(),
            primitives: Vec::new(),
        },
    }
}

fn resolved_relationship(
    inspection: &sysml_query::resolved_slice::ElementInspection,
    kind: &str,
) -> Result<Option<SymbolId>, ModelQueryError> {
    let values = inspection
        .relationships
        .iter()
        .filter(|relationship| relationship.kind == kind)
        .collect::<Vec<_>>();
    if values.len() > 1 {
        return Err(ModelQueryError::Ambiguous(format!(
            "multiple `{kind}` relationships"
        )));
    }
    match values.first().map(|value| &value.target) {
        None => Ok(None),
        Some(RelationshipTarget::Resolved(target)) => Ok(Some(*target)),
        Some(RelationshipTarget::Ambiguous(_)) => Err(ModelQueryError::Ambiguous(kind.into())),
        Some(RelationshipTarget::Unresolved) => Err(ModelQueryError::Unresolved(kind.into())),
        Some(RelationshipTarget::Unsupported) => Err(ModelQueryError::Unsupported(kind.into())),
    }
}

fn unsupported(code: &str, message: &str) -> spec42_generator_protocol::UnsupportedReason {
    spec42_generator_protocol::UnsupportedReason {
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

fn outcome<T>(value: QueryOutcome<T>, operation: &str) -> Result<T, ModelQueryError> {
    match value.answer {
        QueryAnswer::Resolved(value) => Ok(value),
        QueryAnswer::Unresolved => Err(ModelQueryError::Unresolved(operation.into())),
        QueryAnswer::Ambiguous(values) => Err(ModelQueryError::Ambiguous(format!(
            "{operation} returned {} candidates",
            values.len()
        ))),
        QueryAnswer::Unsupported => Err(ModelQueryError::Unsupported(operation.into())),
        QueryAnswer::Recovery => Err(ModelQueryError::Unresolved(format!(
            "{operation} is in parser recovery"
        ))),
        QueryAnswer::Incomplete => Err(ModelQueryError::Incomplete),
    }
}

fn handle_from_semantic_id(id: &str) -> String {
    let mut hash = Sha256::new();
    hash.update(b"spec42-generator-handle-v2\0");
    hash.update(id.as_bytes());
    format!("h:{:x}", hash.finalize())
}
fn summary_order(a: &ElementSummary, b: &ElementSummary) -> std::cmp::Ordering {
    a.qualified_name
        .cmp(&b.qualified_name)
        .then_with(|| a.metaclass.as_str().cmp(b.metaclass.as_str()))
        .then_with(|| a.semantic_id.cmp(&b.semantic_id))
}
fn api_metaclass(kind: ElementKind) -> Metaclass {
    // The semantic vocabulary uses the concrete `FlowConnection*` spelling while the generator
    // wire contract uses SysML's `Flow*` metaclass spelling. Keep the typed bridge at this boundary;
    // `Metaclass::parse` is for its own vocabulary, not a second semantic classification table.
    match kind {
        ElementKind::FlowConnectionDefinition => return Metaclass::FlowDefinition,
        ElementKind::FlowConnectionUsage => return Metaclass::FlowUsage,
        _ => {}
    }
    let parsed = Metaclass::parse(kind.as_str());
    if parsed.is_unrecognized() {
        Metaclass::Unrecognized(kind.as_str().to_owned())
    } else {
        parsed
    }
}

fn diagram_notation_role(kind: ElementKind) -> DiagramNotationRole {
    let metaclass = api_metaclass(kind);
    match metaclass {
        Metaclass::ActionDefinition
        | Metaclass::AllocationDefinition
        | Metaclass::AnalysisCaseDefinition
        | Metaclass::AttributeDefinition
        | Metaclass::CalculationDefinition
        | Metaclass::CaseDefinition
        | Metaclass::ConcernDefinition
        | Metaclass::ConnectionDefinition
        | Metaclass::ConstraintDefinition
        | Metaclass::EnumerationDefinition
        | Metaclass::FlowDefinition
        | Metaclass::IndividualDefinition
        | Metaclass::InterfaceDefinition
        | Metaclass::ItemDefinition
        | Metaclass::MetadataDefinition
        | Metaclass::OccurrenceDefinition
        | Metaclass::PartDefinition
        | Metaclass::PortDefinition
        | Metaclass::RenderingDefinition
        | Metaclass::RequirementDefinition
        | Metaclass::StateDefinition
        | Metaclass::UseCaseDefinition
        | Metaclass::VerificationCaseDefinition
        | Metaclass::ViewDefinition
        | Metaclass::ViewpointDefinition
        | Metaclass::ConjugatedPortDefinition => DiagramNotationRole::Definition,
        Metaclass::ReferenceUsage => DiagramNotationRole::ReferenceUsage,
        Metaclass::Package | Metaclass::Alias | Metaclass::Import => DiagramNotationRole::Namespace,
        Metaclass::Documentation
        | Metaclass::MetadataUsage
        | Metaclass::TextualRepresentation
        | Metaclass::Diagnostic => DiagramNotationRole::Annotation,
        Metaclass::ActionUsage
        | Metaclass::AllocationUsage
        | Metaclass::AnalysisCaseUsage
        | Metaclass::AttributeUsage
        | Metaclass::CalculationUsage
        | Metaclass::CaseUsage
        | Metaclass::ConcernUsage
        | Metaclass::ConnectionUsage
        | Metaclass::ConstraintUsage
        | Metaclass::EnumerationUsage
        | Metaclass::FlowUsage
        | Metaclass::IndividualUsage
        | Metaclass::InterfaceUsage
        | Metaclass::ItemUsage
        | Metaclass::OccurrenceUsage
        | Metaclass::PartUsage
        | Metaclass::PortUsage
        | Metaclass::RenderingUsage
        | Metaclass::RequirementUsage
        | Metaclass::StateUsage
        | Metaclass::UseCaseUsage
        | Metaclass::VerificationCaseUsage
        | Metaclass::ViewUsage
        | Metaclass::ViewpointUsage
        | Metaclass::TransitionUsage
        | Metaclass::TransitionTrigger
        | Metaclass::TransitionGuard
        | Metaclass::TransitionEffect
        | Metaclass::FinalState
        | Metaclass::ActorUsage
        | Metaclass::StakeholderUsage
        | Metaclass::SubjectUsage
        | Metaclass::PerformUsage
        | Metaclass::IncludeUseCaseUsage
        | Metaclass::ViewRendering
        | Metaclass::ViewColumn
        | Metaclass::KermlDeclaration
        | Metaclass::AnalysisResultUsage
        | Metaclass::AssertConstraintUsage
        | Metaclass::AssertUsage
        | Metaclass::AssignmentActionUsage
        | Metaclass::BindingConnectorUsage
        | Metaclass::DecisionNodeUsage
        | Metaclass::Dependency
        | Metaclass::DerivationConnectorUsage
        | Metaclass::ElseActionUsage
        | Metaclass::FilterUsage
        | Metaclass::FlowPayload
        | Metaclass::ForLoopActionUsage
        | Metaclass::ForkNodeUsage
        | Metaclass::IfActionUsage
        | Metaclass::InterfaceEndUsage
        | Metaclass::JoinNodeUsage
        | Metaclass::MergeNodeUsage
        | Metaclass::NeedUsage
        | Metaclass::ObjectiveUsage
        | Metaclass::ParameterUsage
        | Metaclass::PurposeUsage
        | Metaclass::RequireConstraintUsage
        | Metaclass::TerminateActionUsage
        | Metaclass::VerdictUsage
        | Metaclass::VerifyUsage
        | Metaclass::WhileLoopActionUsage => DiagramNotationRole::Usage,
        Metaclass::Unrecognized(_) => DiagramNotationRole::Unsupported,
    }
}
fn source_range(range: sysml_query::resolved_slice::TextRange) -> SourceRange {
    SourceRange {
        start_line: range.start.line,
        start_character: range.start.character,
        end_line: range.end.line,
        end_character: range.end.character,
    }
}
fn bound(value: MultiplicityBound) -> Result<Option<String>, ModelQueryError> {
    match value {
        MultiplicityBound::Unbounded => Ok(None),
        MultiplicityBound::Literal(value) => Ok(Some(value.to_string())),
        MultiplicityBound::Expression => Err(ModelQueryError::Unsupported(
            "generator element detail cannot serialize a non-literal multiplicity bound".into(),
        )),
    }
}
fn scalar(value: &EvaluatedScalar) -> String {
    match value {
        EvaluatedScalar::Boolean(v) => v.to_string(),
        EvaluatedScalar::Integer(v) => v.to_string(),
        EvaluatedScalar::Real(v) => v.to_string(),
        EvaluatedScalar::String(v) => v.to_string(),
        EvaluatedScalar::Quantity { magnitude, unit } => {
            format!("{} [{}]", scalar(magnitude), unit)
        }
    }
}

fn generator_relationship_kind(kind: &str) -> &str {
    match kind {
        "featureTyping" => "typing",
        "subclassification" | "specialization" => "specializes",
        other => other,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sysml_query::{source::SourceKind, Services};

    #[test]
    fn model_view_registers_one_canonical_traversal_with_provenance() {
        let services = Services::new();
        let sources = vec![
            services
                .source
                .admit_memory(
                    "generator-test",
                    "workspace.sysml",
                    "package Workspace { part def Vehicle; }",
                    SourceKind::Workspace,
                )
                .unwrap(),
            services
                .source
                .admit_memory(
                    "generator-test",
                    "library.sysml",
                    "package Library { attribute def Mass; }",
                    SourceKind::StandardLibrary,
                )
                .unwrap(),
        ];
        let model = services.publication.publish(&sources, []).unwrap();
        let expected = match model.inspection().all_elements().answer {
            QueryAnswer::Resolved(elements) => elements,
            other => panic!("expected complete traversal, got {other:?}"),
        };
        let view = GeneratorModelView::new(model, "digest", "version", QueryLimits::default())
            .expect("complete generator model");

        assert_eq!(view.by_identity.len(), expected.len());
        for element in expected {
            let registered = view
                .by_identity
                .get(&element.entry.identity)
                .expect("canonical traversal entry registered once");
            assert_eq!(registered.entry, element.entry);
            assert_eq!(registered.source, element.source);
        }
        assert_eq!(
            view.roots()
                .unwrap()
                .iter()
                .filter_map(|root| root.name.as_deref())
                .collect::<Vec<_>>(),
            vec!["Workspace"],
            "library provenance remains available to the adapter's workspace-root policy"
        );
    }

    #[test]
    fn incomplete_publication_retains_status_beside_usable_partial_enumeration() {
        let services = Services::new();
        let source = services
            .source
            .admit_memory(
                "generator-test",
                "recovery.sysml",
                "package P { part def Wheel; part broken : ; }",
                SourceKind::Workspace,
            )
            .unwrap();
        let model = services.publication.publish(&[source], []).unwrap();
        let completeness = model.publication().completeness();
        assert!(!completeness.is_complete());

        let view = GeneratorModelView::new(model, "digest", "version", QueryLimits::default())
            .expect("incomplete publications are normal generator inputs");
        assert_eq!(view.publication_completeness(), completeness);
        assert!(!view
            .roots()
            .expect("settled partial roots remain usable")
            .is_empty());
    }
}
