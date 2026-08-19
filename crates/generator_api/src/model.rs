use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spec42_generator_protocol::{
    DiagramEdge, DiagramEdgeKind, DiagramElement, DiagramIncompleteReason, DiagramNotationRole,
    DiagramRelationship, DiagramRelationshipTarget, DiagramSemanticReference, DiagramSourceDomain,
    DiagramViewKind, DiagramViewMetadata, DiagramViewProjection, DiagramViewSummary,
    ElementIdentity, ProjectionCompleteness, ProjectionFeature, SourceReference,
    StateMachineIdentity, StateMachineSummary, StateTransitionEdge, StateTransitionNode,
    StateTransitionNodeKind, StateTransitionViewProjection, StateTransitionViewSummary,
    TransitionTrigger,
};
use spec42_generator_protocol::{Metaclass, RelationshipKind as ApiRelationshipKind};
use sysml_query::resolved_slice::{
    AnnotationForm, ElementKind, ElementModifier, ElementSearch, ElementSource, EvaluatedScalar,
    MultiplicityBound, MultiplicityFacts, QueryOutcome, RelationshipProvenance, RelationshipTarget,
    SymbolEntry, SymbolIdentity,
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
    by_identity: HashMap<SymbolIdentity, RegisteredElement>,
    handles: Mutex<HashMap<String, SymbolIdentity>>,
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
    ) -> Self {
        let mut by_identity = HashMap::new();
        for source in [
            ElementSource::Workspace,
            ElementSource::StandardLibrary,
            ElementSource::Library,
            ElementSource::External,
        ] {
            for &kind in ElementKind::ALL {
                if let QueryOutcome::Resolved(entries)
                | QueryOutcome::Recovered(entries)
                | QueryOutcome::UnsupportedWith(entries) = model
                    .inspection()
                    .search_elements(ElementSearch { kind, source })
                {
                    for entry in entries {
                        by_identity
                            .insert(entry.identity.clone(), RegisteredElement { entry, source });
                    }
                }
            }
        }
        Self {
            model,
            model_digest: model_digest.into(),
            spec42_version: spec42_version.into(),
            query_limits,
            by_identity,
            handles: Mutex::new(HashMap::new()),
        }
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
                .map(|value| &value.entry.identity),
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
                .map(|value| &value.entry.identity),
        )
    }

    pub fn children(&self, handle: &str) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let parent = self.resolve_handle(handle)?;
        self.summaries(
            self.by_identity
                .values()
                .filter(|value| value.entry.owner.as_ref() == Some(&parent))
                .map(|value| &value.entry.identity),
        )
    }

    pub fn element(&self, handle: &str) -> Result<ElementDetail, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let inspection = outcome(
            self.model.inspection().inspect(&identity),
            "element inspection",
        )?;
        let summary = self.summary(&identity)?;
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
            source_uri: inspection.location.document.to_string(),
            source_range: source_range(inspection.declaration_range),
            definition: inspection.kind.as_str().ends_with("Definition"),
            documentation: inspection
                .documentation
                .iter()
                .find(|doc| doc.form == AnnotationForm::Documentation)
                .map(|doc| doc.text.to_string()),
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
        let types = outcome(self.model.types().direct_types(&identity), "direct typing")?;
        match types.as_ref() {
            [] => Ok(None),
            [one] => self.summary(&one.symbol).map(Some),
            _ => Err(ModelQueryError::Ambiguous(format!(
                "element `{}` has {} direct types",
                identity.as_str(),
                types.len()
            ))),
        }
    }

    pub fn requirement_usage_typing(
        &self,
        handle: &str,
    ) -> Result<RequirementUsageTypingSummary, ModelQueryError> {
        use sysml_query::resolved_slice::{QueryOutcome, RequirementUsageTyping as Owned};
        use RequirementUsageTypingSummary as Wire;
        let identity = self.resolve_handle(handle)?;
        Ok(
            match self.model.types().requirement_usage_typing(&identity) {
                QueryOutcome::Resolved(Owned::Missing) => Wire::Missing,
                QueryOutcome::Resolved(Owned::Resolved(reference)) => Wire::Resolved {
                    definition: self.summary(&reference.symbol)?,
                    provenance: match reference.provenance {
                        RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                        RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                    },
                },
                QueryOutcome::Resolved(Owned::Ambiguous(values)) => Wire::Ambiguous {
                    candidates: values
                        .iter()
                        .map(|value| self.summary(value))
                        .collect::<Result<Vec<_>, _>>()?,
                },
                QueryOutcome::Resolved(Owned::Unresolved) | QueryOutcome::Unresolved => {
                    Wire::Unresolved
                }
                QueryOutcome::Resolved(Owned::Unsupported)
                | QueryOutcome::Unsupported
                | QueryOutcome::UnsupportedWith(_) => Wire::Unsupported,
                QueryOutcome::Recovered(Owned::Resolved(reference)) => Wire::RecoveredResolved {
                    definition: self.summary(&reference.symbol)?,
                    provenance: match reference.provenance {
                        RelationshipProvenance::Authored => TypingProvenanceSummary::Authored,
                        RelationshipProvenance::Implied => TypingProvenanceSummary::Implied,
                    },
                },
                QueryOutcome::Recovered(Owned::Missing) => Wire::RecoveredMissing,
                QueryOutcome::Recovered(Owned::Unresolved) => Wire::RecoveredUnresolved,
                QueryOutcome::Recovered(Owned::Ambiguous(values)) => Wire::RecoveredAmbiguous {
                    candidates: values
                        .iter()
                        .map(|value| self.summary(value))
                        .collect::<Result<Vec<_>, _>>()?,
                },
                QueryOutcome::Recovered(Owned::Unsupported) => Wire::RecoveredUnsupported,
                QueryOutcome::Recovery => Wire::Recovery,
                QueryOutcome::Ambiguous(values) => Wire::Ambiguous {
                    candidates: values
                        .iter()
                        .flat_map(|value| match value {
                            Owned::Resolved(reference) => self.summary(&reference.symbol).ok(),
                            _ => None,
                        })
                        .collect(),
                },
                QueryOutcome::Incomplete => Wire::Incomplete,
            },
        )
    }

    pub fn relationships(&self, handle: &str) -> Result<Vec<RelationshipSummary>, ModelQueryError> {
        let identity = self.resolve_handle(handle)?;
        let inspection = outcome(
            self.model.inspection().inspect(&identity),
            "element relationships",
        )?;
        let source = self.summary(&identity)?;
        let mut values = Vec::new();
        for relationship in &inspection.relationships {
            let target = match &relationship.target {
                RelationshipTarget::Resolved(target) => self.summary(target)?,
                RelationshipTarget::Ambiguous(_) => {
                    return Err(ModelQueryError::Ambiguous(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        identity.as_str()
                    )))
                }
                RelationshipTarget::Unresolved => {
                    return Err(ModelQueryError::Unresolved(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        identity.as_str()
                    )))
                }
                RelationshipTarget::Unsupported => {
                    return Err(ModelQueryError::Unsupported(format!(
                        "relationship `{}` from `{}`",
                        relationship.kind,
                        identity.as_str()
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
            QueryOutcome, SatisfyEndpoint as OwnedEndpoint, SatisfyPolarity as OwnedPolarity,
        };
        let (relationships, recovered) = match self.model.inspection().satisfy_relationships() {
            QueryOutcome::Resolved(values) => (values, false),
            QueryOutcome::Recovered(values) => (values, true),
            QueryOutcome::UnsupportedWith(_) | QueryOutcome::Unsupported => {
                return Err(ModelQueryError::Unsupported("satisfy relationships".into()))
            }
            QueryOutcome::Unresolved => {
                return Err(ModelQueryError::Unresolved("satisfy relationships".into()))
            }
            QueryOutcome::Ambiguous(_) => {
                return Err(ModelQueryError::Ambiguous("satisfy relationships".into()))
            }
            QueryOutcome::Recovery => {
                return Err(ModelQueryError::Unresolved(
                    "satisfy relationships are in parser recovery".into(),
                ))
            }
            QueryOutcome::Incomplete => return Err(ModelQueryError::Incomplete),
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
                    semantic_id: relationship.identity.as_str().to_owned(),
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
            QueryOutcome, VerificationOutcome as OwnedOutcome,
            VerificationRequirement as OwnedRequirement,
        };
        let (relationships, recovered) = match self.model.inspection().requirement_verifications() {
            QueryOutcome::Resolved(values) => (values, false),
            QueryOutcome::Recovered(values) => (values, true),
            QueryOutcome::UnsupportedWith(_) | QueryOutcome::Unsupported => {
                return Err(ModelQueryError::Unsupported(
                    "requirement verifications".into(),
                ))
            }
            QueryOutcome::Unresolved | QueryOutcome::Recovery => {
                return Err(ModelQueryError::Unresolved(
                    "requirement verifications".into(),
                ))
            }
            QueryOutcome::Ambiguous(_) => {
                return Err(ModelQueryError::Ambiguous(
                    "requirement verifications".into(),
                ))
            }
            QueryOutcome::Incomplete => return Err(ModelQueryError::Incomplete),
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
                    semantic_id: value.identity.as_str().to_owned(),
                    verification_case: self.summary(&value.verification_case)?,
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
            self.model.inspection().effective_features(&identity),
            "effective features",
        )?;
        let values = features
            .iter()
            .map(|feature| self.summary(&feature.identity))
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
                let handle = handle_from_semantic_id(entry.semantic_id.as_str());
                self.handles
                    .lock()
                    .expect("generator handle index poisoned")
                    .insert(handle.clone(), entry.semantic_id.clone());
                Ok(DiagramViewSummary {
                    handle,
                    kind: diagram_kind(entry.kind),
                    reference: self.diagram_reference(&entry.semantic_id)?,
                    name: entry.name.to_string(),
                    source: source_reference(&entry.source),
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
        let projection = outcome(self.model.diagrams().view(&identity), "diagram view")?;
        let view = DiagramViewSummary {
            handle: handle.to_owned(),
            reference: self.diagram_reference(&projection.view.semantic_id)?,
            kind: diagram_kind(projection.view.kind),
            name: projection.view.name.to_string(),
            source: source_reference(&projection.view.source),
        };
        let elements = projection
            .elements
            .iter()
            .map(|element| DiagramElement {
                reference: self
                    .diagram_reference(&element.semantic_id)
                    .expect("published diagram element"),
                metaclass: api_metaclass(element.kind),
                notation_role: diagram_notation_role(element.kind),
                name: element.name.as_deref().map(str::to_owned),
                owner: element.owner.as_ref().map(|owner| {
                    self.diagram_reference(owner)
                        .expect("published diagram owner")
                }),
                source: source_reference(&element.source),
            })
            .collect::<Vec<_>>();
        let relationships = projection
            .relationships
            .iter()
            .enumerate()
            .map(|(ordinal, relationship)| {
                let kind = spec42_generator_protocol::RelationshipKind::parse(
                    generator_relationship_kind(&relationship.kind),
                );
                Ok(DiagramRelationship {
                    reference: self.diagram_relationship_reference(
                        &relationship.source,
                        kind.clone(),
                        ordinal,
                    )?,
                    source_element: self.diagram_reference(&relationship.source)?,
                    kind,
                    target: match &relationship.target {
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Resolved(
                            target,
                        ) => DiagramRelationshipTarget::Resolved(self.diagram_reference(target)?),
                        sysml_query::resolved_slice::DiagramRelationshipTarget::Ambiguous(
                            values,
                        ) => DiagramRelationshipTarget::Ambiguous(
                            values
                                .iter()
                                .map(|value| self.diagram_reference(value))
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
                    source: relationship.source_location.as_ref().map(source_reference),
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
                Ok(DiagramEdge {
                    reference: self.diagram_relationship_reference(
                        &edge.source,
                        reference_kind,
                        ordinal,
                    )?,
                    source_element: self.diagram_reference(&edge.source)?,
                    target_element: self.diagram_reference(&edge.target)?,
                    kind,
                    provenance: relationship_provenance(edge.provenance),
                    source: edge.source_location.as_ref().map(source_reference),
                })
            })
            .collect::<Result<Vec<_>, ModelQueryError>>()?;
        let roots = projection
            .exposed_roots
            .iter()
            .map(|root| self.diagram_reference(root))
            .collect::<Result<Vec<_>, _>>()?;
        let reasons = projection
            .incomplete_reasons
            .iter()
            .cloned()
            .map(|reason| self.diagram_incomplete_reason(reason))
            .collect::<Result<Vec<_>, _>>()?;
        let metadata = diagram_metadata(view.kind, &roots, &elements, &relationships);
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
                self.model.types().direct_types(&registered.entry.identity),
                "state-transition view typing",
            )?;
            if !types.iter().any(|ty| ty.symbol == standard_view) {
                continue;
            }
            let machine = self.exposed_machine(&registered.entry.identity)?;
            values.push(self.view_summary(&registered.entry.identity, &machine)?);
        }
        values.sort_by(|a, b| a.semantic_id.cmp(&b.semantic_id));
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    fn standard_state_transition_view(&self) -> Result<SymbolIdentity, ModelQueryError> {
        let matches = self
            .by_identity
            .values()
            .filter(|entry| {
                entry.source == ElementSource::StandardLibrary
                    && entry.entry.kind == ElementKind::ViewDefinition
                    && entry.entry.name.as_deref() == Some("StateTransitionView")
            })
            .map(|entry| entry.entry.identity.clone())
            .collect::<Vec<_>>();
        match matches.as_slice() {
            [identity] => Ok(identity.clone()),
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
        let machine_id = self.exposed_machine(&view_id)?;
        let machine_entry = self.by_identity.get(&machine_id).ok_or_else(|| {
            ModelQueryError::Unresolved("exposed state machine is absent from publication".into())
        })?;
        if machine_entry.entry.kind != ElementKind::StateDefinition {
            return Err(ModelQueryError::Unsupported(
                "state-transition view must expose one state definition".into(),
            ));
        }
        let view = self.view_summary(&view_id, &machine_id)?;
        let machine_inspection = self.inspection(&machine_id, "state machine")?;
        let machine = StateMachineSummary {
            semantic_id: machine_id.as_str().to_owned(),
            label: display_label(&machine_entry.entry),
            source: inspection_source(&machine_inspection),
        };
        let children = self
            .by_identity
            .values()
            .filter(|entry| entry.entry.owner.as_ref() == Some(&machine_id))
            .collect::<Vec<_>>();
        let mut nodes = Vec::new();
        let mut transitions = Vec::new();
        for child in &children {
            let inspection = self.inspection(&child.entry.identity, "state-machine member")?;
            match child.entry.kind {
                ElementKind::StateUsage | ElementKind::FinalState => {
                    nodes.push(StateTransitionNode {
                        semantic_id: child.entry.identity.as_str().to_owned(),
                        label: display_label(&child.entry),
                        kind: if child.entry.kind == ElementKind::FinalState {
                            StateTransitionNodeKind::Final
                        } else {
                            StateTransitionNodeKind::State
                        },
                        source: inspection_source(&inspection),
                    })
                }
                ElementKind::SuccessionAsUsage => {
                    if let Some(target) = resolved_relationship(&inspection, "initialState")? {
                        let initial_id = format!("{}#initial", child.entry.identity.as_str());
                        nodes.push(StateTransitionNode {
                            semantic_id: initial_id.clone(),
                            label: String::new(),
                            kind: StateTransitionNodeKind::Initial,
                            source: inspection_source(&inspection),
                        });
                        transitions.push(StateTransitionEdge {
                            semantic_id: format!("{}#edge", child.entry.identity.as_str()),
                            label: None,
                            source: initial_id,
                            target: target.as_str().to_owned(),
                            trigger: TransitionTrigger::None,
                            guard: ProjectionFeature::Absent,
                            effect: ProjectionFeature::Absent,
                            provenance: spec42_generator_protocol::RelationshipProvenance::Authored,
                            source_reference: inspection_source(&inspection),
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
                                label: display_label(&target_entry.entry),
                                target: Some(ElementIdentity {
                                    semantic_id: trigger.as_str().to_owned(),
                                    label: display_label(&target_entry.entry),
                                }),
                                source: inspection_source(&inspection),
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
                        semantic_id: child.entry.identity.as_str().to_owned(),
                        label: child.entry.name.as_deref().map(str::to_owned),
                        source: source.as_str().to_owned(),
                        target: target.as_str().to_owned(),
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
                        source_reference: inspection_source(&inspection),
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
        identity: &SymbolIdentity,
        subject: &str,
    ) -> Result<sysml_query::resolved_slice::ElementInspection, ModelQueryError> {
        outcome(self.model.inspection().inspect(identity), subject)
    }

    fn exposed_machine(&self, view: &SymbolIdentity) -> Result<SymbolIdentity, ModelQueryError> {
        let mut targets = Vec::new();
        for child in self
            .by_identity
            .values()
            .filter(|entry| entry.entry.owner.as_ref() == Some(view))
        {
            if child.entry.kind != ElementKind::Expose {
                continue;
            }
            let inspection = self.inspection(&child.entry.identity, "view exposure")?;
            if let Some(target) = resolved_relationship(&inspection, "viewExpose")? {
                targets.push(target);
            }
        }
        match targets.as_slice() {
            [one] => Ok(one.clone()),
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
        view: &SymbolIdentity,
        machine: &SymbolIdentity,
    ) -> Result<StateTransitionViewSummary, ModelQueryError> {
        let view_entry = &self.by_identity[view].entry;
        let machine_entry = self
            .by_identity
            .get(machine)
            .ok_or_else(|| ModelQueryError::Unresolved("exposed machine".into()))?;
        let inspection = self.inspection(view, "state-transition view")?;
        let handle = handle_from_semantic_id(view.as_str());
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .insert(handle.clone(), view.clone());
        Ok(StateTransitionViewSummary {
            handle,
            semantic_id: view.as_str().to_owned(),
            name: display_label(view_entry),
            exposed_machine: StateMachineIdentity {
                semantic_id: machine.as_str().to_owned(),
                label: display_label(&machine_entry.entry),
            },
            source: inspection_source(&inspection),
        })
    }

    fn diagram_reference(
        &self,
        identity: &SymbolIdentity,
    ) -> Result<DiagramSemanticReference, ModelQueryError> {
        let registered = self.by_identity.get(identity).ok_or_else(|| {
            ModelQueryError::Unresolved(format!(
                "diagram reference target `{}` is absent from the publication",
                identity.as_str()
            ))
        })?;
        let source_domain = diagram_source_domain(registered.source);
        if registered.entry.name.is_some() {
            Ok(DiagramSemanticReference::Qualified {
                document: registered.entry.location.document.to_string(),
                qualified_name: registered.entry.qualified_name.to_string(),
                source_domain,
            })
        } else {
            Ok(DiagramSemanticReference::SourceAnchor {
                document: registered.entry.location.document.to_string(),
                owner_qualified_name: registered
                    .entry
                    .owner
                    .as_ref()
                    .and_then(|owner| self.by_identity.get(owner))
                    .map(|owner| owner.entry.qualified_name.to_string()),
                metaclass: api_metaclass(registered.entry.kind),
                source_domain,
                range: protocol_source_range(registered.entry.declaration_range),
            })
        }
    }

    fn diagram_relationship_reference(
        &self,
        source: &SymbolIdentity,
        relationship_kind: spec42_generator_protocol::RelationshipKind,
        ordinal: usize,
    ) -> Result<DiagramSemanticReference, ModelQueryError> {
        let registered = self.by_identity.get(source).ok_or_else(|| {
            ModelQueryError::Unresolved("diagram relationship source is absent".into())
        })?;
        Ok(DiagramSemanticReference::Relationship {
            document: registered.entry.location.document.to_string(),
            source_qualified_name: registered.entry.qualified_name.to_string(),
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
                exposure: self.diagram_reference(&exposure)?,
            },
            Owned::ExposureAmbiguous { exposure } => DiagramIncompleteReason::ExposureAmbiguous {
                exposure: self.diagram_reference(&exposure)?,
            },
            Owned::ExposureUnsupported { exposure } => {
                DiagramIncompleteReason::ExposureUnsupported {
                    exposure: self.diagram_reference(&exposure)?,
                }
            }
            Owned::RelationshipUnresolved { relationship } => {
                DiagramIncompleteReason::RelationshipUnresolved {
                    relationship_kind: relationship.to_string(),
                }
            }
            Owned::RelationshipAmbiguous { relationship } => {
                DiagramIncompleteReason::RelationshipAmbiguous {
                    relationship_kind: relationship.to_string(),
                }
            }
            Owned::RelationshipUnsupported { relationship } => {
                DiagramIncompleteReason::RelationshipUnsupported {
                    relationship_kind: relationship.to_string(),
                }
            }
            Owned::ViewFilterApplicationUnavailable => {
                DiagramIncompleteReason::ViewFilterApplicationUnavailable
            }
            Owned::GeometryFactsUnavailable => DiagramIncompleteReason::GeometryFactsUnavailable,
        })
    }

    pub fn is_valid_handle(&self, handle: &str) -> bool {
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .contains_key(handle)
    }

    fn resolve_handle(&self, handle: &str) -> Result<SymbolIdentity, ModelQueryError> {
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .get(handle)
            .cloned()
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))
    }
    fn summary(&self, identity: &SymbolIdentity) -> Result<ElementSummary, ModelQueryError> {
        let registered = self.by_identity.get(identity).ok_or_else(|| {
            ModelQueryError::Unresolved(format!(
                "element identity `{}` is not in the publication",
                identity.as_str()
            ))
        })?;
        let semantic_id = identity.as_str().to_owned();
        let handle = handle_from_semantic_id(&semantic_id);
        self.handles
            .lock()
            .expect("generator handle index poisoned")
            .insert(handle.clone(), identity.clone());
        Ok(ElementSummary {
            handle,
            semantic_id,
            metaclass: api_metaclass(registered.entry.kind),
            name: registered.entry.name.as_deref().map(str::to_owned),
            qualified_name: registered.entry.qualified_name.to_string(),
            library_element: registered.source != ElementSource::Workspace,
        })
    }
    fn summaries<'a>(
        &self,
        identities: impl Iterator<Item = &'a SymbolIdentity>,
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

fn display_label(entry: &SymbolEntry) -> String {
    entry
        .name
        .as_deref()
        .unwrap_or(entry.qualified_name.as_ref())
        .to_owned()
}

fn inspection_source(
    inspection: &sysml_query::resolved_slice::ElementInspection,
) -> SourceReference {
    let range = source_range(inspection.declaration_range);
    SourceReference {
        uri: inspection.location.document.to_string(),
        range: spec42_generator_protocol::SourceRange {
            start_line: range.start_line,
            start_character: range.start_character,
            end_line: range.end_line,
            end_character: range.end_character,
        },
    }
}

fn source_reference(location: &sysml_query::resolved_slice::SourceLocation) -> SourceReference {
    let range = source_range(location.range);
    SourceReference {
        uri: location.document.to_string(),
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
    roots: &[DiagramSemanticReference],
    elements: &[DiagramElement],
    relationships: &[DiagramRelationship],
) -> DiagramViewMetadata {
    let ids = |classes: &[Metaclass]| {
        elements
            .iter()
            .filter(|element| classes.contains(&element.metaclass))
            .map(|element| element.reference.clone())
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
                .map(|relationship| relationship.source_element.clone())
                .collect(),
            final_nodes: ids(&[Metaclass::FinalState]),
        },
        DiagramViewKind::SequenceView => DiagramViewMetadata::Sequence {
            participants: ids(&[
                Metaclass::PartUsage,
                Metaclass::PortUsage,
                Metaclass::ActorUsage,
            ]),
            messages: ids(&[Metaclass::FlowUsage]),
        },
        DiagramViewKind::BrowserView => DiagramViewMetadata::Browser {
            roots: roots.to_vec(),
        },
        DiagramViewKind::GridView => DiagramViewMetadata::Grid {
            rows: elements
                .iter()
                .map(|element| element.reference.clone())
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
                .map(|element| element.reference.clone())
                .collect(),
            primitives: Vec::new(),
        },
    }
}

fn resolved_relationship(
    inspection: &sysml_query::resolved_slice::ElementInspection,
    kind: &str,
) -> Result<Option<SymbolIdentity>, ModelQueryError> {
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
        Some(RelationshipTarget::Resolved(target)) => Ok(Some(target.clone())),
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
    match value {
        QueryOutcome::Resolved(value)
        | QueryOutcome::Recovered(value)
        | QueryOutcome::UnsupportedWith(value) => Ok(value),
        QueryOutcome::Unresolved => Err(ModelQueryError::Unresolved(operation.into())),
        QueryOutcome::Ambiguous(values) => Err(ModelQueryError::Ambiguous(format!(
            "{operation} returned {} candidates",
            values.len()
        ))),
        QueryOutcome::Unsupported => Err(ModelQueryError::Unsupported(operation.into())),
        QueryOutcome::Recovery => Err(ModelQueryError::Unresolved(format!(
            "{operation} is in parser recovery"
        ))),
        QueryOutcome::Incomplete => Err(ModelQueryError::Incomplete),
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
