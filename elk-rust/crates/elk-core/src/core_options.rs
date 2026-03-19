use std::collections::BTreeMap;

#[derive(Clone, Debug, PartialEq)]
pub enum CorePropertyValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Null,
    Array(Vec<CorePropertyValue>),
    Object(BTreeMap<String, CorePropertyValue>),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoreOptionScope {
    Graph,
    Node,
    Port,
    Edge,
    Label,
    EdgeSection,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CoreValidationIssueKind {
    UnknownKey,
    WrongType,
    DisallowedScope { scope: CoreOptionScope },
    DeprecatedKey { replacement: String },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CoreValidationIssue {
    pub key: String,
    pub kind: CoreValidationIssueKind,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct CoreOptionPreflight {
    pub normalized: Vec<(String, CorePropertyValue)>,
    pub issues: Vec<CoreValidationIssue>,
}

pub trait CoreOptionPipeline {
    fn preflight(&self, scope: CoreOptionScope, input: &[(String, CorePropertyValue)]) -> CoreOptionPreflight;
}
