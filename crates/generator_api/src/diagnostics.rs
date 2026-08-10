use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GeneratorDiagnosticLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorDiagnostic {
    pub level: GeneratorDiagnosticLevel,
    pub message: String,
    pub element_id: Option<String>,
}
