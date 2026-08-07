use std::collections::{BTreeSet, HashMap, HashSet};
use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use spec42_generator_protocol::{Metaclass, RelationshipKind as ApiRelationshipKind};
use thiserror::Error;
use workspace::{
    HostSemanticModelNode, HostWorkspaceSnapshot, NodeId, RelationshipKind, SemanticNode,
};

pub const GENERATOR_SEMANTIC_API_VERSION: &str = "0.1.0";

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
    /// Opaque invocation-scoped handle used by query methods.
    pub handle: String,
    /// Deterministic semantic identity suitable for provenance.
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

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum ModelQueryError {
    #[error("unknown or expired element handle `{0}`")]
    UnknownHandle(String),
    #[error("query returned {actual} elements; the configured limit is {limit}")]
    ResultLimit { actual: usize, limit: usize },
}

/// Immutable, runtime-neutral semantic query facade for one workspace snapshot.
pub struct GeneratorModelView {
    snapshot: Arc<HostWorkspaceSnapshot>,
    query_limits: QueryLimits,
    exposed: Mutex<HashMap<String, NodeId>>,
    projected_by_key: HashMap<(String, String), HostSemanticModelNode>,
}

impl std::fmt::Debug for GeneratorModelView {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("GeneratorModelView")
            .field("model_digest", &self.model_digest())
            .field("query_limits", &self.query_limits)
            .finish_non_exhaustive()
    }
}

impl GeneratorModelView {
    pub fn new(snapshot: Arc<HostWorkspaceSnapshot>, query_limits: QueryLimits) -> Self {
        let mut exposed = HashMap::new();
        let mut projected_by_key = HashMap::new();
        for node in &snapshot.semantic_projection().nodes {
            if let Some(id) = node_id_for_projected(&snapshot, node) {
                if let Some(semantic) = snapshot.semantic_graph().get_node(&id) {
                    exposed.insert(handle_for(&snapshot, semantic), id);
                }
            }
            projected_by_key.insert(
                (node.uri.clone(), node.qualified_name.clone()),
                node.clone(),
            );
        }
        Self {
            snapshot,
            query_limits,
            exposed: Mutex::new(exposed),
            projected_by_key,
        }
    }

    pub fn model_digest(&self) -> String {
        let metadata = self.snapshot.metadata();
        let mut hasher = Sha256::new();
        hasher.update(b"spec42-generator-model-v1\0");
        hasher.update(metadata.engine_version.as_bytes());
        hasher.update([0]);
        hasher.update(GENERATOR_SEMANTIC_API_VERSION.as_bytes());
        hasher.update([0]);
        hasher.update(
            metadata
                .schema_versions
                .projection_schema_version
                .to_le_bytes(),
        );
        let mut documents = self
            .snapshot
            .documents()
            .iter()
            .map(|document| {
                (
                    normalized_origin(&self.snapshot, &NodeId::new(&document.uri, "")),
                    document.sha256.clone().unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>();
        documents.sort();
        for (origin, digest) in documents {
            hasher.update([0]);
            hasher.update(origin.as_bytes());
            hasher.update([0]);
            hasher.update(digest.as_bytes());
        }
        format!("sha256:{:x}", hasher.finalize())
    }

    pub fn spec42_version(&self) -> &str {
        &self.snapshot.metadata().engine_version
    }

    pub fn semantic_api_version(&self) -> &'static str {
        GENERATOR_SEMANTIC_API_VERSION
    }

    pub fn roots(&self) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let projected = &self.snapshot.semantic_projection().nodes;
        let projected_names = projected
            .iter()
            .map(|node| node.qualified_name.as_str())
            .collect::<HashSet<_>>();
        let values = projected
            .iter()
            .filter(|node| {
                !node.facts.is_library_element
                    && node
                        .parent
                        .as_deref()
                        .is_none_or(|parent| !projected_names.contains(parent))
            })
            .filter_map(|node| node_id_for_projected(&self.snapshot, node))
            .filter_map(|id| self.summary_for_id(&id))
            .collect();
        self.sorted_bounded(values)
    }

    pub fn find(&self, metaclass: Option<&str>) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let values = self
            .snapshot
            .semantic_projection()
            .nodes
            .iter()
            .filter(|node| !node.facts.is_library_element)
            .filter(|node| metaclass.is_none_or(|kind| metaclass_matches(node, kind)))
            .filter_map(|node| node_id_for_projected(&self.snapshot, node))
            .filter_map(|id| self.summary_for_id(&id))
            .collect();
        self.sorted_bounded(values)
    }

    pub fn children(&self, handle: &str) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let id = self.resolve_handle(handle)?;
        let Some(parent) = self.snapshot.semantic_graph().get_node(&id) else {
            return Err(ModelQueryError::UnknownHandle(handle.to_owned()));
        };
        let values = self
            .snapshot
            .semantic_graph()
            .children_of(parent)
            .into_iter()
            .filter(|node| node.element_kind.as_str() != "diagnostic")
            .map(|node| self.expose_node(node))
            .collect();
        self.sorted_bounded(values)
    }

    pub fn element(&self, handle: &str) -> Result<ElementDetail, ModelQueryError> {
        let id = self.resolve_handle(handle)?;
        let node = self
            .snapshot
            .semantic_graph()
            .get_node(&id)
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))?;
        let owner = self
            .snapshot
            .semantic_graph()
            .parent_of(node)
            .map(|parent| self.expose_node(parent));
        let projected = self.projected(node);
        let attributes = projected
            .map(|value| &value.attributes)
            .unwrap_or(&node.attributes);
        let properties = projected.and_then(|value| value.facts.feature_properties.as_ref());
        let declared_properties = node.declared_facts.feature_properties.as_ref();
        let multiplicity =
            node.declared_facts
                .multiplicity
                .as_ref()
                .map(|value| MultiplicitySummary {
                    lower: value.lower.as_ref().map(expression_string),
                    upper: value.upper.as_ref().map(expression_string),
                    ordered: value.is_ordered,
                    unique: value.is_unique,
                    implied: value.is_implied,
                });
        Ok(ElementDetail {
            summary: self.expose_node(node),
            owner,
            declared_name: projected
                .and_then(|value| value.facts.declared_name.clone())
                .or_else(|| (!node.name.is_empty()).then(|| node.name.clone())),
            effective_name: projected
                .map(|value| value.facts.effective_name.clone())
                .filter(|value| !value.is_empty())
                .or_else(|| (!node.name.is_empty()).then(|| node.name.clone())),
            source_uri: normalized_origin(&self.snapshot, &node.id),
            source_range: SourceRange {
                start_line: node.range.start.line,
                start_character: node.range.start.character,
                end_line: node.range.end.line,
                end_character: node.range.end.character,
            },
            definition: node.element_kind.is_definition(),
            documentation: attributes
                .get("doc")
                .and_then(|value| value.as_str())
                .map(str::to_owned),
            short_name: attributes
                .get("shortName")
                .and_then(|value| value.as_str())
                .map(str::to_owned),
            direction: properties
                .and_then(|value| value.direction.clone())
                .or_else(|| declared_properties.and_then(|value| value.direction.clone())),
            derived: properties
                .map(|value| value.is_derived)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_derived)),
            constant: properties
                .map(|value| value.is_constant)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_constant)),
            abstract_: properties
                .map(|value| value.is_abstract)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_abstract)),
            variation: properties
                .map(|value| value.is_variation)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_variation)),
            individual: properties
                .map(|value| value.is_individual)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_individual)),
            conjugated: properties
                .map(|value| value.is_conjugated)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_conjugated)),
            composite: properties
                .and_then(|value| value.is_composite)
                .or_else(|| declared_properties.and_then(|value| value.is_composite)),
            reference: properties
                .and_then(|value| value.is_reference)
                .or_else(|| declared_properties.and_then(|value| value.is_reference)),
            end: properties
                .map(|value| value.is_end)
                .unwrap_or_else(|| declared_properties.is_some_and(|value| value.is_end)),
            ordered: properties
                .and_then(|value| value.is_ordered)
                .or_else(|| declared_properties.and_then(|value| value.is_ordered)),
            unique: properties
                .and_then(|value| value.is_unique)
                .or_else(|| declared_properties.and_then(|value| value.is_unique)),
            multiplicity,
            evaluated_value: attributes.get("evaluatedValue").map(value_string),
        })
    }

    pub fn typed_by(&self, handle: &str) -> Result<Option<ElementSummary>, ModelQueryError> {
        let id = self.resolve_handle(handle)?;
        let node = self
            .snapshot
            .semantic_graph()
            .get_node(&id)
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))?;
        let mut targets = self
            .snapshot
            .semantic_graph()
            .outgoing_targets_by_kind(node, RelationshipKind::Typing);
        targets.sort_by(node_order);
        Ok(targets.first().map(|node| self.expose_node(node)))
    }

    pub fn relationships(&self, handle: &str) -> Result<Vec<RelationshipSummary>, ModelQueryError> {
        let id = self.resolve_handle(handle)?;
        let node = self
            .snapshot
            .semantic_graph()
            .get_node(&id)
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))?;
        let source = self.expose_node(node);
        let projection = self.snapshot.semantic_projection();
        let mut seen = BTreeSet::new();
        let mut values = self
            .snapshot
            .semantic_graph()
            .outgoing_relationships(node)
            .into_iter()
            .map(|(target, kind)| {
                let target = self.expose_node(target);
                let kind = ApiRelationshipKind::parse(kind.as_str());
                let implied = projection.relationships.iter().any(|relationship| {
                    relationship.source == node.id.qualified_name
                        && relationship.target == target.qualified_name
                        && relationship.kind.as_str() == kind.as_str()
                        && relationship.is_implied
                });
                seen.insert((kind.clone(), target.semantic_id.clone()));
                RelationshipSummary {
                    kind,
                    source: source.clone(),
                    target,
                    implied,
                }
            })
            .collect::<Vec<_>>();
        for relationship in projection
            .relationships
            .iter()
            .filter(|relationship| relationship.source == node.id.qualified_name)
        {
            let Some(target_node) = projection
                .nodes
                .iter()
                .find(|candidate| candidate.semantic_id == relationship.target_id)
                .and_then(|projected| node_id_for_projected(&self.snapshot, projected))
                .and_then(|id| self.snapshot.semantic_graph().get_node(&id))
            else {
                continue;
            };
            let target = self.expose_node(target_node);
            let kind = ApiRelationshipKind::parse(relationship.kind.as_str());
            if seen.insert((kind.clone(), target.semantic_id.clone())) {
                values.push(RelationshipSummary {
                    kind,
                    source: source.clone(),
                    target,
                    implied: relationship.is_implied,
                });
            }
        }
        // Order by the kind's wire spelling, not by the enum's derived `Ord`. Deriving would
        // tie the documented result order to variant declaration order, so inserting a
        // variant would silently reorder every generator's output.
        values.sort_by(|a, b| {
            a.kind
                .as_str()
                .cmp(b.kind.as_str())
                .then_with(|| summary_order(&a.target, &b.target))
        });
        self.enforce_limit(values.len())?;
        Ok(values)
    }

    /// Returns direct features first, followed by inherited features from nearest to furthest
    /// specialized definitions. A same-named direct feature shadows an inherited feature.
    pub fn effective_features(&self, handle: &str) -> Result<Vec<ElementSummary>, ModelQueryError> {
        let id = self.resolve_handle(handle)?;
        let node = self
            .snapshot
            .semantic_graph()
            .get_node(&id)
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))?;
        let graph = self.snapshot.semantic_graph();
        let mut queue = vec![node];
        if !node.element_kind.is_definition() {
            let mut typed = graph.outgoing_targets_by_kind(node, RelationshipKind::Typing);
            typed.sort_by(node_order);
            queue.extend(typed);
        }
        let mut visited = HashSet::<NodeId>::new();
        let mut names = BTreeSet::<String>::new();
        let mut result = Vec::new();
        let mut cursor = 0;
        while let Some(owner) = queue.get(cursor).copied() {
            cursor += 1;
            if !visited.insert(owner.id.clone()) {
                continue;
            }
            let mut children = graph.children_of(owner);
            children.sort_by(node_order);
            for child in children.into_iter().filter(|child| is_feature(child)) {
                let identity = if child.name.is_empty() {
                    child.id.qualified_name.clone()
                } else {
                    child.name.clone()
                };
                if names.insert(identity) {
                    result.push(self.expose_node(child));
                    self.enforce_limit(result.len())?;
                }
            }
            let mut bases = graph.outgoing_targets_by_kind(owner, RelationshipKind::Specializes);
            bases.sort_by(node_order);
            queue.extend(bases);
        }
        Ok(result)
    }

    /// Whether `handle` names an element this view has exposed.
    ///
    /// Cheaper than `element()` for callers that only need validity, such as attaching a
    /// diagnostic to an element, which would otherwise build and discard a full detail.
    pub fn is_valid_handle(&self, handle: &str) -> bool {
        self.exposed
            .lock()
            .expect("generator handle index poisoned")
            .contains_key(handle)
    }

    fn resolve_handle(&self, handle: &str) -> Result<NodeId, ModelQueryError> {
        self.exposed
            .lock()
            .expect("generator handle index poisoned")
            .get(handle)
            .cloned()
            .ok_or_else(|| ModelQueryError::UnknownHandle(handle.to_owned()))
    }

    fn expose_node(&self, node: &SemanticNode) -> ElementSummary {
        let handle = handle_for(&self.snapshot, node);
        self.exposed
            .lock()
            .expect("generator handle index poisoned")
            .insert(handle.clone(), node.id.clone());
        let projected = self.projected(node);
        ElementSummary {
            handle,
            semantic_id: generator_semantic_id(&self.snapshot, node),
            metaclass: projected
                .and_then(|value| value.facts.element_type.as_deref())
                .map(Metaclass::parse)
                .unwrap_or_else(|| api_metaclass(node.element_kind.as_str())),
            name: (!node.name.is_empty()).then(|| node.name.clone()),
            qualified_name: node.id.qualified_name.clone(),
            library_element: workspace::semantic::uri_under_any_library(
                &node.id.uri,
                self.snapshot.library_urls(),
            ),
        }
    }

    fn summary_for_id(&self, id: &NodeId) -> Option<ElementSummary> {
        self.snapshot
            .semantic_graph()
            .get_node(id)
            .map(|node| self.expose_node(node))
    }

    fn projected(&self, node: &SemanticNode) -> Option<&HostSemanticModelNode> {
        self.projected_by_key
            .get(&(node.id.uri.to_string(), node.id.qualified_name.clone()))
    }

    fn sorted_bounded(
        &self,
        mut values: Vec<ElementSummary>,
    ) -> Result<Vec<ElementSummary>, ModelQueryError> {
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

fn node_id_for_projected(
    snapshot: &HostWorkspaceSnapshot,
    projected: &HostSemanticModelNode,
) -> Option<NodeId> {
    snapshot
        .semantic_graph()
        .node_ids_for_qualified_name(&projected.qualified_name)
        .into_iter()
        .flatten()
        .find(|id| id.uri.as_str() == projected.uri)
        .cloned()
}

fn handle_for(snapshot: &HostWorkspaceSnapshot, node: &SemanticNode) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-generator-handle-v1\0");
    hasher.update(generator_semantic_id(snapshot, node).as_bytes());
    format!("h:{:x}", hasher.finalize())
}

fn generator_semantic_id(snapshot: &HostWorkspaceSnapshot, node: &SemanticNode) -> String {
    let mut hasher = Sha256::new();
    hasher.update(b"spec42-generator-semantic-element-v1\0");
    hasher.update(normalized_origin(snapshot, &node.id).as_bytes());
    hasher.update([0]);
    hasher.update(node.element_kind.as_str().as_bytes());
    hasher.update([0]);
    hasher.update(node.id.qualified_name.as_bytes());
    format!("s42g:{:x}", hasher.finalize())
}

fn normalized_origin(snapshot: &HostWorkspaceSnapshot, id: &NodeId) -> String {
    if let Ok(path) = id.uri.to_file_path() {
        if let Ok(relative) = path.strip_prefix(snapshot.workspace_root()) {
            return format!("workspace:{}", portable_path(relative));
        }
        for library_root in snapshot.library_paths() {
            if let Ok(relative) = path.strip_prefix(library_root) {
                let library_name = library_root
                    .file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or("library");
                return format!("library:{library_name}:{}", portable_path(relative));
            }
        }
    }
    let content_digest = snapshot
        .metadata()
        .document_hashes
        .get(id.uri.as_str())
        .cloned()
        .unwrap_or_else(|| "unknown".to_owned());
    format!("content:{content_digest}")
}

fn portable_path(path: &std::path::Path) -> String {
    path.components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect::<Vec<_>>()
        .join("/")
}

fn metaclass_matches(node: &HostSemanticModelNode, requested: &str) -> bool {
    node.element_kind.as_str().eq_ignore_ascii_case(requested)
        || api_metaclass(node.element_kind.as_str())
            .as_str()
            .eq_ignore_ascii_case(requested)
        || node
            .facts
            .element_type
            .as_deref()
            .is_some_and(|value| value.eq_ignore_ascii_case(requested))
}

fn api_metaclass(kind: &str) -> Metaclass {
    match kind {
        "package" => Metaclass::Package,
        "part def" => Metaclass::PartDefinition,
        "part" => Metaclass::PartUsage,
        "port def" => Metaclass::PortDefinition,
        "port" => Metaclass::PortUsage,
        "item def" => Metaclass::ItemDefinition,
        "item" => Metaclass::ItemUsage,
        "attribute def" => Metaclass::AttributeDefinition,
        "attribute" => Metaclass::AttributeUsage,
        "action def" => Metaclass::ActionDefinition,
        "action" => Metaclass::ActionUsage,
        "occurrence def" => Metaclass::OccurrenceDefinition,
        "occurrence" => Metaclass::OccurrenceUsage,
        "flow def" => Metaclass::FlowDefinition,
        "flow" => Metaclass::FlowUsage,
        "allocation def" => Metaclass::AllocationDefinition,
        "allocation" => Metaclass::AllocationUsage,
        "state def" => Metaclass::StateDefinition,
        "state" => Metaclass::StateUsage,
        "requirement def" => Metaclass::RequirementDefinition,
        "requirement" | "verified requirement" => Metaclass::RequirementUsage,
        "use case def" => Metaclass::UseCaseDefinition,
        "use case" => Metaclass::UseCaseUsage,
        "concern def" => Metaclass::ConcernDefinition,
        "concern" => Metaclass::ConcernUsage,
        "verification def" => Metaclass::VerificationCaseDefinition,
        "verification" => Metaclass::VerificationCaseUsage,
        "analysis def" => Metaclass::AnalysisCaseDefinition,
        "analysis" => Metaclass::AnalysisCaseUsage,
        "case def" => Metaclass::CaseDefinition,
        "case" => Metaclass::CaseUsage,
        "constraint def" => Metaclass::ConstraintDefinition,
        "constraint" => Metaclass::ConstraintUsage,
        "calc def" => Metaclass::CalculationDefinition,
        "calc" => Metaclass::CalculationUsage,
        "enum def" => Metaclass::EnumerationDefinition,
        "enumerated value" => Metaclass::EnumerationUsage,
        "connection def" => Metaclass::ConnectionDefinition,
        "connection" => Metaclass::ConnectionUsage,
        "interface def" => Metaclass::InterfaceDefinition,
        "interface" => Metaclass::InterfaceUsage,
        "individual def" => Metaclass::IndividualDefinition,
        "individual" => Metaclass::IndividualUsage,
        "view def" => Metaclass::ViewDefinition,
        "view" => Metaclass::ViewUsage,
        "viewpoint def" => Metaclass::ViewpointDefinition,
        "viewpoint" => Metaclass::ViewpointUsage,
        "rendering def" => Metaclass::RenderingDefinition,
        "rendering" => Metaclass::RenderingUsage,
        "metadata def" => Metaclass::MetadataDefinition,
        "metadata usage" | "metadata keyword" => Metaclass::MetadataUsage,
        "ref" => Metaclass::ReferenceUsage,
        "documentation" => Metaclass::Documentation,
        other => Metaclass::Unrecognized(other.to_owned()),
    }
}

fn summary_order(left: &ElementSummary, right: &ElementSummary) -> std::cmp::Ordering {
    left.qualified_name
        .cmp(&right.qualified_name)
        .then_with(|| left.metaclass.as_str().cmp(right.metaclass.as_str()))
        .then_with(|| left.semantic_id.cmp(&right.semantic_id))
}

fn node_order(left: &&SemanticNode, right: &&SemanticNode) -> std::cmp::Ordering {
    left.id
        .qualified_name
        .cmp(&right.id.qualified_name)
        .then_with(|| left.element_kind.as_str().cmp(right.element_kind.as_str()))
        .then_with(|| left.id.uri.as_str().cmp(right.id.uri.as_str()))
}

fn is_feature(node: &SemanticNode) -> bool {
    !node.element_kind.is_definition()
        && !matches!(
            node.element_kind.as_str(),
            "package" | "import" | "documentation" | "textualRep" | "diagnostic" | "kermlDecl"
        )
}

fn expression_string(value: &impl Serialize) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "null".to_owned())
}

fn value_string(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(value) => value.clone(),
        other => other.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use workspace::{
        EngineBuilder, HostContext, HostFilesystemProvider, ValidationTiming, WorkspaceLoadRequest,
    };

    fn test_view() -> GeneratorModelView {
        let temp = tempfile::tempdir().unwrap();
        let model = temp.path().join("model.sysml");
        fs::write(
            &model,
            r#"
package P {
    part def Base {
        attribute mass;
    }
    part def Vehicle :> Base {
        attribute speed;
    }
    part vehicle : Vehicle;
}
"#,
        )
        .unwrap();
        let engine = EngineBuilder::default()
            .cache_dir(temp.path().join("cache"))
            .no_stdlib(true)
            .build()
            .unwrap();
        let provider =
            HostFilesystemProvider::from_paths(&model, Some(temp.path()), engine.package_roots());
        let request = WorkspaceLoadRequest::single_target(model)
            .with_workspace_root(Some(temp.path().to_path_buf()))
            .with_validation_timing(ValidationTiming::Deferred);
        let snapshot = engine
            .load_workspace(provider, request, HostContext::default())
            .unwrap();
        GeneratorModelView::new(snapshot, QueryLimits::default())
    }

    #[test]
    fn queries_normative_metaclasses_typing_and_effective_features() {
        let view = test_view();
        let definitions = view.find(Some("PartDefinition")).unwrap();
        assert_eq!(
            definitions
                .iter()
                .map(|element| element.qualified_name.as_str())
                .collect::<Vec<_>>(),
            ["P::Base", "P::Vehicle"]
        );
        let usage = view
            .find(Some("PartUsage"))
            .unwrap()
            .into_iter()
            .find(|element| element.qualified_name == "P::vehicle")
            .unwrap();
        assert_eq!(
            view.typed_by(&usage.handle)
                .unwrap()
                .unwrap()
                .qualified_name,
            "P::Vehicle"
        );
        let features = view.effective_features(&usage.handle).unwrap();
        assert_eq!(
            features
                .iter()
                .map(|element| element.qualified_name.as_str())
                .collect::<Vec<_>>(),
            ["P::Vehicle::speed", "P::Base::mass"]
        );
    }

    /// The escape hatch must stay unused in practice. If Spec42's upstream model gains an
    /// element kind or relationship kind the ABI has not mapped, this fails here rather than
    /// leaking a raw internal spelling to generators as though it were a metaclass.
    #[test]
    fn no_element_or_relationship_maps_to_the_unrecognized_escape_hatch() {
        let view = test_view();
        let elements = view.find(None).expect("find");
        assert!(!elements.is_empty(), "the fixture should expose elements");

        for element in &elements {
            assert!(
                !element.metaclass.is_unrecognized(),
                "`{}` has an unmapped metaclass `{}`",
                element.qualified_name,
                element.metaclass
            );
            for edge in view.relationships(&element.handle).expect("relationships") {
                assert!(
                    !edge.kind.is_unrecognized(),
                    "`{}` has an unmapped relationship kind `{}`",
                    element.qualified_name,
                    edge.kind
                );
            }
        }
    }

    #[test]
    fn model_digest_and_query_order_are_stable() {
        let view = test_view();
        let relocated = test_view();
        assert_eq!(view.model_digest(), relocated.model_digest());
        assert_eq!(view.find(None).unwrap(), relocated.find(None).unwrap());
        let found = view.find(None).unwrap();
        assert!(found
            .windows(2)
            .all(|pair| summary_order(&pair[0], &pair[1]).is_le()));
    }
}
