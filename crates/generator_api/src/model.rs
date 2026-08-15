use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
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
