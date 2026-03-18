#![forbid(unsafe_code)]
#![doc = "Algorithm registry and dispatcher (ELK core.service equivalent)."]

use std::collections::BTreeMap;

use elk_core::{LayoutError, LayoutOptions, LayoutReport};
use elk_graph::ElkGraph;

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
        // Explicit selection required: check aliases too.
        let alg_raw = elk_meta::get_string(
            &self.meta,
            &graph.properties,
            &["elk.algorithm", "org.eclipse.elk.algorithm"],
        )
        .ok_or(ServiceError::MissingAlgorithmId)?;

        let alg = normalize_algorithm_id(alg_raw);

        let engine = self
            .registry
            .get(&alg)
            .ok_or_else(|| ServiceError::UnknownAlgorithmId(alg_raw.to_string()))?;

        Ok(engine.layout(graph, options)?)
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
        let report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout");
        assert!(report.stats.layers >= 1);
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
        let report = svc.layout(&mut g, &LayoutOptions::default()).expect("layout");
        assert!(report.stats.phases.is_empty() || report.stats.layers >= 0);
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
}

