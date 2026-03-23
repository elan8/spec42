#![forbid(unsafe_code)]
#![doc = "Option metadata registry and validation (ELK core.meta equivalent)."]

use std::collections::{BTreeMap, BTreeSet};

use elk_graph::{PropertyBag, PropertyKey, PropertyValue};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OptionId(pub String);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OptionType {
    Bool,
    Int,
    Float,
    String,
    Null,
    Array,
    Object,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OptionScope {
    Graph,
    Node,
    Port,
    Edge,
    Label,
    EdgeSection,
}

#[derive(Clone, Debug)]
pub struct OptionMeta {
    pub id: OptionId,
    pub option_type: OptionType,
    pub default_value: Option<PropertyValue>,
    pub allowed_scopes: BTreeSet<OptionScope>,
    pub aliases: Vec<String>,
    pub deprecated_aliases: Vec<String>,
    pub doc: &'static str,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationSeverity {
    Warning,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationIssueKind {
    UnknownKey,
    WrongType { expected: OptionType, actual: OptionType },
    DisallowedScope { scope: OptionScope },
    DeprecatedKey { replacement: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationIssue {
    pub severity: ValidationSeverity,
    pub key: String,
    pub kind: ValidationIssueKind,
}

#[derive(Clone, Debug, Default)]
pub struct OptionRegistry {
    /// canonical_id -> meta
    by_id: BTreeMap<String, OptionMeta>,
    /// alias -> canonical_id
    alias_to_id: BTreeMap<String, String>,
    /// keys that should be replaced by canonical ids
    deprecated_keys: BTreeSet<String>,
}

impl OptionRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Canonicalize option keys (trim + lowercase).
    #[must_use]
    pub fn canonicalize_key(key: &str) -> String {
        key.trim().to_ascii_lowercase()
    }

    pub fn register(&mut self, mut meta: OptionMeta) {
        let canonical = Self::canonicalize_key(&meta.id.0);
        meta.id.0 = canonical.clone();

        // Record aliases.
        for alias in meta.aliases.iter_mut() {
            let a = Self::canonicalize_key(alias);
            *alias = a.clone();
            self.alias_to_id.insert(a, canonical.clone());
        }
        for alias in meta.deprecated_aliases.iter_mut() {
            let a = Self::canonicalize_key(alias);
            *alias = a.clone();
            self.alias_to_id.insert(a.clone(), canonical.clone());
            self.deprecated_keys.insert(a);
        }

        self.by_id.insert(canonical, meta);
    }

    #[must_use]
    pub fn canonical_key(&self, key: &str) -> Option<String> {
        let k = Self::canonicalize_key(key);
        if self.by_id.contains_key(&k) {
            return Some(k);
        }
        self.alias_to_id.get(&k).cloned()
    }

    #[must_use]
    pub fn lookup(&self, key: &str) -> Option<&OptionMeta> {
        let k = Self::canonicalize_key(key);
        if let Some(m) = self.by_id.get(&k) {
            return Some(m);
        }
        let id = self.alias_to_id.get(&k)?;
        self.by_id.get(id)
    }

    /// Return the first matching property value for a set of keys/aliases.
    #[must_use]
    pub fn get_canonical<'a>(&self, bag: &'a PropertyBag, keys: &[&str]) -> Option<&'a PropertyValue> {
        for k in keys {
            let k = Self::canonicalize_key(k);
            // First, direct key
            if let Some(v) = bag.get(&PropertyKey(k.clone())) {
                return Some(v);
            }
            // Then alias mapping
            if let Some(id) = self.alias_to_id.get(&k) {
                if let Some(v) = bag.get(&PropertyKey(id.clone())) {
                    return Some(v);
                }
            }
        }
        None
    }

    #[must_use]
    pub fn validate_bag(&self, scope: OptionScope, bag: &PropertyBag) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();
        for (key, value) in bag.iter() {
            let key_str = key.0.clone();
            let key_canonical = Self::canonicalize_key(&key_str);
            let Some(meta) = self.lookup(&key_str) else {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    key: key_str,
                    kind: ValidationIssueKind::UnknownKey,
                });
                continue;
            };

            if self.deprecated_keys.contains(&key_canonical) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    key: key_str.clone(),
                    kind: ValidationIssueKind::DeprecatedKey {
                        replacement: meta.id.0.clone(),
                    },
                });
            }

            if !meta.allowed_scopes.is_empty() && !meta.allowed_scopes.contains(&scope) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    key: key_str.clone(),
                    kind: ValidationIssueKind::DisallowedScope { scope },
                });
            }

            let actual = option_type_of(value);
            if actual != meta.option_type {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Warning,
                    key: key_str,
                    kind: ValidationIssueKind::WrongType {
                        expected: meta.option_type,
                        actual,
                    },
                });
            }
        }
        issues
    }

    /// Produce a normalized property bag where known aliases/deprecated keys are
    /// replaced with canonical option ids.
    ///
    /// If both alias and canonical key exist, canonical value wins.
    #[must_use]
    pub fn normalize_bag(&self, bag: &PropertyBag) -> PropertyBag {
        let mut out = PropertyBag::default();
        for (key, value) in bag.iter() {
            let canonical = self
                .canonical_key(&key.0)
                .unwrap_or_else(|| key.0.clone());
            let canonical_key = PropertyKey(canonical.clone());
            if out.get(&canonical_key).is_none() {
                out.insert(PropertyKey(canonical), value.clone());
            }
        }
        out
    }
}

/// Convenience: read a string option by any of its keys/aliases.
#[must_use]
pub fn get_string<'a>(reg: &OptionRegistry, bag: &'a PropertyBag, keys: &[&str]) -> Option<&'a str> {
    reg.get_canonical(bag, keys).and_then(PropertyValue::as_str)
}

#[must_use]
pub fn option_type_of(v: &PropertyValue) -> OptionType {
    match v {
        PropertyValue::Bool(_) => OptionType::Bool,
        PropertyValue::Int(_) => OptionType::Int,
        PropertyValue::Float(_) => OptionType::Float,
        PropertyValue::String(_) => OptionType::String,
        PropertyValue::Null => OptionType::Null,
        PropertyValue::Array(_) => OptionType::Array,
        PropertyValue::Object(_) => OptionType::Object,
    }
}

/// A baseline registry containing core + layered option IDs and aliases used by current code.
#[must_use]
pub fn default_registry() -> OptionRegistry {
    let mut reg = OptionRegistry::new();
    let scopes_graph: BTreeSet<OptionScope> = [OptionScope::Graph].into_iter().collect();

    reg.register(OptionMeta {
        id: OptionId("elk.algorithm".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: scopes_graph.clone(),
        aliases: vec!["org.eclipse.elk.algorithm".to_string()],
        deprecated_aliases: Vec::new(),
        doc: "Algorithm id used for dispatch.",
    });

    // Algorithm ids (aliases) for non-layered algorithms we’re porting.
    // Note: the service layer matches ids case-insensitively; these are for validation/compatibility.
    reg.register(OptionMeta {
        id: OptionId("org.eclipse.elk.rectpacking".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: scopes_graph.clone(),
        aliases: vec![
            "elk.rectpacking".to_string(),
        ],
        deprecated_aliases: vec![
            "org.eclipse.elk.alg.rectpacking".to_string(),
        ],
        doc: "Rectpacking algorithm id (compat).",
    });
    reg.register(OptionMeta {
        id: OptionId("org.eclipse.elk.topdownpacking".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: scopes_graph.clone(),
        aliases: vec![
            "elk.topdownpacking".to_string(),
        ],
        deprecated_aliases: vec![
            "org.eclipse.elk.alg.topdownpacking".to_string(),
        ],
        doc: "Topdownpacking algorithm id (compat).",
    });
    reg.register(OptionMeta {
        id: OptionId("org.eclipse.elk.libavoid".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: scopes_graph.clone(),
        aliases: vec![
            "elk.libavoid".to_string(),
        ],
        deprecated_aliases: vec![
            "org.eclipse.elk.alg.libavoid".to_string(),
        ],
        doc: "Libavoid algorithm id (compat).",
    });

    // Libavoid routing options (graph/edge scope).
    let scopes_graph_edge = [OptionScope::Graph, OptionScope::Edge]
        .into_iter()
        .collect::<BTreeSet<_>>();
    for (id, aliases, doc) in [
        (
            "elk.libavoid.clearance",
            vec!["org.eclipse.elk.libavoid.clearance"],
            "Clearance between route and obstacles.",
        ),
        (
            "elk.libavoid.bendpenalty",
            vec!["org.eclipse.elk.libavoid.bendPenalty", "org.eclipse.elk.libavoid.bendpenalty"],
            "Penalty per bend in A* cost.",
        ),
        (
            "elk.libavoid.segmentpenalty",
            vec!["org.eclipse.elk.libavoid.segmentPenalty", "org.eclipse.elk.libavoid.segmentpenalty"],
            "Penalty per segment in A* cost.",
        ),
        (
            "elk.libavoid.routingstyle",
            vec!["org.eclipse.elk.libavoid.routingStyle", "org.eclipse.elk.libavoid.routingstyle"],
            "Polyline or orthogonal.",
        ),
    ] {
        reg.register(OptionMeta {
            id: OptionId(id.to_string()),
            option_type: if id.contains("clearance") || id.contains("penalty") {
                OptionType::Float
            } else {
                OptionType::String
            },
            default_value: None,
            allowed_scopes: scopes_graph_edge.clone(),
            aliases: aliases.into_iter().map(String::from).collect(),
            deprecated_aliases: Vec::new(),
            doc,
        });
    }
    reg.register(OptionMeta {
        id: OptionId("elk.direction".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: [OptionScope::Graph, OptionScope::Node].into_iter().collect(),
        aliases: vec!["org.eclipse.elk.direction".to_string()],
        deprecated_aliases: Vec::new(),
        doc: "Overall layout direction.",
    });
    reg.register(OptionMeta {
        id: OptionId("elk.edgerouting".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: [OptionScope::Graph, OptionScope::Edge].into_iter().collect(),
        aliases: vec![
            "elk.edgeRouting".to_string(),
            "org.eclipse.elk.edgeRouting".to_string(),
            "org.eclipse.elk.edgerouting".to_string(),
        ],
        deprecated_aliases: Vec::new(),
        doc: "Edge routing style.",
    });
    reg.register(OptionMeta {
        id: OptionId("elk.layered.routingbackend".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: [OptionScope::Graph].into_iter().collect(),
        aliases: vec![
            "elk.routing".to_string(),
            "org.eclipse.elk.layered.routingBackend".to_string(),
        ],
        deprecated_aliases: Vec::new(),
        doc: "Layered routing backend: default (simple) or libavoid.",
    });
    reg.register(OptionMeta {
        id: OptionId("elk.layered.unnecessarybendpoints".to_string()),
        option_type: OptionType::Bool,
        default_value: None,
        allowed_scopes: [OptionScope::Graph].into_iter().collect(),
        aliases: vec!["org.eclipse.elk.layered.unnecessaryBendpoints".to_string()],
        deprecated_aliases: Vec::new(),
        doc: "Keep unnecessary bend points after long-edge join-like simplification.",
    });
    reg.register(OptionMeta {
        id: OptionId("elk.portconstraints".to_string()),
        option_type: OptionType::String,
        default_value: None,
        allowed_scopes: [OptionScope::Graph, OptionScope::Node].into_iter().collect(),
        aliases: vec![
            "elk.portConstraints".to_string(),
            "org.eclipse.elk.portConstraints".to_string(),
            "org.eclipse.elk.portconstraints".to_string(),
        ],
        deprecated_aliases: Vec::new(),
        doc: "Port constraints.",
    });

    // Keep spacing keys broad and scalar-only for now.
    for (id, aliases, doc) in [
        (
            "elk.spacing.nodenodebetweenlayers",
            vec![
                "elk.spacing.nodeNodeBetweenLayers",
                "org.eclipse.elk.spacing.nodeNodeBetweenLayers",
                "org.eclipse.elk.spacing.nodenodebetweenlayers",
                "org.eclipse.elk.alg.layered.spacing.nodenodebetweenlayers",
            ],
            "Spacing between layers.",
        ),
        (
            "elk.spacing.nodenode",
            vec!["elk.spacing.nodeNode", "org.eclipse.elk.spacing.nodeNode", "org.eclipse.elk.spacing.nodenode"],
            "Node-node spacing.",
        ),
        (
            "elk.padding",
            vec!["org.eclipse.elk.padding"],
            "Node/graph padding.",
        ),
        (
            "elk.nodelabels.placement",
            vec!["org.eclipse.elk.nodeLabels.placement"],
            "Node label placement.",
        ),
        (
            "elk.portlabels.placement",
            vec!["org.eclipse.elk.portLabels.placement"],
            "Port label placement.",
        ),
        (
            "elk.edgelabels.placement",
            vec!["org.eclipse.elk.edgeLabels.placement"],
            "Edge label placement.",
        ),
    ] {
        reg.register(OptionMeta {
            id: OptionId(id.to_string()),
            option_type: OptionType::String,
            default_value: None,
            allowed_scopes: BTreeSet::new(),
            aliases: aliases.into_iter().map(|s| s.to_string()).collect(),
            deprecated_aliases: Vec::new(),
            doc,
        });
    }

    reg
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aliases_resolve() {
        let reg = default_registry();
        let m = reg.lookup("org.eclipse.elk.direction").expect("alias should resolve");
        assert_eq!(m.id.0, "elk.direction");
    }

    #[test]
    fn validate_bag_flags_unknowns() {
        let reg = default_registry();
        let mut bag = PropertyBag::default();
        bag.insert("elk.unknownOption", PropertyValue::Bool(true));
        let issues = reg.validate_bag(OptionScope::Graph, &bag);
        assert!(issues.iter().any(|i| matches!(i.kind, ValidationIssueKind::UnknownKey)));
    }

    #[test]
    fn validate_bag_flags_wrong_type_and_scope() {
        let reg = default_registry();
        let mut bag = PropertyBag::default();
        bag.insert("elk.direction", PropertyValue::Int(42));
        let issues = reg.validate_bag(OptionScope::Port, &bag);
        assert!(issues.iter().any(|i| {
            matches!(
                i.kind,
                ValidationIssueKind::WrongType {
                    expected: OptionType::String,
                    actual: OptionType::Int
                }
            )
        }));
        assert!(issues.iter().any(|i| matches!(i.kind, ValidationIssueKind::DisallowedScope { .. })));
    }

    #[test]
    fn normalize_bag_rewrites_deprecated_aliases() {
        let reg = default_registry();
        let mut bag = PropertyBag::default();
        bag.insert(
            "org.eclipse.elk.alg.rectpacking",
            PropertyValue::String("org.eclipse.elk.alg.rectpacking".to_string()),
        );
        let normalized = reg.normalize_bag(&bag);
        assert!(normalized.get(&PropertyKey("org.eclipse.elk.alg.rectpacking".to_string())).is_none());
        let value = normalized
            .get(&PropertyKey("org.eclipse.elk.rectpacking".to_string()))
            .and_then(PropertyValue::as_str);
        assert_eq!(value, Some("org.eclipse.elk.alg.rectpacking"));
    }

    #[test]
    fn validate_bag_flags_deprecated_key() {
        let reg = default_registry();
        let mut bag = PropertyBag::default();
        bag.insert(
            "org.eclipse.elk.alg.rectpacking",
            PropertyValue::String("org.eclipse.elk.alg.rectpacking".to_string()),
        );
        let issues = reg.validate_bag(OptionScope::Graph, &bag);
        assert!(issues.iter().any(|i| {
            matches!(
                i.kind,
                ValidationIssueKind::DeprecatedKey {
                    ref replacement
                } if replacement == "org.eclipse.elk.rectpacking"
            )
        }));
    }
}
