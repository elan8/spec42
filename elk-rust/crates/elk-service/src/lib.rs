#![forbid(unsafe_code)]
#![doc = "Algorithm registry and dispatcher (ELK core.service equivalent)."]

use std::collections::BTreeMap;

use elk_core::{
    CoreOptionPipeline, CoreOptionScope, CorePropertyValue, CoreValidationIssue, CoreValidationIssueKind,
    LayoutError, LayoutOptions, LayoutReport,
};
use elk_graph::{ElkGraph, PropertyBag, PropertyKey, PropertyValue};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AlgorithmId(pub String);

pub trait LayoutAlgorithm: Send + Sync {
    fn id(&self) -> AlgorithmId;
    fn layout(&self, graph: &mut ElkGraph, options: &LayoutOptions)
        -> Result<LayoutReport, LayoutError>;
}

#[derive(Default)]
pub struct AlgorithmRegistry {
    by_id: BTreeMap<String, Box<dyn LayoutAlgorithm>>,
}

impl AlgorithmRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, alg: Box<dyn LayoutAlgorithm>) {
        let id = alg.id().0.to_ascii_lowercase();
        self.by_id.insert(id, alg);
    }

    #[must_use]
    pub fn get(&self, id: &str) -> Option<&dyn LayoutAlgorithm> {
        self.by_id
            .get(&id.to_ascii_lowercase())
            .map(|b| b.as_ref())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServiceError {
    MissingAlgorithmId,
    UnknownAlgorithmId(String),
    Layout(LayoutError),
}

impl From<LayoutError> for ServiceError {
    fn from(value: LayoutError) -> Self {
        ServiceError::Layout(value)
    }
}

pub struct LayoutService {
    pub meta: elk_meta::OptionRegistry,
    pub registry: AlgorithmRegistry,
}

impl LayoutService {
    #[must_use]
    pub fn new(meta: elk_meta::OptionRegistry, registry: AlgorithmRegistry) -> Self {
        Self { meta, registry }
    }

    #[must_use]
    pub fn default_registry() -> Self {
        let meta = elk_meta::default_registry();
        let mut registry = AlgorithmRegistry::new();
        registry.register(Box::new(LayeredAlgorithm));
        registry.register(Box::new(TreeAlgorithm));
        registry.register(Box::new(RectPackingAlgorithm));
        registry.register(Box::new(TopDownPackingAlgorithm));
        registry.register(Box::new(LibavoidAlgorithm));
        Self { meta, registry }
    }

    pub fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, ServiceError> {
        let pipeline = MetaOptionPipeline {
            registry: &self.meta,
        };
        let entries = bag_to_core_entries(&graph.properties);
        let preflight = pipeline.preflight(CoreOptionScope::Graph, &entries);
        graph.properties = core_entries_to_bag(&preflight.normalized);

        let mut preflight_warnings = preflight
            .issues
            .iter()
            .map(issue_to_warning)
            .collect::<Vec<_>>();

        // Explicit selection required: check aliases too.
        let alg_raw = elk_meta::get_string(
            &self.meta,
            &graph.properties,
            &["elk.algorithm", "org.eclipse.elk.algorithm"],
        )
        .ok_or(ServiceError::MissingAlgorithmId)?
        .to_string();

        let alg = normalize_algorithm_id(&alg_raw);
        if alg != alg_raw.to_ascii_lowercase() {
            preflight_warnings.push(format!(
                "Deprecated algorithm id {} replaced by {}",
                alg_raw, alg
            ));
        }
        graph
            .properties
            .insert("elk.algorithm", PropertyValue::String(alg.clone()));

        let engine = self
            .registry
            .get(&alg)
            .ok_or_else(|| ServiceError::UnknownAlgorithmId(alg_raw.clone()))?;

        let mut report = engine.layout(graph, options)?;
        report.warnings.extend(preflight_warnings);
        Ok(report)
    }
}

struct MetaOptionPipeline<'a> {
    registry: &'a elk_meta::OptionRegistry,
}

impl CoreOptionPipeline for MetaOptionPipeline<'_> {
    fn preflight(&self, scope: CoreOptionScope, input: &[(String, CorePropertyValue)]) -> elk_core::CoreOptionPreflight {
        let bag = core_entries_to_bag(input);
        let normalized = self.registry.normalize_bag(&bag);
        let issues = self
            .registry
            .validate_bag(scope_into_meta(scope), &bag)
            .into_iter()
            .map(meta_issue_into_core)
            .collect::<Vec<_>>();
        elk_core::CoreOptionPreflight {
            normalized: bag_to_core_entries(&normalized),
            issues,
        }
    }
}

fn scope_into_meta(scope: CoreOptionScope) -> elk_meta::OptionScope {
    match scope {
        CoreOptionScope::Graph => elk_meta::OptionScope::Graph,
        CoreOptionScope::Node => elk_meta::OptionScope::Node,
        CoreOptionScope::Port => elk_meta::OptionScope::Port,
        CoreOptionScope::Edge => elk_meta::OptionScope::Edge,
        CoreOptionScope::Label => elk_meta::OptionScope::Label,
        CoreOptionScope::EdgeSection => elk_meta::OptionScope::EdgeSection,
    }
}

fn meta_issue_into_core(issue: elk_meta::ValidationIssue) -> CoreValidationIssue {
    let kind = match issue.kind {
        elk_meta::ValidationIssueKind::UnknownKey => CoreValidationIssueKind::UnknownKey,
        elk_meta::ValidationIssueKind::WrongType { .. } => CoreValidationIssueKind::WrongType,
        elk_meta::ValidationIssueKind::DisallowedScope { scope } => CoreValidationIssueKind::DisallowedScope {
            scope: match scope {
                elk_meta::OptionScope::Graph => CoreOptionScope::Graph,
                elk_meta::OptionScope::Node => CoreOptionScope::Node,
                elk_meta::OptionScope::Port => CoreOptionScope::Port,
                elk_meta::OptionScope::Edge => CoreOptionScope::Edge,
                elk_meta::OptionScope::Label => CoreOptionScope::Label,
                elk_meta::OptionScope::EdgeSection => CoreOptionScope::EdgeSection,
            },
        },
        elk_meta::ValidationIssueKind::DeprecatedKey { replacement } => {
            CoreValidationIssueKind::DeprecatedKey { replacement }
        }
    };
    CoreValidationIssue { key: issue.key, kind }
}

fn bag_to_core_entries(bag: &PropertyBag) -> Vec<(String, CorePropertyValue)> {
    bag.iter()
        .map(|(k, v)| (k.0.clone(), graph_value_to_core(v)))
        .collect::<Vec<_>>()
}

fn core_entries_to_bag(entries: &[(String, CorePropertyValue)]) -> PropertyBag {
    let mut bag = PropertyBag::default();
    for (k, v) in entries {
        bag.insert(PropertyKey(k.clone()), core_value_to_graph(v.clone()));
    }
    bag
}

fn graph_value_to_core(value: &PropertyValue) -> CorePropertyValue {
    match value {
        PropertyValue::Bool(v) => CorePropertyValue::Bool(*v),
        PropertyValue::Int(v) => CorePropertyValue::Int(*v),
        PropertyValue::Float(v) => CorePropertyValue::Float(*v),
        PropertyValue::String(v) => CorePropertyValue::String(v.clone()),
        PropertyValue::Null => CorePropertyValue::Null,
        PropertyValue::Array(v) => CorePropertyValue::Array(v.iter().map(graph_value_to_core).collect()),
        PropertyValue::Object(v) => CorePropertyValue::Object(
            v.iter()
                .map(|(k, v)| (k.clone(), graph_value_to_core(v)))
                .collect(),
        ),
    }
}

fn core_value_to_graph(value: CorePropertyValue) -> PropertyValue {
    match value {
        CorePropertyValue::Bool(v) => PropertyValue::Bool(v),
        CorePropertyValue::Int(v) => PropertyValue::Int(v),
        CorePropertyValue::Float(v) => PropertyValue::Float(v),
        CorePropertyValue::String(v) => PropertyValue::String(v),
        CorePropertyValue::Null => PropertyValue::Null,
        CorePropertyValue::Array(v) => PropertyValue::Array(v.into_iter().map(core_value_to_graph).collect()),
        CorePropertyValue::Object(v) => {
            PropertyValue::Object(v.into_iter().map(|(k, v)| (k, core_value_to_graph(v))).collect())
        }
    }
}

fn issue_to_warning(issue: &CoreValidationIssue) -> String {
    match &issue.kind {
        CoreValidationIssueKind::UnknownKey => format!("Unknown option key: {}", issue.key),
        CoreValidationIssueKind::WrongType => format!("Wrong option type: {}", issue.key),
        CoreValidationIssueKind::DisallowedScope { scope } => {
            format!("Option key disallowed at {scope:?} scope: {}", issue.key)
        }
        CoreValidationIssueKind::DeprecatedKey { replacement } => {
            format!("Deprecated option key {} replaced by {}", issue.key, replacement)
        }
    }
}

/// Map ELK algorithm id aliases (e.g. org.eclipse.elk.alg.*) to canonical ids used in the registry.
#[must_use]
fn normalize_algorithm_id(alg: &str) -> String {
    let lower = alg.to_ascii_lowercase();
    match lower.as_str() {
        "org.eclipse.elk.alg.rectpacking" => "org.eclipse.elk.rectpacking".to_string(),
        "org.eclipse.elk.alg.topdownpacking" => "org.eclipse.elk.topdownpacking".to_string(),
        "org.eclipse.elk.alg.libavoid" => "org.eclipse.elk.libavoid".to_string(),
        "org.eclipse.elk.alg.mrtree" => "org.eclipse.elk.mrtree".to_string(),
        "org.eclipse.elk.alg.layered" => "org.eclipse.elk.layered".to_string(),
        _ => lower,
    }
}

struct LayeredAlgorithm;

impl LayoutAlgorithm for LayeredAlgorithm {
    fn id(&self) -> AlgorithmId {
        AlgorithmId("org.eclipse.elk.layered".to_string())
    }

    fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        elk_layered::layout(graph, options)
    }
}

struct TreeAlgorithm;

impl LayoutAlgorithm for TreeAlgorithm {
    fn id(&self) -> AlgorithmId {
        AlgorithmId("org.eclipse.elk.mrtree".to_string())
    }

    fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        elk_tree::layout(graph, options)
    }
}

struct RectPackingAlgorithm;

impl LayoutAlgorithm for RectPackingAlgorithm {
    fn id(&self) -> AlgorithmId {
        AlgorithmId("org.eclipse.elk.rectpacking".to_string())
    }

    fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        elk_rectpacking::layout(graph, options)
    }
}

struct TopDownPackingAlgorithm;

impl LayoutAlgorithm for TopDownPackingAlgorithm {
    fn id(&self) -> AlgorithmId {
        AlgorithmId("org.eclipse.elk.topdownpacking".to_string())
    }

    fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        elk_topdownpacking::layout(graph, options)
    }
}

struct LibavoidAlgorithm;

impl LayoutAlgorithm for LibavoidAlgorithm {
    fn id(&self) -> AlgorithmId {
        AlgorithmId("org.eclipse.elk.libavoid".to_string())
    }

    fn layout(
        &self,
        graph: &mut ElkGraph,
        options: &LayoutOptions,
    ) -> Result<LayoutReport, LayoutError> {
        elk_libavoid::layout(graph, options)
    }
}

#[cfg(test)]
mod tests {
    use elk_graph_json::import_str;

    use super::*;

    #[test]
    fn missing_algorithm_errors() {
        let mut g = ElkGraph::new();
        let svc = LayoutService::default_registry();
        let err = svc.layout(&mut g, &LayoutOptions::default()).unwrap_err();
        assert!(matches!(err, ServiceError::MissingAlgorithmId));
    }

    #[test]
    fn unknown_algorithm_errors() {
        let mut g = ElkGraph::new();
        g.properties.insert("elk.algorithm", elk_graph::PropertyValue::String("nope".to_string()));
        let svc = LayoutService::default_registry();
        let err = svc.layout(&mut g, &LayoutOptions::default()).unwrap_err();
        assert!(matches!(err, ServiceError::UnknownAlgorithmId(_)));
    }

    #[test]
    fn layered_dispatch_succeeds_on_fixture() {
        let json = r#"
        {
          "id": "root",
          "layoutOptions": { "elk.algorithm": "org.eclipse.elk.layered", "elk.direction": "DOWN" },
          "children":[
            {"id":"a","width":80,"height":40},
            {"id":"b","width":80,"height":40}
          ],
          "edges":[{"id":"e1","sources":["a"],"targets":["b"]}]
        }
        "#;
        let mut g = import_str(json).expect("import").graph;
        let svc = LayoutService::default_registry();
        let _report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout");
        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());
    }

    #[test]
    fn tree_dispatch_succeeds_on_fixture() {
        let json = r#"
        {
          "id": "root",
          "layoutOptions": { "elk.algorithm": "org.eclipse.elk.mrtree", "elk.direction": "DOWN" },
          "children":[
            {"id":"a","width":80,"height":40},
            {"id":"b","width":80,"height":40},
            {"id":"c","width":80,"height":40}
          ],
          "edges":[{"id":"e1","sources":["a"],"targets":["b"]},{"id":"e2","sources":["a"],"targets":["c"]}]
        }
        "#;
        let mut g = import_str(json).expect("import").graph;
        let svc = LayoutService::default_registry();
        let _report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout");
        let root = g.nodes[g.root.index()].geometry;
        assert!(root.width.is_finite());
        assert!(root.height.is_finite());
    }

    #[test]
    fn algorithm_id_aliases_dispatch() {
        let svc = LayoutService::default_registry();
        for (alg_value, has_edges) in [
            ("org.eclipse.elk.alg.rectpacking", false),
            ("org.eclipse.elk.alg.topdownpacking", false),
            ("org.eclipse.elk.alg.libavoid", true),
        ] {
            let json = format!(
                r#"
                {{ "id": "root", "layoutOptions": {{ "elk.algorithm": "{}" }},
                  "children": [ {{ "id": "a", "width": 60, "height": 40 }}, {{ "id": "b", "x": 150, "y": 0, "width": 60, "height": 40 }} ],
                  "edges": [ {{ "id": "e1", "sources": ["a"], "targets": ["b"] }} ]
                }}
                "#,
                alg_value
            );
            let mut g = import_str(&json).expect("import").graph;
            let report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout should succeed");
            if has_edges {
                assert!(
                    g.edges.iter().any(|e| !e.sections.is_empty()),
                    "libavoid should produce sections"
                );
            }
            let root = g.nodes[g.root.index()].geometry;
            assert!(root.width.is_finite());
            assert!(root.height.is_finite());
            let _ = report;
        }
    }

    #[test]
    fn deprecated_alias_is_normalized_and_warned() {
        let mut g = ElkGraph::new();
        g.properties.insert(
            "elk.algorithm",
            elk_graph::PropertyValue::String("org.eclipse.elk.alg.rectpacking".to_string()),
        );
        let svc = LayoutService::default_registry();
        let report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout");
        let canonical = elk_meta::get_string(
            &svc.meta,
            &g.properties,
            &["elk.algorithm", "org.eclipse.elk.algorithm"],
        );
        assert_eq!(canonical, Some("org.eclipse.elk.rectpacking"));
        assert!(report.warnings.iter().any(|w| w.contains("Deprecated algorithm id")));
    }
}

